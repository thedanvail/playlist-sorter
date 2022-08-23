pub mod models;

use clap::{arg, command, ArgGroup, ArgMatches, Command, ValueEnum};

pub fn cli() -> Command<'static> {
    Command::new("M3U Parser")
        .about("Parses and sorts an M3U file")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("--path")
            .short_flag('p')
            .about(
                "[Optional] Specifies the path to the .m3u file. Defaults to file provided."
            )
            .arg(arg!([PATH]))
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
                "[Optional] Allows the user to specify an output file (.json or .txt). If no file is specified, no output will be written to disk."
            )
            .arg(arg!([OUTPUT]))
        )
        
}