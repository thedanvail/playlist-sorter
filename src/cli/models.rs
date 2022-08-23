use std::str::FromStr;
use crate::sorting::SortOptions;

pub struct Options {
    content: String,
    sort: Option<SortOptions>,
    output: Option<String>
}

impl Options {
    pub fn new(
        contents: String,
        sorting_method: Option<SortOptions>,
        output_path: Option<String>
    ) -> Self {
        Self { content: contents, sort: sorting_method, output: output_path }
    }
}
