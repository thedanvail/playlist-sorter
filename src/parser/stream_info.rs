use std::collections::HashMap;

use crate::parser::parse::Parse;
use crate::parser::Parser;

use anyhow::Result;

const TYPE_HEADER: &str = "#EXT-X-STREAM-INF:";

pub struct StreamInfoData {
    bandwidth: i32,
    average_bandwidth: i32,
    codecs:Vec<String>,
    resolution: (i32, i32),
    framerate: f32,
    video_range: String,
    audio: String,
    closed_captions: String
}

impl Parse for StreamInfoData {
    fn from_string(contents: String) -> Result<Box<StreamInfoData>> {
        let mut fields: HashMap<String, String> = std::collections::HashMap::new();
        let cleaned_contents = contents.replace(TYPE_HEADER, "");
        let iter_contents = cleaned_contents.split(',').collect::<Vec<&str>>();
        for c in iter_contents {
            let pair = c.split('=').collect::<Vec<_>>();
            fields.insert(pair[0].to_string(), pair[1].to_string());
        };
        Self::from_hashmap(fields)
    }

    fn from_hashmap(map: HashMap<String, String>) -> Result<Box<StreamInfoData>> {
        Ok(
            Box::new(
                Self {
                    bandwidth: Parser::parse_to_i32("BANDWIDTH", &map)?,
                    average_bandwidth: Parser::parse_to_i32("AVERAGE-BANDWIDTH", &map)?,
                    codecs: Parser::parse_to_vec_string("CODECS", &map)?,
                    resolution: Parser::parse_to_tuple_i32("RESOLUTION", &map)?,
                    framerate: Parser::parse_to_f32("FRAME-RATE", &map)?,
                    video_range: Parser::parse_to_string("VIDEO-RANGE", &map)?,
                    audio: Parser::parse_to_string("AUDIO", &map)?,
                    closed_captions: Parser::parse_to_string("CLOSED-CAPTIONS", &map)?
                }
            )
        )
    }
}