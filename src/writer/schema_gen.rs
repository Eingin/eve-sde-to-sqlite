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

/// Generate CREATE INDEX statements for foreign key columns
pub fn generate_indexes(schema: &TableSchema) -> Vec<String> {
    schema
        .foreign_keys
        .iter()
        .map(|fk| {
            format!(
                "CREATE INDEX idx_{}_{} ON {}({})",
                schema.name, fk.column, schema.name, fk.column
            )
        })
        .collect()
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
