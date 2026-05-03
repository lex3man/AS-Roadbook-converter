pub fn matches_debug(matches: &clap::ArgMatches, state: &mut crate::State) {
    if matches.get_flag("debug") {
        state.debug = true;
        println!("DEBUG mode is ON");
    }
    if let Some(source) = matches.get_one::<String>("source") {
        state.source = source.clone();
        if state.debug {
            println!("[DEBUG] Source file set to {}", source);
        }
    }
    if let Some(output) = matches.get_one::<String>("output") {
        state.output = output.clone();
        if state.debug {
            println!("[DEBUG] Output directory set to {}", output);
        }
    }
}