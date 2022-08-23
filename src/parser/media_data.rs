use std::collections::HashMap;
use crate::parser::parse::Parse;

use anyhow::Result;
use crate::parser::Parser;

const TYPE_HEADER: &str = "#EXT-X-MEDIA:";

pub struct MediaData {
    ttype: String,
    group_id: String,
    name: String,
    language: String,
    default: String,
    autoselect: String,
    channels: i32,
    uri: String
}

impl Parse for MediaData {
    fn from_string(contents: String) -> Result<Box<Self>> {
        let mut fields: HashMap<String, String> = HashMap::new();
        let cleaned_contents = contents.replace(TYPE_HEADER, "");
        let iter_contents = cleaned_contents.split(',').collect::<Vec<&str>>();
        for c in iter_contents {
            let pair = c.split('=').collect::<Vec<_>>();
            fields.insert(pair[0].to_string(), pair[1].to_string());
        };
        Self::from_hashmap(fields)
    }

    fn from_hashmap(map: HashMap<String, String>) -> Result<Box<Self>> {
        Ok(
            Box::new(
                Self {
                    ttype: Parser::parse_to_string("TYPE", &map)?,
                    group_id: Parser::parse_to_string("GROUP-ID", &map)?,
                    name: Parser::parse_to_string("NAME", &map)?,
                    language: Parser::parse_to_string("LANGUAGE", &map)?,
                    default: Parser::parse_to_string("DEFAULT", &map)?,
                    autoselect: Parser::parse_to_string("AUTOSELECT", &map)?,
                    channels: Parser::parse_to_i32("CHANNEL", &map)?,
                    uri: Parser::parse_to_string("URI", &map)?
                }
            )
        )
    }
}
