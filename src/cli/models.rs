use crate::sorting::SortOptions;

pub struct Options {
    pub content_path: String,
    pub sort: Option<SortOptions>,
    pub output: Option<String>
}

impl Options {
    pub fn new(
        content_path: &String,
        sorting_method: Option<SortOptions>,
        output_path: Option<String>
    ) -> Self {
        Self { content_path: content_path.to_owned(), sort: sorting_method, output: output_path }
    }
}
