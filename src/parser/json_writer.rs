use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonObject {
    pub data: Vec<HashMap<String, Value>>,
}

impl JsonObject {
    pub fn write_to_file(self, path: &str) -> Result<(), String> {
        let text = match serde_json::to_string(&self.data) {
            Ok(t) => t,
            Err(e) => return Err(format!("Error format of parsed result.\n{}", e)),
        };

        let mut file = match fs::File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Cannot create file {}.\n{}", path, e)),
        };

        file.write_all(text.as_bytes())
            .map_err(|e| format!("Error writing to {}.\n{}", path, e))?;

        Ok(())
    }
}
