pub mod sort;

use sort::Sort;

use std::fmt::Display;
use std::str::FromStr;
use crate::parser::line::Line;
use anyhow::Result;

#[derive(Clone)]
pub enum SortOptions {
    Resolution,
    Bandwidth
}

fn sort_options_to_string() -> String {
    String::from("Resolution, Bandwidth")
}

impl FromStr for SortOptions {
    type Err = clap::Error;

    /// Converts the `&str` into a `Result<SortOptions, Err>`. This can be updated later as
    /// we add support for more types of sorting.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "RESOLUTION" => Ok(Self::Resolution),
            "BANDWIDTH" => Ok(Self::Bandwidth),
            _ => Err(clap::Error::raw(
                clap::ErrorKind::UnknownArgument,
                format!("Valid sort options are {}", sort_options_to_string())
            ))
        }
    }
}

/// Detects the type of sorting the user is requesting, and then passes that onto the
/// underlying struct that implements `Sort`.
pub fn sort<T: Sort + Display>(lines: &mut [Line<T>], sort_option: &SortOptions) {
    match sort_option {
        SortOptions::Resolution => {
            lines.sort_by(|a, b| a.data.compare_resolution(&b.data))
        },
        SortOptions::Bandwidth => {
            lines.sort_by(|a, b| a.data.compare_bandwidth(&b.data))
        }
    };
}
