//! Appearance golden-capture helpers.
//!
//! The native shell can emit a single-frame screenshot during the first render
//! pass. The helper functions in this module encode the shell's `0RGB` raster
//! buffer into a deterministic PNG payload suitable for golden baselines.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Writes an `0RGB` pixel buffer (`0x00RRGGBB`) into a PNG file.
pub(crate) fn write_png_0rgb(
    path: &Path,
    width: u32,
    height: u32,
    pixels_0rgb: &[u32],
) -> Result<(), Box<dyn std::error::Error>> {
    if width == 0 || height == 0 {
        return Err("cannot encode empty screenshot".into());
    }
    let required = (width as usize).saturating_mul(height as usize);
    if pixels_0rgb.len() < required {
        return Err("pixel buffer smaller than screenshot dimensions".into());
    }

    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }

    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut encoder = png::Encoder::new(writer, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Fast);
    encoder.set_filter(png::FilterType::NoFilter);

    let mut png_writer = encoder.write_header()?;
    let mut rgba = vec![0u8; required.saturating_mul(4)];
    for (idx, rgb) in pixels_0rgb.iter().take(required).enumerate() {
        let out = idx.saturating_mul(4);
        if let Some(px) = rgba.get_mut(out..out.saturating_add(4)) {
            px[0] = ((rgb >> 16) & 0xff) as u8;
            px[1] = ((rgb >> 8) & 0xff) as u8;
            px[2] = (rgb & 0xff) as u8;
            px[3] = 0xff;
        }
    }
    png_writer.write_image_data(&rgba)?;
    Ok(())
}
