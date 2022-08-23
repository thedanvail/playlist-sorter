mod parser;

use cli::models::Options;

mod cli;
mod utils;

fn main() {
    let matches = cli::cli().get_matches();
    let mut content = String::from("");
    let sorting_method: Option<cli::models::Sort> = None;
    let output_path: Option<String> = None;
    match matches.subcommand() {
        Some(("--path", sub_matches)) => {
            let uri = sub_matches.get_one::<String>("PATH").unwrap();
            content = utils::read_file(uri).unwrap();
        },
        Some(("--sort", sub_matches)) => {
            println!(
                "received sort matches of {:?}",
                sub_matches.get_one::<String>("SORT").unwrap()
            )
        },
        Some(("--output", sub_matches)) => {
            println!(
                "received output matches of {:?}",
                sub_matches.get_one::<String>("OUTPUT").unwrap()
            )
        }
        _ => panic!("Received an invalid flag!"),
    }
    let options: Options = Options::new(content, sorting_method, output_path);
}