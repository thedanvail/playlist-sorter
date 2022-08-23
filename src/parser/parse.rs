use std::collections::HashMap;

use anyhow::Result;

pub trait Parse {
    fn from_string(contents: String) -> Result<Box<Self>>;

    fn from_hashmap(map: HashMap<String, String>) -> Result<Box<Self>>;
}
