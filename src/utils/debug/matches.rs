pub fn matches_debug(matches: &clap::ArgMatches, state: &mut crate::State) {
    if matches.get_flag("debug") {
        state.debug = true;
        println!("Debug mode is on");
    }
    if let Some(source) = matches.get_one::<String>("source") {
        state.source = source.clone();
        if state.debug {
            println!("Debug: Source file set to {}", source);
        }
    }
    if let Some(output) = matches.get_one::<String>("output") {
        state.output = output.clone();
        if state.debug {
            println!("Debug: Output directory set to {}", output);
        }
    }
}