use anyhow::{Context, Result};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rusqlite::Connection;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::parser::{parse_record, ParsedRow};
use crate::schema::{ColumnType, TableSchema, LANGUAGES};
use super::schema_gen::{generate_create_table, generate_indexes};

const BATCH_SIZE: usize = 1000;

pub struct SqliteWriter {
    conn: Connection,
}

impl SqliteWriter {
    pub fn new(db_path: &Path) -> Result<Self> {
        // Remove existing database if present
        if db_path.exists() {
            std::fs::remove_file(db_path)
                .context("Failed to remove existing database")?;
        }

        let conn = Connection::open(db_path)
            .context("Failed to create database")?;

        // Enable foreign keys and optimize for bulk insert
        conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA cache_size = -64000;"
        )?;

        Ok(Self { conn })
    }

    /// Create all tables for the given schemas
    pub fn create_tables(&self, schemas: &[&TableSchema]) -> Result<()> {
        println!("Creating {} tables...", schemas.len());
        
        for schema in schemas {
            let sql = generate_create_table(schema);
            self.conn.execute(&sql, [])
                .with_context(|| format!("Failed to create table: {}", schema.name))?;

            for index_sql in generate_indexes(schema) {
                self.conn.execute(&index_sql, [])
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
        progress: &ProgressBar,
    ) -> Result<u64> {
        let file_path = input_dir.join(schema.source_file);
        
        if !file_path.exists() {
            progress.finish_with_message(format!("{}: skipped (file not found)", schema.name));
            return Ok(0);
        }

        let file = File::open(&file_path)
            .with_context(|| format!("Failed to open: {:?}", file_path))?;
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

        for line in reader.lines() {
            let line = line.context("Failed to read line")?;
            if line.trim().is_empty() {
                continue;
            }

            let row = parse_record(&line, schema)
                .with_context(|| format!("Failed to parse record in {}", schema.source_file))?;
            
            batch.push(row);

            if batch.len() >= BATCH_SIZE {
                insert_batch(&tx, &insert_sql, &columns, &batch)?;
                count += batch.len() as u64;
                progress.set_position(count);
                batch.clear();
            }
        }

        // Insert remaining batch
        if !batch.is_empty() {
            insert_batch(&tx, &insert_sql, &columns, &batch)?;
            count += batch.len() as u64;
        }

        tx.commit()?;
        progress.set_position(count);
        progress.finish_with_message(format!("{}: {} records", schema.name, count));

        Ok(count)
    }

    /// Finalize the database (VACUUM, etc.)
    pub fn finalize(self) -> Result<()> {
        println!("Finalizing database...");
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
            let value = row.values.get(col_name)
                .cloned()
                .unwrap_or(crate::parser::SqlValue::Null);
            value.bind_to(idx + 1, &mut stmt)?;
        }
        stmt.raw_execute()?;
    }

    Ok(())
}

/// Convert JSONL files to SQLite with progress bars
pub fn convert_to_sqlite(
    input_dir: &Path,
    output_db: &Path,
    tables: Vec<&TableSchema>,
) -> Result<u64> {
    let mut writer = SqliteWriter::new(output_db)?;
    
    // Create all tables first
    writer.create_tables(&tables)?;

    // Set up progress bars
    let multi = MultiProgress::new();
    let style = ProgressStyle::default_bar()
        .template("{msg:30} [{bar:40.cyan/blue}] {pos}/{len}")
        .unwrap()
        .progress_chars("=>-");

    let mut total_records: u64 = 0;

    for schema in &tables {
        // Count lines for progress bar
        let file_path = input_dir.join(schema.source_file);
        let line_count = if file_path.exists() {
            BufReader::new(File::open(&file_path)?).lines().count() as u64
        } else {
            0
        };

        let pb = multi.add(ProgressBar::new(line_count));
        pb.set_style(style.clone());
        pb.set_message(schema.name.to_string());

        let count = writer.import_table(schema, input_dir, &pb)?;
        total_records += count;
    }

    writer.finalize()?;

    Ok(total_records)
}
