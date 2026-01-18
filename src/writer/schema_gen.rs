use crate::schema::{ColumnType, TableSchema, LANGUAGES};

/// Generate CREATE TABLE SQL for a table schema
pub fn generate_create_table(schema: &TableSchema) -> String {
    let mut sql = format!("CREATE TABLE {} (\n", schema.name);
    let mut columns = Vec::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                // Expand localized columns to per-language columns
                for lang in LANGUAGES {
                    let col_name = format!("{}_{}", col.name, lang);
                    columns.push(format!("    {} TEXT", col_name));
                }
            }
            _ => {
                let sql_type = match col.col_type {
                    ColumnType::Integer => "INTEGER",
                    ColumnType::Real => "REAL",
                    ColumnType::Text => "TEXT",
                    ColumnType::Boolean => "INTEGER",
                    ColumnType::Json => "TEXT",
                    ColumnType::Localized => unreachable!(),
                };

                let null_constraint = if !col.nullable { " NOT NULL" } else { "" };
                let pk = if col.name == "id" { " PRIMARY KEY" } else { "" };

                columns.push(format!(
                    "    {} {}{}{}",
                    col.name, sql_type, pk, null_constraint
                ));
            }
        }
    }

    // Add foreign key constraints
    for fk in schema.foreign_keys {
        columns.push(format!(
            "    FOREIGN KEY ({}) REFERENCES {}({})",
            fk.column, fk.references_table, fk.references_column
        ));
    }

    sql.push_str(&columns.join(",\n"));
    sql.push_str("\n)");

    sql
}

/// Generate CREATE INDEX statements for a table
///
/// Automatically creates indexes for:
/// 1. Foreign key columns (for JOIN performance)
/// 2. name_en columns (for lookups by name)
/// 3. Composite unique indexes on junction tables
/// 4. Boolean filter columns (published, is_*, visible_*, deleted)
/// 5. Activity columns on blueprint tables
/// 6. Security status for map queries
pub fn generate_indexes(schema: &TableSchema) -> Vec<String> {
    let mut indexes = Vec::new();

    // 1. Index all foreign key columns
    for fk in schema.foreign_keys {
        indexes.push(format!(
            "CREATE INDEX idx_{}_{} ON {}({})",
            schema.name, fk.column, schema.name, fk.column
        ));
    }

    // 2. Index name_en for tables with localized names
    let has_name = schema
        .columns
        .iter()
        .any(|c| c.name == "name" && c.col_type == ColumnType::Localized);
    if has_name {
        indexes.push(format!(
            "CREATE INDEX idx_{}_name_en ON {}(name_en)",
            schema.name, schema.name
        ));
    }

    // 3. Composite unique index for junction tables with 2 FKs
    if schema.array_source.is_some() && schema.foreign_keys.len() == 2 {
        let cols: Vec<_> = schema.foreign_keys.iter().map(|fk| fk.column).collect();
        indexes.push(format!(
            "CREATE UNIQUE INDEX idx_{}_composite ON {}({}, {})",
            schema.name, schema.name, cols[0], cols[1]
        ));
    }

    // 4. Index boolean filter columns (very common in WHERE clauses)
    for col in schema.columns {
        let dominated_by_fk = schema.foreign_keys.iter().any(|fk| fk.column == col.name);
        if dominated_by_fk {
            continue;
        }

        let should_index = match col.name {
            // Common filter columns
            "published" | "deleted" => true,
            // Activity column for blueprint filtering
            "activity" => true,
            // Security status for map filtering
            "security_status" => true,
            // is_* and visible_* boolean flags
            name if name.starts_with("is_") || name.starts_with("visible_") => true,
            _ => false,
        };

        if should_index {
            indexes.push(format!(
                "CREATE INDEX idx_{}_{} ON {}({})",
                schema.name, col.name, schema.name, col.name
            ));
        }
    }

    indexes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::tables::TYPES;

    #[test]
    fn test_generate_create_table() {
        let sql = generate_create_table(&TYPES);
        assert!(sql.contains("CREATE TABLE types"));
        assert!(sql.contains("id INTEGER PRIMARY KEY"));
        assert!(sql.contains("name_en TEXT"));
        assert!(sql.contains("name_de TEXT"));
        assert!(sql.contains("FOREIGN KEY (group_id) REFERENCES groups(id)"));
    }

    #[test]
    fn test_generate_indexes() {
        let indexes = generate_indexes(&TYPES);
        assert!(indexes.iter().any(|i| i.contains("idx_types_group_id")));
    }
}
