use clap::{Args, ValueEnum};
use std::str::FromStr;

pub struct Options {
    content: String,
    sort: Option<Sort>,
    output: Option<String>
}

impl Options {
    pub fn new(
        contents: String,
        sorting_method: Option<Sort>,
        output_path: Option<String>
    ) -> Self {
        Self { content: contents, sort: sorting_method, output: output_path }
    }
}

#[derive(Clone)]
pub enum Sort {
    Resolution,
    Bandwidth
}

fn sort_to_string() -> String {
    String::from("Resolution, Bandwidth")
}

impl FromStr for Sort {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RESOLUTION" => Ok(Self::Resolution),
            "BANDWIDTH" => Ok(Self::Bandwidth),
            _ => Err(clap::Error::raw(
                clap::ErrorKind::UnknownArgument,
                format!("Valid sort options are {}", sort_to_string())
            ))
        }
    }
}
