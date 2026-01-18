use anyhow::{Context, Result};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use super::schema_gen::{generate_create_table, generate_indexes};
use crate::parser::{parse_junction_records, parse_record, ParsedRow};
use crate::schema::{ColumnType, TableSchema, LANGUAGES};
use crate::ui::Ui;

const BATCH_SIZE: usize = 1000;

pub struct SqliteWriter {
    conn: Connection,
}

impl SqliteWriter {
    pub fn new(db_path: &Path) -> Result<Self> {
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

        Ok(Self { conn })
    }

    /// Create all tables for the given schemas
    pub fn create_tables(&self, schemas: &[&TableSchema], ui: &mut impl Ui) -> Result<()> {
        ui.log(format!("Creating {} tables...", schemas.len()));

        for (i, schema) in schemas.iter().enumerate() {
            let sql = generate_create_table(schema);
            self.conn
                .execute(&sql, [])
                .with_context(|| format!("Failed to create table: {}", schema.name))?;

            for index_sql in generate_indexes(schema) {
                self.conn
                    .execute(&index_sql, [])
                    .with_context(|| format!("Failed to create index for: {}", schema.name))?;
            }

            ui.set_progress((i + 1) as u64, schemas.len() as u64, "Creating tables");
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
            ui.log(format!("{}: skipped (file not found)", schema.name));
            return Ok(0);
        }

        let file =
            File::open(&file_path).with_context(|| format!("Failed to open: {:?}", file_path))?;
        let reader = BufReader::new(file);

        // Build insert statement
        let columns = get_column_names(schema);
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
                let rows = parse_junction_records(&line, schema).with_context(|| {
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
                let row = parse_record(&line, schema)
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
        ui.log(format!("{}: {} records", schema.name, count));

        Ok(count)
    }

    /// Finalize the database (enable FKs, optimize, etc.)
    pub fn finalize(self, ui: &mut impl Ui) -> Result<()> {
        ui.log("Finalizing database...");

        // Enable foreign keys for future use
        self.conn.execute("PRAGMA foreign_keys = ON;", [])?;
        self.conn.execute("PRAGMA optimize;", [])?;

        Ok(())
    }
}

/// Get column names for a schema, expanding localized columns
fn get_column_names(schema: &TableSchema) -> Vec<String> {
    let mut columns = Vec::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                for lang in LANGUAGES {
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
    ui: &mut impl Ui,
) -> Result<u64> {
    let mut writer = SqliteWriter::new(output_db)?;

    // Create all tables first
    writer.create_tables(&tables, ui)?;

    let mut total_records: u64 = 0;

    for (i, schema) in tables.iter().enumerate() {
        ui.log(format!(
            "Importing table {}/{}: {}",
            i + 1,
            tables.len(),
            schema.name
        ));

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
