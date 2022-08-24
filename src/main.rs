use std::str::FromStr;

mod parser;
mod cli;
mod utils;
mod sorting;

use cli::models::Options;
use crate::parser::i_frame_stream_info_data::IFrameStreamInfoData;
use crate::parser::media_data::MediaData;
use crate::parser::stream_info::StreamInfoData;
use crate::sorting::SortOptions;

fn main() {
    let matches = cli::cli().get_matches();
    let mut content_path: &String = &String::from("master_unenc_hdr10_all.m3u");
    if let Some(path) = matches.get_one::<String>("PATH") {
        content_path = path;
    }
    let mut sorting_method: Option<sorting::SortOptions> = None;
    let mut output_path: Option<String> = None;
    match matches.subcommand() {
        Some(("--sort", sub_matches)) => {
            if let Some(opt) = sub_matches.get_one::<String>("SORT") {
                sorting_method = Some(SortOptions::from_str(opt.as_str()).unwrap())
            }
        },
        Some(("--output", sub_matches)) => {
            output_path = Some(sub_matches.get_one::<String>("OUTPUT").unwrap().to_owned());
        }
        _ => {},
    }
    let options: Options = Options::new(content_path, sorting_method, output_path);
    dispatch_options(options);
}

/// The driver (or, the real tofu and potatoes)
fn dispatch_options(options: Options) {
    let content = utils::read_file(&options.content_path).unwrap();
    let (
        media_data_vec,
        mut i_frame_vec,
        mut stream_info_vec
    ) = parser::parse_text(content);
    if let Some(sort_field) = options.sort {
        sorting::sort::<IFrameStreamInfoData>(&mut i_frame_vec, &sort_field);
        sorting::sort::<StreamInfoData>(&mut stream_info_vec, &sort_field);
    }
    utils::write_to_console::<IFrameStreamInfoData>(&i_frame_vec);
    utils::write_to_console::<StreamInfoData>(&stream_info_vec);
    utils::write_to_console::<MediaData>(&media_data_vec);
    if let Some(output_path) = options.output {
        utils::write_output_to_file(
            media_data_vec,
            i_frame_vec,
            stream_info_vec,
            output_path
        ).unwrap();
    }
}
