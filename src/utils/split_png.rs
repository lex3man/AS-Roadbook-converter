use std::fs;

use crate::utils::debug::debug_println;

pub fn make_slices(state: &crate::State) -> Result<(), Box<dyn std::error::Error>> {
    let input_dir = std::path::Path::new(&state.output).join("pages");
    let output_dir = std::path::Path::new(&state.output);
    std::fs::create_dir_all(&output_dir)?;

    let padding: u32 = state.padding.parse().unwrap_or(114);
    let height: u32 = state.height.parse().unwrap_or(215);
    let width: u32 = state.width.parse().unwrap_or(840);

    for (index, entry) in std::fs::read_dir(input_dir)?.into_iter().enumerate() {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            let mut img = image::open(&path)?;

            let slices_per_page = img.height() / height;

            for i in 0..slices_per_page {
                let slice = img.crop(17, i * height + padding, width, height);
                let pref = if index < 10 { "00" } else if index < 100 { "0" } else { "" };
                let slice_path = output_dir.join(format!(
                    "{}{}_slice_{}.png",
                    pref,
                    index,
                    i
                ));
                debug_println(state, &format!("Saving slice to: {:?}", slice_path));
                slice.save_with_format(slice_path, image::ImageFormat::Png)?;
            }
        }
    }
    fs::remove_dir_all(std::path::Path::new(&state.output).join("pages"))?;
    Ok(())
}