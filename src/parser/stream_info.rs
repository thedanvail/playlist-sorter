use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

use crate::parser::parse::Parse;
use crate::parser::{headers, Parser};
use crate::sorting::sort::Sort;

use anyhow::Result;

pub struct StreamInfoData {
    bandwidth: i32,
    average_bandwidth: i32,
    codecs: String,
    resolution: (i32, i32),
    framerate: f32,
    video_range: String,
    audio: String,
    closed_captions: String
}

impl Parse for StreamInfoData {
    fn from_string(contents: String) -> Result<Box<StreamInfoData>> {
        let mut fields: HashMap<String, String> = std::collections::HashMap::new();
        let cleaned_contents = contents.replace(&headers::STREAM_INFO_DATA, "");
        let iter_contents = cleaned_contents.split(',').collect::<Vec<&str>>();
        for c in iter_contents {
            let pair = c.split('=').collect::<Vec<_>>();
            if pair[0].starts_with("hvc") {
                let mut codec = fields.get_mut("CODECS").unwrap().to_owned();
                codec = format!("{},{}", codec, pair[0]).to_owned();
                fields.insert("CODECS".to_string(), codec.to_string());
            }
            else {
                fields.insert(pair[0].to_string(), pair[1].to_string());
            }
        };
        Self::from_hashmap(fields)
    }

    fn from_hashmap(map: HashMap<String, String>) -> Result<Box<StreamInfoData>> {
        Ok(
            Box::new(
                Self {
                    bandwidth: Parser::parse_to_i32("BANDWIDTH", &map)?,
                    average_bandwidth: Parser::parse_to_i32("AVERAGE-BANDWIDTH", &map)?,
                    codecs: Parser::parse_to_string("CODECS", &map)?,
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

impl Sort for StreamInfoData {
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

impl std::fmt::Display for StreamInfoData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\
            BANDWIDTH={},\
            AVERAGE-BANDWIDTH={},\
            CODECS={},\
            RESOLUTION={}x{},\
            FRAME-RATE={},\
            VIDEO-RANGE={},\
            AUDIO={},\
            CLOSED-CAPTIONS={}\
            ",
            self.bandwidth,
            self.average_bandwidth,
            self.codecs,
            self.resolution.0, self.resolution.1,
            self.framerate,
            self.video_range,
            self.audio,
            self.closed_captions
        )
    }
}