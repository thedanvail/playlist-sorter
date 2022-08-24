pub mod line;
pub mod parse;
pub mod media_data;
pub mod stream_info;
pub mod i_frame_stream_info_data;
mod headers;

use std::collections::HashMap;
use anyhow::{Context, Result};
use crate::parser::line::Line;

use crate::parser::i_frame_stream_info_data::IFrameStreamInfoData;
use crate::parser::media_data::MediaData;
use crate::parser::stream_info::StreamInfoData;

pub struct Parser;

type Lines<T> = Vec<Line<T>>;

impl Parser {
    /// Parses a given &str to an i32, erring if the parse fails.
    fn parse_to_i32(field: &str, map: &HashMap<String, String>) -> Result<i32> {
        let val = Self::get_val(field, map)?;
        if val.starts_with('\"') || val.ends_with('\"') {
            return val.replace('\"', "")
                .parse::<i32>()
                .with_context(|| format!("Failed to parse {} as i64", val));
        };
        val.parse::<i32>().with_context(|| format!("Failed to parse {} as i64", val))
    }

    /// Parses a given &str to a String, erring if the parse fails.
    fn parse_to_string(field: &str, map: &HashMap<String, String>) -> Result<String> {
        let val = Self::get_val(field, map)?;
        Ok(String::from(val))
    }

    /// Parses a given &str to a tuple of (i32, i32), erring if the parse fails.
    fn parse_to_tuple_i32(field: &str, map: &HashMap<String, String>) -> Result<(i32, i32)> {
        let val = Self::get_val(field, map)?;
        let res = val.split('x').collect::<Vec<&str>>();
        let x_res = res[0].parse::<i32>().with_context(
            || format!("Could not parse {} to i32", res[0])
        );
        let y_res = res[1].parse::<i32>().with_context(
            || format!("Could not parse {} to i32", res[1])
        );
        Ok((x_res?, y_res?))
    }

    /// Parses a given &str to an f32, erring if the parse fails.
    fn parse_to_f32(field: &str, map: &HashMap<String, String>) -> Result<f32> {
        let val = Self::get_val(field, map)?;
        val.parse::<f32>().with_context(|| format!("Failed to parse {} to f32", val))
    }

    /// Helper function to add more context to the HashMap
    fn get_val<'a>(field: &str, map: &'a HashMap<String, String>) -> Result<&'a String> {
        map.get(field).with_context(|| {
            format!("Map did not contain value of {}", field)
        })
    }
}

/// Takes the given text and tries to transform it into multiple Vectors of Lines of T, where
/// T is one of [MediaData, IFrameStreamInfoData, StreamInfoData].
/// It will strip the header from the line (which can be reinserted later) and retains only
/// the structural data.
pub fn parse_text(text: String) -> (
    Lines<MediaData>, Lines<IFrameStreamInfoData>, Lines<StreamInfoData>
) {
    let splits = text
                    .split('\n')
                    .map(std::string::String::from)
                    .filter(|s|
                        s.contains('#') &&
                        !s.contains(&headers::M3U) &&
                        !s.contains(&headers::INDEPENDENT_SEGMENTS)
                    )
                    .collect::<Vec<String>>();
    let mut media_data_vec: Vec<Line<_>> = vec!();
    let mut i_frame_data_vec: Vec<Line<_>> = vec!();
    let mut stream_info_data_vec: Vec<Line<_>> = vec!();
    for line in splits.iter() {
        if line.contains(&headers::MEDIA_DATA) {
            media_data_vec.push(Line::media_data(line.to_string()))
        }
        else if line.contains(&headers::I_FRAME_STREAM) {
            i_frame_data_vec.push(Line::i_frame_stream_info_data(line.to_string()))
        }
        else if line.contains(&headers::STREAM_INFO_DATA) {
            stream_info_data_vec.push(Line::stream_info_data(line.to_string()))
        }
    }
    (media_data_vec, i_frame_data_vec, stream_info_data_vec)
}
