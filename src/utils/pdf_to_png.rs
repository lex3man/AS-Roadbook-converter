use pdf2image::{DPI, PDF, RenderOptionsBuilder};
use std::fs;
use std::path::Path;

use crate::utils::debug::debug_println;

pub fn convert_to_png(state: &crate::State) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&state.output).join("pages");
    let pdf_path = &state.source;
    fs::create_dir_all(&output_dir)?;

    let pdf = PDF::from_file(&pdf_path)?;
    let pages = pdf.page_count();

    debug_println(state, &format!("Loaded PDF: {} with {} pages", pdf_path, pages));

    let mut render_options = RenderOptionsBuilder::default().build()?;
    render_options.resolution = DPI::Uniform(150);
    let pages = pdf.render(pdf2image::Pages::All, render_options)?;
    for (i, page) in pages.iter().enumerate() {
        let path = output_dir.join(format!("page_{}.png", i));
        debug_println(state, &format!("Saving page {} to: {:?}", i, path));
        page.save_with_format(path, image::ImageFormat::Png)?;
    }
    Ok(())
}
