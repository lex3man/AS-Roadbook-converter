use clap::{Arg, ArgAction};

pub fn cli() -> clap::Command {
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