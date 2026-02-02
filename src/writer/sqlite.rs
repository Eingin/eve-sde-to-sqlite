use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::schema_gen::{generate_create_table, generate_indexes};
use crate::parser::{parse_junction_records, parse_record, ParsedRow};
use crate::schema::{ColumnType, LanguageFilter, TableSchema};
use crate::ui::Ui;

const BATCH_SIZE: usize = 1000;

pub struct SqliteWriter<'a> {
    conn: Connection,
    languages: &'a LanguageFilter,
}

impl<'a> SqliteWriter<'a> {
    pub fn new(db_path: &Path, languages: &'a LanguageFilter) -> Result<Self> {
        // Remove existing database if present
        if db_path.exists() {
            std::fs::remove_file(db_path).context("Failed to remove existing database")?;
        }

        let conn = Connection::open(db_path).context("Failed to create database")?;

        // Optimize for bulk insert - defer FK checks until finalize
        conn.execute_batch(
            "PRAGMA foreign_keys = OFF;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;",
        )?;

        Ok(Self { conn, languages })
    }

    /// Create all tables for the given schemas
    pub fn create_tables(&self, schemas: &[&TableSchema], ui: &mut impl Ui) -> Result<()> {
        for (i, schema) in schemas.iter().enumerate() {
            ui.set_progress(
                (i + 1) as u64,
                schemas.len() as u64,
                format!("Creating table: {}", schema.name),
            );

            let sql = generate_create_table(schema, self.languages);
            self.conn
                .execute(&sql, [])
                .with_context(|| format!("Failed to create table: {}", schema.name))?;

            for index_sql in generate_indexes(schema) {
                self.conn
                    .execute(&index_sql, [])
                    .with_context(|| format!("Failed to create index for: {}", schema.name))?;
            }
        }

        Ok(())
    }

    /// Import data from JSONL file for a single table
    pub fn import_table(
        &mut self,
        schema: &TableSchema,
        input_dir: &Path,
        line_count: u64,
        ui: &mut impl Ui,
    ) -> Result<u64> {
        let file_path = input_dir.join(schema.source_file);

        if !file_path.exists() {
            return Ok(0);
        }

        let file =
            File::open(&file_path).with_context(|| format!("Failed to open: {:?}", file_path))?;
        let reader = BufReader::new(file);

        // Build insert statement
        let columns = get_column_names(schema, self.languages);
        let placeholders: Vec<&str> = columns.iter().map(|_| "?").collect();
        let insert_sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            schema.name,
            columns.join(", "),
            placeholders.join(", ")
        );

        let tx = self.conn.transaction()?;
        let mut count: u64 = 0;
        let mut batch: Vec<ParsedRow> = Vec::with_capacity(BATCH_SIZE);

        let is_junction = schema.array_source.is_some();

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            if line.trim().is_empty() {
                continue;
            }

            if is_junction {
                // Junction table: one JSON line produces multiple rows
                let rows =
                    parse_junction_records(&line, schema, self.languages).with_context(|| {
                        format!("Failed to parse junction record in {}", schema.source_file)
                    })?;

                for row in rows {
                    batch.push(row);

                    if batch.len() >= BATCH_SIZE {
                        insert_batch(&tx, &insert_sql, &columns, &batch)?;
                        count += batch.len() as u64;
                        ui.set_progress(count, line_count, schema.name);
                        batch.clear();
                    }
                }
            } else {
                // Regular table: one JSON line = one row
                let row = parse_record(&line, schema, self.languages)
                    .with_context(|| format!("Failed to parse record in {}", schema.source_file))?;

                batch.push(row);

                if batch.len() >= BATCH_SIZE {
                    insert_batch(&tx, &insert_sql, &columns, &batch)?;
                    count += batch.len() as u64;
                    ui.set_progress(count, line_count, schema.name);
                    batch.clear();
                }
            }
        }

        // Insert remaining batch
        if !batch.is_empty() {
            insert_batch(&tx, &insert_sql, &columns, &batch)?;
            count += batch.len() as u64;
        }

        tx.commit()?;

        Ok(count)
    }

    /// Finalize the database (enable FKs, optimize, etc.)
    pub fn finalize(self, ui: &mut impl Ui) -> Result<()> {
        ui.set_info("Finalizing database...");

        // Create convenience views for common queries
        self.create_views()?;

        // Enable foreign keys for future use
        self.conn.execute("PRAGMA foreign_keys = ON;", [])?;
        self.conn.execute("PRAGMA optimize;", [])?;

        Ok(())
    }

    /// Create convenience views for common query patterns.
    ///
    /// Views are created to simplify common queries without sacrificing performance,
    /// as SQLite optimizes view queries using the underlying table indexes.
    fn create_views(&self) -> Result<()> {
        // View: type_bonuses
        //
        // Combines role bonuses (always-active ship bonuses) and trait bonuses
        // (per-skill-level bonuses) into a single unified view.
        //
        // This is particularly useful for fitting tools that need to display
        // all bonuses for a ship or module without writing complex UNION queries.
        //
        // Columns:
        //   - type_id: The ship/module type ID
        //   - bonus_type: 'role' for role bonuses, 'skill' for skill-based bonuses
        //   - skill_type_id: The skill type ID (NULL for role bonuses)
        //   - skill_name: Skill name in English (NULL for role bonuses)
        //   - bonus: Numeric bonus value (NULL for text-only bonuses like "Can fit X")
        //   - bonus_text_en: Description text (may contain HTML showinfo links)
        //   - importance: Display priority (lower = more important)
        //   - unit_id: Foreign key to dogma_units
        //   - unit_name: Unit name like 'Percentage', 'Modifier'
        //   - unit_display: Display symbol like '%', '+'
        //
        // Example usage:
        //   SELECT * FROM type_bonuses WHERE type_id = 22430 ORDER BY bonus_type, importance;
        //
        // Performance: Uses indexes on type_role_bonuses(type_id) and
        // type_trait_bonuses(type_id) for efficient lookups.
        self.conn.execute_batch(
            r#"
            CREATE VIEW IF NOT EXISTS type_bonuses AS
            SELECT 
                trb.type_id,
                'role' AS bonus_type,
                NULL AS skill_type_id,
                NULL AS skill_name,
                trb.bonus,
                trb.bonus_text_en,
                trb.importance,
                trb.unit_id,
                du.name AS unit_name,
                du.display_name_en AS unit_display
            FROM type_role_bonuses trb
            LEFT JOIN dogma_units du ON trb.unit_id = du.id

            UNION ALL

            SELECT 
                ttb.type_id,
                'skill' AS bonus_type,
                ttb.skill_type_id,
                t.name_en AS skill_name,
                ttb.bonus,
                ttb.bonus_text_en,
                ttb.importance,
                ttb.unit_id,
                du.name AS unit_name,
                du.display_name_en AS unit_display
            FROM type_trait_bonuses ttb
            LEFT JOIN types t ON ttb.skill_type_id = t.id
            LEFT JOIN dogma_units du ON ttb.unit_id = du.id;
            "#,
        )?;

        Ok(())
    }
}

/// Get column names for a schema, expanding localized columns
fn get_column_names(schema: &TableSchema, languages: &LanguageFilter) -> Vec<String> {
    let mut columns = Vec::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                for lang in languages.languages() {
                    columns.push(format!("{}_{}", col.name, lang));
                }
            }
            _ => {
                columns.push(col.name.to_string());
            }
        }
    }

    columns
}

/// Insert a batch of rows into the database
fn insert_batch(
    tx: &rusqlite::Transaction,
    sql: &str,
    columns: &[String],
    batch: &[ParsedRow],
) -> Result<()> {
    let mut stmt = tx.prepare_cached(sql)?;

    for row in batch {
        for (idx, col_name) in columns.iter().enumerate() {
            let value = row
                .values
                .get(col_name)
                .cloned()
                .unwrap_or(crate::parser::SqlValue::Null);
            value.bind_to(idx + 1, &mut stmt)?;
        }
        stmt.raw_execute()?;
    }

    Ok(())
}

/// Convert JSONL files to SQLite with UI progress
pub fn convert_to_sqlite(
    input_dir: &Path,
    output_db: &Path,
    tables: Vec<&TableSchema>,
    languages: &LanguageFilter,
    ui: &mut impl Ui,
) -> Result<u64> {
    let mut writer = SqliteWriter::new(output_db, languages)?;

    // Create all tables first
    writer.create_tables(&tables, ui)?;

    let mut total_records: u64 = 0;

    for (i, schema) in tables.iter().enumerate() {
        ui.set_progress(
            (i + 1) as u64,
            tables.len() as u64,
            format!("Importing: {}", schema.name),
        );

        // Count lines for progress estimation
        let file_path = input_dir.join(schema.source_file);
        let line_count = if file_path.exists() {
            BufReader::new(File::open(&file_path)?).lines().count() as u64
        } else {
            0
        };

        let count = writer.import_table(schema, input_dir, line_count, ui)?;
        total_records += count;
    }

    writer.finalize(ui)?;

    Ok(total_records)
}
