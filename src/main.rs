mod utils;

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

fn main() {
    let mut state = State::default();
    let matches = utils::cli::cli().get_matches();
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
