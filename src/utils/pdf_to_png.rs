use pdf2image::{DPI, PDF, RenderOptionsBuilder};
use std::fs;
use std::path::Path;

pub fn convert_to_png(state: &crate::State) -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = Path::new(&state.output).join("pages");
    let pdf_path = &state.source;
    fs::create_dir_all(&output_dir)?;

    let pdf = PDF::from_file(&pdf_path)?;
    let pages = pdf.page_count();

    if state.debug {
        println!("Debug: PDF loaded successfully with {} pages", pages);
    }

    let mut render_options = RenderOptionsBuilder::default().build()?;
    render_options.resolution = DPI::Uniform(150);
    let pages = pdf.render(pdf2image::Pages::All, render_options)?;
    for (i, page) in pages.iter().enumerate() {
        let path = output_dir.join(format!("page_{}.png", i));
        if state.debug {
            println!("Debug: Saving page {} to {}", i, path.display());
        }
        page.save_with_format(path, image::ImageFormat::Png)?;
    }
    Ok(())
}
