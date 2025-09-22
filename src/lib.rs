#[cfg(feature = "cli")]
use color_eyre::Result;

#[cfg(not(feature = "cli"))]
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(feature = "wasm")]
pub mod web;
use serde::{Deserialize, Serialize};
use png::{Decoder, Encoder, ColorType, BitDepth};
use serde_json::Value;
use std::collections::HashMap;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
use std::io::{BufReader, BufWriter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PngDbError {
    #[error("PNG format error: {0}")]
    PngError(#[from] png::DecodingError),
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Query error: {0}")]
    QueryError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct DataRow {
    pub x: u32,
    pub y: u32,
    pub data: Value,
}

pub struct PngDatabase {
    pub width: u32,
    pub height: u32,
    pub schema: Schema,
    pub rows: Vec<DataRow>,
}

impl PngDatabase {
    pub fn new(width: u32, height: u32, schema: Schema) -> Self {
        Self {
            width,
            height,
            schema,
            rows: Vec::new(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn create_empty_png(width: u32, height: u32, schema: Schema, filename: &str) -> Result<Self> {
        let db = Self::new(width, height, schema);
        db.save_to_png(filename)?;
        Ok(db)
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_from_png(filename: &str) -> Result<Self> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let decoder = Decoder::new(reader);
        let reader = decoder.read_info()?;

        let info = reader.info();
        let width = info.width;
        let height = info.height;

        let mut schema = Schema { fields: HashMap::new() };
        let mut rows = Vec::new();

        // Read zTXt chunks
        for chunk in reader.info().compressed_latin1_text.iter() {
            if chunk.keyword == "schema" {
                let decompressed_text = chunk.get_text()?;
                schema = serde_json::from_str(&decompressed_text)?;
            } else if chunk.keyword.starts_with("row_") {
                let decompressed_text = chunk.get_text()?;
                let row_data: Value = serde_json::from_str(&decompressed_text)?;
                
                // Extract coordinates from keyword (row_x_y format)
                let coords: Vec<&str> = chunk.keyword.split('_').collect();
                if coords.len() == 3 {
                    let x = coords[1].parse::<u32>().unwrap_or(0);
                    let y = coords[2].parse::<u32>().unwrap_or(0);
                    
                    rows.push(DataRow {
                        x,
                        y,
                        data: row_data,
                    });
                }
            }
        }

        Ok(Self {
            width,
            height,
            schema,
            rows,
        })
    }

    pub fn insert(&mut self, x: u32, y: u32, data: Value) -> Result<()> {
        if x >= self.width || y >= self.height {
            return Err(PngDbError::DatabaseError(
                format!("Coordinates ({}, {}) out of bounds", x, y)
            ).into());
        }

        self.rows.push(DataRow { x, y, data });
        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_to_png(&self, filename: &str) -> Result<()> {
        let file = File::create(filename)?;
        let w = &mut BufWriter::new(file);
        
        let mut encoder = Encoder::new(w, self.width, self.height);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);
        
        // Add schema as zTXt chunk
        let schema_json = serde_json::to_string(&self.schema)?;
        encoder.add_ztxt_chunk("schema".to_string(), schema_json)?;
        
        // Add each row as a zTXt chunk
        for row in &self.rows {
            let row_json = serde_json::to_string(&row.data)?;
            let keyword = format!("row_{}_{}", row.x, row.y);
            encoder.add_ztxt_chunk(keyword, row_json)?;
        }
        
        let mut writer = encoder.write_header()?;
        
        // Create a simple RGB image with black pixels
        let image_data = vec![0u8; (self.width * self.height * 3) as usize];
        writer.write_image_data(&image_data)?;
        writer.finish()?;
        
        Ok(())
    }

    pub fn query(&self, query_str: &str) -> Result<Vec<&DataRow>> {
        let query = parse_query(query_str)?;
        let mut results = Vec::new();

        for row in &self.rows {
            if matches_query(row, &query)? {
                results.push(row);
            }
        }

        Ok(results)
    }
}

#[derive(Debug)]
pub struct Query {
    pub conditions: Vec<Condition>,
}

#[derive(Debug)]
pub enum Condition {
    Coordinate { field: String, op: ComparisonOp, value: u32 },
    JsonField { field: String, op: ComparisonOp, value: Value },
}

#[derive(Debug)]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}

pub fn parse_query(query_str: &str) -> Result<Query> {
    // Simple parser for WHERE clauses
    let query_str = query_str.trim();
    
    if !query_str.to_lowercase().starts_with("where") {
        return Err(PngDbError::QueryError("Query must start with WHERE".to_string()).into());
    }
    
    let conditions_str = &query_str[5..].trim(); // Remove "WHERE"
    let condition_parts: Vec<&str> = conditions_str.split(" AND ").collect();
    
    let mut conditions = Vec::new();
    
    for part in condition_parts {
        let condition = parse_condition(part.trim())?;
        conditions.push(condition);
    }
    
    Ok(Query { conditions })
}

fn parse_condition(condition_str: &str) -> Result<Condition> {
    let operators = [">=", "<=", "!=", "=", ">", "<"];
    
    for op_str in &operators {
        if let Some(pos) = condition_str.find(op_str) {
            let field = condition_str[..pos].trim();
            let value_str = condition_str[pos + op_str.len()..].trim();
            
            let op = match *op_str {
                "=" => ComparisonOp::Equal,
                "!=" => ComparisonOp::NotEqual,
                ">" => ComparisonOp::GreaterThan,
                "<" => ComparisonOp::LessThan,
                ">=" => ComparisonOp::GreaterThanOrEqual,
                "<=" => ComparisonOp::LessThanOrEqual,
                _ => unreachable!(),
            };
            
            // Check if it's a coordinate field
            if field == "x" || field == "y" {
                let value = value_str.parse::<u32>()
                    .map_err(|_| PngDbError::QueryError(format!("Invalid coordinate value: {}", value_str)))?;
                return Ok(Condition::Coordinate {
                    field: field.to_string(),
                    op,
                    value,
                });
            }
            
            // Parse JSON value
            let value = if value_str.starts_with('"') && value_str.ends_with('"') {
                Value::String(value_str[1..value_str.len()-1].to_string())
            } else if let Ok(num) = value_str.parse::<i64>() {
                Value::Number(serde_json::Number::from(num))
            } else if let Ok(float) = value_str.parse::<f64>() {
                Value::Number(serde_json::Number::from_f64(float).unwrap())
            } else if value_str == "true" {
                Value::Bool(true)
            } else if value_str == "false" {
                Value::Bool(false)
            } else {
                Value::String(value_str.to_string())
            };
            
            return Ok(Condition::JsonField {
                field: field.to_string(),
                op,
                value,
            });
        }
    }
    
    Err(PngDbError::QueryError(format!("Invalid condition: {}", condition_str)).into())
}

fn matches_query(row: &DataRow, query: &Query) -> Result<bool> {
    for condition in &query.conditions {
        if !matches_condition(row, condition)? {
            return Ok(false);
        }
    }
    Ok(true)
}

fn matches_condition(row: &DataRow, condition: &Condition) -> Result<bool> {
    match condition {
        Condition::Coordinate { field, op, value } => {
            let coord_value = if field == "x" { row.x } else { row.y };
            Ok(compare_numbers(coord_value as i64, *value as i64, op))
        }
        Condition::JsonField { field, op, value } => {
            if let Some(field_value) = row.data.get(field) {
                compare_json_values(field_value, value, op)
            } else {
                Ok(false)
            }
        }
    }
}

fn compare_numbers(left: i64, right: i64, op: &ComparisonOp) -> bool {
    match op {
        ComparisonOp::Equal => left == right,
        ComparisonOp::NotEqual => left != right,
        ComparisonOp::GreaterThan => left > right,
        ComparisonOp::LessThan => left < right,
        ComparisonOp::GreaterThanOrEqual => left >= right,
        ComparisonOp::LessThanOrEqual => left <= right,
    }
}

fn compare_json_values(left: &Value, right: &Value, op: &ComparisonOp) -> Result<bool> {
    use Value::*;
    
    match (left, right) {
        (String(l), String(r)) => Ok(match op {
            ComparisonOp::Equal => l == r,
            ComparisonOp::NotEqual => l != r,
            _ => return Err(PngDbError::QueryError("String comparison only supports = and !=".to_string()).into()),
        }),
        (Number(l), Number(r)) => {
            let l_val = l.as_f64().unwrap_or(0.0);
            let r_val = r.as_f64().unwrap_or(0.0);
            Ok(match op {
                ComparisonOp::Equal => (l_val - r_val).abs() < f64::EPSILON,
                ComparisonOp::NotEqual => (l_val - r_val).abs() >= f64::EPSILON,
                ComparisonOp::GreaterThan => l_val > r_val,
                ComparisonOp::LessThan => l_val < r_val,
                ComparisonOp::GreaterThanOrEqual => l_val >= r_val,
                ComparisonOp::LessThanOrEqual => l_val <= r_val,
            })
        }
        (Bool(l), Bool(r)) => Ok(match op {
            ComparisonOp::Equal => l == r,
            ComparisonOp::NotEqual => l != r,
            _ => return Err(PngDbError::QueryError("Boolean comparison only supports = and !=".to_string()).into()),
        }),
        _ => Err(PngDbError::QueryError("Cannot compare different value types".to_string()).into()),
    }
}

