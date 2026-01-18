use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;

use crate::schema::{ArraySource, ColumnType, TableSchema, LANGUAGES};

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

/// Parse a JSON line into rows for a junction table (tables with array_source)
/// Returns multiple rows extracted from nested arrays
pub fn parse_junction_records(line: &str, schema: &TableSchema) -> Result<Vec<ParsedRow>> {
    let json: Value = serde_json::from_str(line).context("Failed to parse JSON")?;

    let array_source = schema
        .array_source
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Table {} has no array_source", schema.name))?;

    match array_source {
        ArraySource::Simple {
            array_field,
            parent_id_column,
        } => parse_simple_array(&json, schema, array_field, parent_id_column),
        ArraySource::SimpleIntArray {
            array_field,
            parent_id_column,
            value_column,
        } => parse_simple_int_array(&json, array_field, parent_id_column, value_column),
        ArraySource::BlueprintActivity {
            activity_column,
            array_field,
        } => parse_blueprint_activity(&json, schema, activity_column, array_field),
        ArraySource::NestedKeyValue {
            array_field,
            parent_id_column,
            nested_key_column,
        } => parse_nested_key_value(
            &json,
            schema,
            array_field,
            parent_id_column,
            nested_key_column,
        ),
        ArraySource::DoubleNested {
            parent_id_column,
            level_key_column,
        } => parse_double_nested(&json, schema, parent_id_column, level_key_column),
    }
}

/// Parse simple nested array: {"_key": X, "fieldName": [{...}, {...}]}
fn parse_simple_array(
    json: &Value,
    schema: &TableSchema,
    array_field: &str,
    parent_id_column: &str,
) -> Result<Vec<ParsedRow>> {
    let parent_id = json
        .get("_key")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing _key in JSON"))?;

    let array = match json.get(array_field) {
        Some(Value::Array(arr)) => arr,
        _ => return Ok(vec![]), // No array or empty
    };

    let mut rows = Vec::with_capacity(array.len());

    for item in array {
        let mut values = HashMap::new();
        values.insert(parent_id_column.to_string(), SqlValue::Integer(parent_id));

        // Extract other columns from the array element
        for col in schema.columns {
            if col.name == parent_id_column {
                continue; // Already added
            }

            // Use explicit json_field if set, otherwise derive from column name
            let json_key = col
                .json_field
                .map(String::from)
                .unwrap_or_else(|| to_camel_case(col.name));
            let value = extract_value(item, &json_key, &col.col_type);
            values.insert(col.name.to_string(), value);
        }

        rows.push(ParsedRow { values });
    }

    Ok(rows)
}

/// Parse blueprint-style nested structure: activities.{activity}.{field}[]
fn parse_blueprint_activity(
    json: &Value,
    schema: &TableSchema,
    activity_column: &str,
    array_field: &str,
) -> Result<Vec<ParsedRow>> {
    let blueprint_id = json
        .get("blueprintTypeID")
        .or_else(|| json.get("_key"))
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing blueprintTypeID or _key in JSON"))?;

    let activities = match json.get("activities") {
        Some(Value::Object(obj)) => obj,
        _ => return Ok(vec![]),
    };

    let mut rows = Vec::new();

    for (activity_name, activity_data) in activities {
        let array = match activity_data.get(array_field) {
            Some(Value::Array(arr)) => arr,
            _ => continue,
        };

        for item in array {
            let mut values = HashMap::new();
            values.insert("blueprint_id".to_string(), SqlValue::Integer(blueprint_id));
            values.insert(
                activity_column.to_string(),
                SqlValue::Text(activity_name.clone()),
            );

            // Extract other columns from the array element
            for col in schema.columns {
                if col.name == "blueprint_id" || col.name == activity_column {
                    continue; // Already added
                }

                let json_key = to_camel_case(col.name);
                let value = extract_value(item, &json_key, &col.col_type);
                values.insert(col.name.to_string(), value);
            }

            rows.push(ParsedRow { values });
        }
    }

    Ok(rows)
}

/// Parse simple integer array: {"_key": X, "fieldName": [123, 456, ...]}
fn parse_simple_int_array(
    json: &Value,
    array_field: &str,
    parent_id_column: &str,
    value_column: &str,
) -> Result<Vec<ParsedRow>> {
    let parent_id = json
        .get("_key")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing _key in JSON"))?;

    let array = match json.get(array_field) {
        Some(Value::Array(arr)) => arr,
        _ => return Ok(vec![]), // No array or empty
    };

    let mut rows = Vec::with_capacity(array.len());

    for item in array {
        if let Some(value) = item.as_i64() {
            let mut values = HashMap::new();
            values.insert(parent_id_column.to_string(), SqlValue::Integer(parent_id));
            values.insert(value_column.to_string(), SqlValue::Integer(value));
            rows.push(ParsedRow { values });
        }
    }

    Ok(rows)
}

/// Parse nested key-value array: {"_key": X, "types": [{"_key": Y, "_value": [{...}]}]}
fn parse_nested_key_value(
    json: &Value,
    schema: &TableSchema,
    array_field: &str,
    parent_id_column: &str,
    nested_key_column: &str,
) -> Result<Vec<ParsedRow>> {
    let parent_id = json
        .get("_key")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing _key in JSON"))?;

    let outer_array = match json.get(array_field) {
        Some(Value::Array(arr)) => arr,
        _ => return Ok(vec![]),
    };

    let mut rows = Vec::new();

    for outer_item in outer_array {
        let nested_key = match outer_item.get("_key").and_then(|v| v.as_i64()) {
            Some(k) => k,
            None => continue,
        };

        let inner_array = match outer_item.get("_value") {
            Some(Value::Array(arr)) => arr,
            _ => continue,
        };

        for item in inner_array {
            let mut values = HashMap::new();
            values.insert(parent_id_column.to_string(), SqlValue::Integer(parent_id));
            values.insert(nested_key_column.to_string(), SqlValue::Integer(nested_key));

            // Extract other columns from the array element
            for col in schema.columns {
                if col.name == parent_id_column || col.name == nested_key_column {
                    continue;
                }

                let json_key = to_camel_case(col.name);
                let value = extract_value(item, &json_key, &col.col_type);
                values.insert(col.name.to_string(), value);
            }

            rows.push(ParsedRow { values });
        }
    }

    Ok(rows)
}

/// Parse double-nested arrays: {"_key": X, "_value": [{"_key": Y, "_value": [Z, ...]}]}
fn parse_double_nested(
    json: &Value,
    schema: &TableSchema,
    parent_id_column: &str,
    level_key_column: &str,
) -> Result<Vec<ParsedRow>> {
    let parent_id = json
        .get("_key")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow::anyhow!("Missing _key in JSON"))?;

    let outer_array = match json.get("_value") {
        Some(Value::Array(arr)) => arr,
        _ => return Ok(vec![]),
    };

    let mut rows = Vec::new();

    for outer_item in outer_array {
        let level_key = match outer_item.get("_key").and_then(|v| v.as_i64()) {
            Some(k) => k,
            None => continue,
        };

        let inner_array = match outer_item.get("_value") {
            Some(Value::Array(arr)) => arr,
            _ => continue,
        };

        for item in inner_array {
            let mut values = HashMap::new();
            values.insert(parent_id_column.to_string(), SqlValue::Integer(parent_id));
            values.insert(level_key_column.to_string(), SqlValue::Integer(level_key));

            // For double-nested, the innermost values are typically just integers (e.g., certificate_id)
            // Find the remaining column (not parent_id or level_key) and set its value
            for col in schema.columns {
                if col.name == parent_id_column || col.name == level_key_column {
                    continue;
                }

                // The inner value is typically a plain integer
                let value = match item.as_i64() {
                    Some(i) => SqlValue::Integer(i),
                    None => extract_value(item, &to_camel_case(col.name), &col.col_type),
                };
                values.insert(col.name.to_string(), value);
            }

            rows.push(ParsedRow { values });
        }
    }

    Ok(rows)
}

/// Parse a JSON line into a row for the given table schema
pub fn parse_record(line: &str, schema: &TableSchema) -> Result<ParsedRow> {
    let json: Value = serde_json::from_str(line).context("Failed to parse JSON")?;

    let mut values = HashMap::new();

    for col in schema.columns {
        match col.col_type {
            ColumnType::Localized => {
                // Handle localized fields
                let json_key = to_camel_case(col.name);
                if let Some(obj) = json.get(&json_key).and_then(|v| v.as_object()) {
                    for lang in LANGUAGES {
                        let col_name = format!("{}_{}", col.name, lang);
                        let value = obj
                            .get(*lang)
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
            ColumnType::Integer => v.as_i64().map(SqlValue::Integer).unwrap_or(SqlValue::Null),
            ColumnType::Real => v.as_f64().map(SqlValue::Real).unwrap_or(SqlValue::Null),
            ColumnType::Text => v
                .as_str()
                .map(|s| SqlValue::Text(s.to_string()))
                .unwrap_or(SqlValue::Null),
            ColumnType::Boolean => v
                .as_bool()
                .map(|b| SqlValue::Integer(if b { 1 } else { 0 }))
                .unwrap_or(SqlValue::Null),
            ColumnType::Json => SqlValue::Text(v.to_string()),
            ColumnType::Localized => SqlValue::Null, // Handled separately
        },
    }
}

/// Convert snake_case to camelCase
/// Handles special case: `_id` suffix becomes `ID` (e.g., category_id -> categoryID)
fn to_camel_case(s: &str) -> String {
    // Handle _id suffix specially (EVE uses categoryID, groupID, etc.)
    if let Some(prefix) = s.strip_suffix("_id") {
        let prefix_camel = to_camel_case_inner(prefix);
        return format!("{}ID", prefix_camel);
    }

    to_camel_case_inner(s)
}

fn to_camel_case_inner(s: &str) -> String {
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
        assert_eq!(to_camel_case("group_id"), "groupID");
        assert_eq!(to_camel_case("category_id"), "categoryID");
        assert_eq!(to_camel_case("solar_system_id"), "solarSystemID");
        assert_eq!(to_camel_case("name"), "name");
        assert_eq!(to_camel_case("sof_faction_name"), "sofFactionName");
    }
}
