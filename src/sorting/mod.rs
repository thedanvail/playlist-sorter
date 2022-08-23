use std::str::FromStr;
use crate::parser::line::Line;
use anyhow::Result;

#[derive(Clone)]
pub enum SortOptions {
    Resolution,
    Bandwidth
}

fn sort_to_string() -> String {
    String::from("Resolution, Bandwidth")
}

impl FromStr for SortOptions {
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

pub fn sort<T>(lines: Vec<Line<T>>, sort_option: SortOptions) -> Result<Vec<Line<T>>> {
    match sort_option {
        SortOptions::Resolution => {
            let mut sorted = lines.clone();
        },
        SortOptions::Bandwidth => {

        }
    };
}
