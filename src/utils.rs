use std::fmt::Display;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::{Result, Context, anyhow};
use crate::{IFrameStreamInfoData, MediaData};
use crate::parser::line::Line;
use crate::parser::stream_info::StreamInfoData;

/// Writes a file to the output path provided in the CLI args
pub fn write_output_to_file(
    media: Vec<Line<MediaData>>,
    i_frame: Vec<Line<IFrameStreamInfoData>>,
    stream_info: Vec<Line<StreamInfoData>>,
    path: String
) -> anyhow::Result<()> {
    if !path.ends_with(".txt") {
        return Err(anyhow!("Specified output file format is unsupported"));
    }
    let output_path = Path::new(path.as_str());
    let mut opened_file = match File::create(output_path) {
        Ok(f) => f,
        Err(_) => File::open(output_path)?
    };
    for line in media.iter() {
        opened_file.write_fmt(format_args!("{}\n", line))?;
    }
    for line in i_frame.iter() {
        opened_file.write_fmt(format_args!("{}\n", line))?;
    }
    for line in stream_info.iter() {
        opened_file.write_fmt(format_args!("{}\n", line))?;
    }
    Ok(())
}

/// Reads a file (locally or remote) and translates it to a String
///
/// Will panic if the file read doesn't work or if the server sends an error response
pub fn read_file(uri: &String) -> Result<String> {
    if uri.contains("https://") | uri.contains("http://") {
        get_remote_file(uri)
    }
    else {
        std::fs::read_to_string(uri).with_context(|| format!("Failed to read file from {:?}", uri))
    }
}

/// Reads a file from a remote server
pub fn get_remote_file(uri: &String) -> Result<String> {
    match reqwest::blocking::get(uri) {
        Ok(resp) => resp.text().with_context(|| format!("Failed to unwrap response from {}", uri)),
        Err(resp) => Err(anyhow!("Received a response from server: {}", resp)),
    }
}

/// Writes the given array to the console
pub fn write_to_console<T: Display>(vec: &[Line<T>]) {
    for line in vec.iter() {
        println!("{}", line);
    }
}
