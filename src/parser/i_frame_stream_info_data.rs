use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Formatter;
use crate::parser::parse::Parse;

use anyhow::Result;
use crate::parser::{headers, Parser};
use crate::sorting::sort::Sort;

pub struct IFrameStreamInfoData {
    bandwidth: i32,
    codecs: String,
    resolution: (i32, i32),
    video_range: String,
    uri: String
}

impl Parse for IFrameStreamInfoData {
    fn from_string(contents: String) -> Result<Box<Self>> {
        let mut fields: HashMap<String, String> = HashMap::new();
        let cleaned_contents = contents.replace(&headers::I_FRAME_STREAM, "");
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
                    codecs: Parser::parse_to_string("CODECS", &map)?,
                    resolution: Parser::parse_to_tuple_i32("RESOLUTION", &map)?,
                    video_range: Parser::parse_to_string("VIDEO-RANGE", &map)?,
                    uri: Parser::parse_to_string("URI", &map)?
                }
            )
        )
    }
}

impl Sort for IFrameStreamInfoData {
    fn compare_resolution(&self, other: &Self) -> Ordering {
        let (x, y) = self.resolution;
        let total_resolution = x * y;
        let (other_x, other_y) = other.resolution;
        let other_total_resolution = other_x * other_y;
        total_resolution.cmp(&other_total_resolution)
    }

    fn compare_bandwidth(&self, other: &Self) -> Ordering {
        self.bandwidth.cmp(&other.bandwidth)
    }
}

impl std::fmt::Display for IFrameStreamInfoData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\
            BANDWIDTH={},\
            CODECS={},\
            RESOLUTION={}x{},\
            VIDEO-RANGE={},\
            URI={}\
            ",
            self.bandwidth,
            self.codecs,
            self.resolution.0, self.resolution.1,
            self.video_range,
            self.uri
        )
    }
}
