use wasm_bindgen::prelude::*;
// Console logging is handled via the log macro defined below
use crate::{PngDatabase, Schema, DataRow};
use serde_json::Value;
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct WebPngDatabase {
    db: PngDatabase,
}

#[wasm_bindgen]
impl WebPngDatabase {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32, schema_json: &str) -> Result<WebPngDatabase, JsValue> {
        let schema_map: HashMap<String, String> = serde_json::from_str(schema_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid schema JSON: {}", e)))?;

        let schema = Schema { fields: schema_map };
        let db = PngDatabase::new(width, height, schema);

        Ok(WebPngDatabase { db })
    }

    #[wasm_bindgen]
    pub fn from_png_bytes(png_bytes: &[u8]) -> Result<WebPngDatabase, JsValue> {
        // Create a temporary file-like reader from bytes
        let cursor = std::io::Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let reader = decoder.read_info()
            .map_err(|e| JsValue::from_str(&format!("PNG decode error: {}", e)))?;

        let info = reader.info();
        let width = info.width;
        let height = info.height;

        let mut schema = Schema { fields: HashMap::new() };
        let mut rows = Vec::new();

        // Read zTXt chunks
        for chunk in reader.info().compressed_latin1_text.iter() {
            if chunk.keyword == "schema" {
                let decompressed = chunk.get_text()
                    .map_err(|e| JsValue::from_str(&format!("Schema decompression error: {}", e)))?;
                schema = serde_json::from_str(&decompressed)
                    .map_err(|e| JsValue::from_str(&format!("Schema parse error: {}", e)))?;
            } else if chunk.keyword.starts_with("row_") {
                let decompressed = chunk.get_text()
                    .map_err(|e| JsValue::from_str(&format!("Row decompression error: {}", e)))?;
                let row_data: Value = serde_json::from_str(&decompressed)
                    .map_err(|e| JsValue::from_str(&format!("Row parse error: {}", e)))?;

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

        let db = PngDatabase {
            width,
            height,
            schema,
            rows,
        };

        Ok(WebPngDatabase { db })
    }

    #[wasm_bindgen]
    pub fn insert(&mut self, x: u32, y: u32, data_json: &str) -> Result<(), JsValue> {
        let data: Value = serde_json::from_str(data_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid JSON data: {}", e)))?;

        self.db.insert(x, y, data)
            .map_err(|e| JsValue::from_str(&format!("Insert error: {}", e)))?;

        Ok(())
    }

    #[wasm_bindgen]
    pub fn query(&self, where_clause: &str) -> Result<String, JsValue> {
        let results = self.db.query(where_clause)
            .map_err(|e| JsValue::from_str(&format!("Query error: {}", e)))?;

        let serializable_results: Vec<_> = results.iter().map(|row| {
            serde_json::json!({
                "x": row.x,
                "y": row.y,
                "data": row.data
            })
        }).collect();

        serde_json::to_string(&serializable_results)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    #[wasm_bindgen]
    pub fn list_all(&self) -> Result<String, JsValue> {
        let serializable_rows: Vec<_> = self.db.rows.iter().map(|row| {
            serde_json::json!({
                "x": row.x,
                "y": row.y,
                "data": row.data
            })
        }).collect();

        serde_json::to_string(&serializable_rows)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
    }

    #[wasm_bindgen]
    pub fn to_png_bytes(&self) -> Result<Vec<u8>, JsValue> {
        let mut buf = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut buf, self.db.width, self.db.height);
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);

            // Add schema as zTXt chunk
            let schema_json = serde_json::to_string(&self.db.schema)
                .map_err(|e| JsValue::from_str(&format!("Schema serialization error: {}", e)))?;
            encoder.add_ztxt_chunk("schema".to_string(), schema_json)
                .map_err(|e| JsValue::from_str(&format!("Schema chunk error: {}", e)))?;

            // Add each row as a zTXt chunk
            for row in &self.db.rows {
                let row_json = serde_json::to_string(&row.data)
                    .map_err(|e| JsValue::from_str(&format!("Row serialization error: {}", e)))?;
                let keyword = format!("row_{}_{}", row.x, row.y);
                encoder.add_ztxt_chunk(keyword, row_json)
                    .map_err(|e| JsValue::from_str(&format!("Row chunk error: {}", e)))?;
            }

            let mut writer = encoder.write_header()
                .map_err(|e| JsValue::from_str(&format!("PNG header error: {}", e)))?;

            // Create a simple RGB image with black pixels
            let image_data = vec![0u8; (self.db.width * self.db.height * 3) as usize];
            writer.write_image_data(&image_data)
                .map_err(|e| JsValue::from_str(&format!("PNG data error: {}", e)))?;
            writer.finish()
                .map_err(|e| JsValue::from_str(&format!("PNG finish error: {}", e)))?;
        }

        Ok(buf)
    }

    #[wasm_bindgen]
    pub fn get_schema(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.db.schema.fields)
            .map_err(|e| JsValue::from_str(&format!("Schema serialization error: {}", e)))
    }

    #[wasm_bindgen]
    pub fn get_dimensions(&self) -> Vec<u32> {
        vec![self.db.width, self.db.height]
    }

    #[wasm_bindgen]
    pub fn get_row_count(&self) -> usize {
        self.db.rows.len()
    }
}
