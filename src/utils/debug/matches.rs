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
    if let Some(padding) = matches.get_one::<String>("top padding") {
        state.padding = padding.clone();
        if state.debug {
            println!("[DEBUG] Top padding set to {}", padding);
        }
    }
    if let Some(height) = matches.get_one::<String>("slice height") {
        state.height = height.clone();
        if state.debug {
            println!("[DEBUG] Slice height set to {}", height);
        }
    }
    if let Some(width) = matches.get_one::<String>("slice width") {
        state.width = width.clone();
        if state.debug {
            println!("[DEBUG] Slice width set to {}", width);
        }
    }
}