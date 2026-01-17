use std::collections::HashSet;

/// Supported languages for localized text
pub const LANGUAGES: &[&str] = &["en", "de", "es", "fr", "ja", "ko", "ru", "zh"];

/// Column data type
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnType {
    Integer,
    Real,
    Text,
    Boolean,
    /// Localized text expands to multiple columns (name_en, name_de, etc.)
    Localized,
    /// JSON blob stored as text
    Json,
}

/// Column definition
#[derive(Debug, Clone)]
pub struct Column {
    pub name: &'static str,
    pub col_type: ColumnType,
    pub nullable: bool,
}

impl Column {
    pub const fn new(name: &'static str, col_type: ColumnType) -> Self {
        Self { name, col_type, nullable: true }
    }

    pub const fn required(name: &'static str, col_type: ColumnType) -> Self {
        Self { name, col_type, nullable: false }
    }
}

/// Foreign key reference
#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub column: &'static str,
    pub references_table: &'static str,
    pub references_column: &'static str,
}

impl ForeignKey {
    pub const fn new(column: &'static str, references_table: &'static str) -> Self {
        Self {
            column,
            references_table,
            references_column: "id",
        }
    }
}

/// Table schema definition
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: &'static str,
    pub source_file: &'static str,
    pub columns: &'static [Column],
    pub foreign_keys: &'static [ForeignKey],
    /// Child tables derived from nested arrays
    pub child_tables: &'static [&'static str],
}

impl TableSchema {
    /// Get all tables this table depends on (FK parents)
    pub fn dependencies(&self) -> HashSet<&'static str> {
        self.foreign_keys
            .iter()
            .map(|fk| fk.references_table)
            .collect()
    }
}
