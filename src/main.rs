mod utils;

pub struct State {
    pub debug: bool,
    pub source: String,
    pub output: String,
    pub padding: String,
    pub height: String,
    pub width: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            debug: false,
            source: String::new(),
            output: String::new(),
            padding: String::from("107"),
            height: String::from("206"),
            width: String::from("816"),
        }
    }
}

fn main() {
    let mut state = State::default();
    let matches = utils::cli::cli().get_matches();
    utils::debug::matches_debug(&matches, &mut state);

    utils::pdf_to_png::convert_to_png(&state).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    utils::split_png::make_slices(&state).unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });
}
