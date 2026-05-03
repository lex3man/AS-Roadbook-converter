mod utils;

use clap::{Arg, ArgAction};

pub struct State {
    pub debug: bool,
    pub source: String,
    pub output: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            debug: false,
            source: String::new(),
            output: String::new(),
        }
    }
}

fn cli() -> clap::Command {
    clap::Command::new("myapp")
        .version("1.0")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Activate debug mode")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            Arg::new("source")
                .short('s')
                .long("source")
                .required(true)
                .help("Input file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .default_value("output")
                .required(false)
                .help("Output directory"),
        )
        .author("Author Name <lex3man@gmail.com>")
}

fn main() {
    let mut state = State::default();
    let matches = cli().get_matches();
    utils::debug::matches_debug(&matches, &mut state);

    utils::pdf_to_png::convert_to_png(&state).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    utils::chank_png::make_slices(&state).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}
