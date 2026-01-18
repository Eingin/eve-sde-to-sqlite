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
    /// Override JSON field name (default: camelCase of name)
    /// Use "_key" for array item IDs
    pub json_field: Option<&'static str>,
}

impl Column {
    /// Create an optional (nullable) column
    pub const fn new(name: &'static str, col_type: ColumnType) -> Self {
        Self {
            name,
            col_type,
            nullable: true,
            json_field: None,
        }
    }

    /// Create a required (non-nullable) column
    pub const fn required(name: &'static str, col_type: ColumnType) -> Self {
        Self {
            name,
            col_type,
            nullable: false,
            json_field: None,
        }
    }

    /// Set the JSON field name (for when it differs from camelCase of column name)
    pub const fn json(self, field: &'static str) -> Self {
        Self {
            json_field: Some(field),
            ..self
        }
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

/// Index definition
#[derive(Debug, Clone)]
pub struct Index {
    pub columns: &'static [&'static str],
    pub unique: bool,
}

impl Index {
    /// Create a non-unique index
    pub const fn on(columns: &'static [&'static str]) -> Self {
        Self {
            columns,
            unique: false,
        }
    }

    /// Create a unique index
    pub const fn unique(columns: &'static [&'static str]) -> Self {
        Self {
            columns,
            unique: true,
        }
    }
}

/// Describes how to extract rows from nested arrays in JSONL
#[derive(Debug, Clone)]
pub enum ArraySource {
    /// Simple nested array: `{"_key": X, "fieldName": [{...}, {...}]}`
    Simple {
        /// JSON field containing the array (e.g., "dogmaAttributes")
        array_field: &'static str,
        /// Column to store the parent's _key value (e.g., "type_id")
        parent_id_column: &'static str,
    },
    /// Simple array of integers: `{"_key": X, "fieldName": [123, 456, ...]}`
    SimpleIntArray {
        /// JSON field containing the array (e.g., "pins")
        array_field: &'static str,
        /// Column to store the parent's _key value (e.g., "schematic_id")
        parent_id_column: &'static str,
        /// Column to store each array value (e.g., "pin_type_id")
        value_column: &'static str,
    },
    /// Blueprint activities: `activities.{activity}.{field}[]`
    BlueprintActivity {
        /// Column to store the activity name (e.g., "activity")
        activity_column: &'static str,
        /// Array field within each activity (e.g., "materials")
        array_field: &'static str,
    },
    /// Nested key-value array: `{"_key": X, "types": [{"_key": Y, "_value": [{...}]}]}`
    NestedKeyValue {
        /// JSON field containing the outer array (e.g., "types")
        array_field: &'static str,
        /// Column to store the parent's _key value (e.g., "type_id")
        parent_id_column: &'static str,
        /// Column to store the nested object's _key value (e.g., "skill_type_id")
        nested_key_column: &'static str,
    },
    /// Double-nested arrays: `{"_key": X, "_value": [{"_key": Y, "_value": [Z, ...]}]}`
    DoubleNested {
        /// Column to store the parent's _key value (e.g., "type_id")
        parent_id_column: &'static str,
        /// Column to store the first-level _key (e.g., "mastery_level")
        level_key_column: &'static str,
    },
}

/// Table schema definition
#[derive(Debug, Clone)]
pub struct TableSchema {
    pub name: &'static str,
    pub source_file: &'static str,
    pub columns: &'static [Column],
    pub foreign_keys: &'static [ForeignKey],
    /// Explicit index definitions
    pub indexes: &'static [Index],
    /// Child tables derived from nested arrays
    pub child_tables: &'static [&'static str],
    /// For junction tables: how to extract rows from nested arrays
    pub array_source: Option<ArraySource>,
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
