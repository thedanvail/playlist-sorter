use std::collections::HashMap;
use crate::parser::parse::Parse;

use anyhow::Result;
use crate::parser::Parser;

const TYPE_HEADER: &str = "#EXT-X-I-FRAME-STREAM-INF:";

pub struct IFrameStreamInfoData {
    bandwidth: i32,
    codecs: Vec<String>,
    resolution: (i32, i32),
    video_range: String,
    uri: String
}

impl Parse for IFrameStreamInfoData {
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
                    bandwidth: Parser::parse_to_i32("BANDWIDTH", &map)?,
                    codecs: Parser::parse_to_vec_string("CODECS", &map)?,
                    resolution: Parser::parse_to_tuple_i32("RESOLUTION", &map)?,
                    video_range: Parser::parse_to_string("VIDEO-RANGE", &map)?,
                    uri: Parser::parse_to_string("URI", &map)?
                }
            )
        )
    }
}