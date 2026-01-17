use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;

use crate::schema::{ColumnType, TableSchema, LANGUAGES};

/// A parsed row ready for insertion
pub struct ParsedRow {
    pub values: HashMap<String, SqlValue>,
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
}

impl SqlValue {
    pub fn bind_to(&self, idx: usize, stmt: &mut rusqlite::Statement) -> rusqlite::Result<()> {
        match self {
            SqlValue::Null => stmt.raw_bind_parameter(idx, rusqlite::types::Null)?,
            SqlValue::Integer(i) => stmt.raw_bind_parameter(idx, i)?,
            SqlValue::Real(f) => stmt.raw_bind_parameter(idx, f)?,
            SqlValue::Text(s) => stmt.raw_bind_parameter(idx, s.as_str())?,
        }
        Ok(())
    }
}

/// Parse a JSON line into a row for the given table schema
pub fn parse_record(line: &str, schema: &TableSchema) -> Result<ParsedRow> {
    let json: Value = serde_json::from_str(line)
        .context("Failed to parse JSON")?;

    let mut values = HashMap::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                // Handle localized fields
                let json_key = to_camel_case(col.name);
                if let Some(obj) = json.get(&json_key).and_then(|v| v.as_object()) {
                    for lang in LANGUAGES {
                        let col_name = format!("{}_{}", col.name, lang);
                        let value = obj.get(*lang)
                            .and_then(|v| v.as_str())
                            .map(|s| SqlValue::Text(s.to_string()))
                            .unwrap_or(SqlValue::Null);
                        values.insert(col_name, value);
                    }
                } else {
                    for lang in LANGUAGES {
                        let col_name = format!("{}_{}", col.name, lang);
                        values.insert(col_name, SqlValue::Null);
                    }
                }
            }
            _ => {
                let json_key = if col.name == "id" {
                    "_key".to_string()
                } else {
                    to_camel_case(col.name)
                };

                let value = extract_value(&json, &json_key, &col.col_type);
                values.insert(col.name.to_string(), value);
            }
        }
    }

    Ok(ParsedRow { values })
}

fn extract_value(json: &Value, key: &str, col_type: &ColumnType) -> SqlValue {
    let val = json.get(key);
    
    match val {
        None | Some(Value::Null) => SqlValue::Null,
        Some(v) => match col_type {
            ColumnType::Integer => {
                v.as_i64().map(SqlValue::Integer).unwrap_or(SqlValue::Null)
            }
            ColumnType::Real => {
                v.as_f64().map(SqlValue::Real).unwrap_or(SqlValue::Null)
            }
            ColumnType::Text => {
                v.as_str().map(|s| SqlValue::Text(s.to_string())).unwrap_or(SqlValue::Null)
            }
            ColumnType::Boolean => {
                v.as_bool().map(|b| SqlValue::Integer(if b { 1 } else { 0 })).unwrap_or(SqlValue::Null)
            }
            ColumnType::Json => {
                SqlValue::Text(v.to_string())
            }
            ColumnType::Localized => SqlValue::Null, // Handled separately
        }
    }
}

/// Convert snake_case to camelCase
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("group_id"), "groupId");
        assert_eq!(to_camel_case("solar_system_id"), "solarSystemId");
        assert_eq!(to_camel_case("name"), "name");
    }
}
