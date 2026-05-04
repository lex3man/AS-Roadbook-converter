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
        .arg(
            Arg::new("top padding")
                .short('p')
                .long("padding")
                .default_value("114")
                .required(false)
                .help("Top padding"),
        )
        .arg(
            Arg::new("slice height")
                .long("height")
                .default_value("215")
                .required(false)
                .help("Slice height"),
        )
        .arg(
            Arg::new("slice width")
                .long("width")
                .default_value("840")
                .required(false)
                .help("Slice width"),
        )
        .author("Author Name <lex3man@gmail.com>")
}