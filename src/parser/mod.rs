pub mod line;
pub mod parse;
pub mod media_data;
pub mod stream_info;
pub mod i_frame_stream_info_data;

use std::collections::HashMap;
use anyhow::{Context, Result};

pub struct Parser;

impl Parser {
    fn parse_to_i32(field: &str, map: &HashMap<String, String>) -> Result<i32> {
        let val = Self::get_val(field, map)?;
        val.parse::<i32>().with_context(|| format!("Failed to parse {} as i64", val))
    }

    fn parse_to_string(field: &str, map: &HashMap<String, String>) -> Result<String> {
        let val = Self::get_val(field, map)?;
        Ok(String::from(val))
    }

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

    fn parse_to_vec_string(
        field: &str,
        map: &HashMap<String, String>
    ) -> Result<Vec<String>> {
        let val = Self::get_val(field, map)?;
        Ok(val.split(',').map(|x| String::from(x)).collect::<Vec<String>>())
    }

    fn parse_to_f32(field: &str, map: &HashMap<String, String>) -> Result<f32> {
        let val = Self::get_val(field, map)?;
        val.parse::<f32>().with_context(|| format!("Failed to parse {} to f32", val))
    }

    fn get_val<'a>(field: &str, map: &'a HashMap<String, String>) -> Result<&'a String> {
        map.get(field).with_context(|| {
            format!("Map did not contain value of {}", field)
        })
    }
}

pub fn parse_text(text: String) -> HashMap<String, Vec<String>> {
    let splits = text
                    .split('\n')
                    .map(|x| std::string::String::from(x))
                    .collect::<Vec<String>>();
}
