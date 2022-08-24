pub mod models;

use clap::{arg, Command};

pub fn cli() -> Command<'static> {
    Command::new("M3U Parser")
        .about("Parses and sorts an M3U file")
        .arg(
            arg!([PATH])
                .long("--path")
                .short('p')
                .help(
                    "Specifies the path to the .m3u file. Defaults to file provided."
                )
        )
        .subcommand(
            Command::new("--sort")
            .short_flag('s')
            .about("[Optional] Specifies fields on which to sort.")
            .arg(arg!([SORT]))
        )
        .subcommand(
            Command::new("--output")
            .short_flag('o')
            .about(
                "[Optional] Allows the user to specify an output file (.txt). If no file is specified, no output will be written to disk."
            )
            .arg(arg!([OUTPUT]))
        )
        
}