use std::collections::HashMap;
use anyhow::{Result, Context, anyhow};

pub fn write_output_to_file(map: HashMap<String, String>, path: &str) {
    for (k, v) in map.iter() {
        // TODO:
        todo!()
    }
}

pub fn read_file(uri: &String) -> Result<String> {
    if uri.contains("https://") | uri.contains("http://") {
        get_remote_file(uri)
    }
    else {
        std::fs::read_to_string(uri).with_context(|| format!("Failed to read file from {:?}", uri))
    }
}

pub fn get_remote_file(uri: &String) -> Result<String> {
    match reqwest::blocking::get(uri) {
        Ok(resp) => resp.text().with_context(|| format!("Failed to unwrap response from {}", uri)),
        Err(resp) => Err(anyhow!("Received a response from server: {}", resp)),
    }
}