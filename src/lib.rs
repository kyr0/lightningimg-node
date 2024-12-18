use image::{DynamicImage, ImageReader};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rayon::prelude::*;
use std::{
  fs,
  path::{Path, PathBuf},
};
use webp::Encoder;

/// Encodes a `DynamicImage` to WebP format.
fn encode_webp(image: &DynamicImage) -> Result<Vec<u8>> {
  let encoded = Encoder::from_image(image)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .encode(100.0)
    .to_vec();
  Ok(encoded)
}

/// Converts an image to WebP format and saves it to the output directory.
fn convert_image(
  input_path: &Path,
  output_dir: Option<&Path>,
  keep_original_ext: bool,
) -> Result<()> {
  let output_path = if keep_original_ext {
    // Keep the original file name and extension
    output_dir
      .map(|dir| dir.join(input_path.file_name().unwrap()))
      .unwrap_or_else(|| input_path.to_path_buf())
  } else {
    // Convert to .webp extension
    output_dir
      .map(|dir| {
        dir
          .join(input_path.file_stem().unwrap())
          .with_extension("webp")
      })
      .unwrap_or_else(|| input_path.with_extension("webp"))
  };

  let image_reader =
    ImageReader::open(input_path).map_err(|e| Error::from_reason(e.to_string()))?;
  let image = image_reader
    .decode()
    .map_err(|e| Error::from_reason(e.to_string()))?;
  let image_data = encode_webp(&image)?;

  // If we're overwriting the original file, we should write to a temporary file first
  if output_path == input_path {
    let temp_path = input_path.with_extension("webp.tmp");
    fs::write(&temp_path, &image_data).map_err(|e| Error::from_reason(e.to_string()))?;
    fs::rename(&temp_path, &output_path).map_err(|e| Error::from_reason(e.to_string()))?;
  } else {
    fs::write(&output_path, image_data).map_err(|e| Error::from_reason(e.to_string()))?;
  }

  //println!("Generated: {}", output_path.display());
  Ok(())
}

/// Processes all images in a directory recursively, converting them to WebP format.
#[napi]
pub fn process_directory(input_dir: String, output_dir: Option<String>) -> Result<()> {
  process_directory_internal(&input_dir, output_dir.as_deref(), false)
}

/// Processes all images in a directory recursively, converting them to WebP format and optionally keeping original file names.
#[napi]
pub fn process_directory_destructive(
  input_dir: String,
  keep_original_names: Option<bool>,
) -> Result<()> {
  process_directory_internal(&input_dir, None, keep_original_names.unwrap_or(false))
}

/// Internal function to handle both destructive and non-destructive processing
fn process_directory_internal(
  input_dir: &str,
  output_dir: Option<&str>,
  keep_original_ext: bool,
) -> Result<()> {
  let input_path = Path::new(input_dir);
  let output_path = output_dir.map(Path::new);

  let entries: Vec<PathBuf> = fs::read_dir(input_path)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .collect();

  entries.par_iter().try_for_each(|entry| {
    if entry.is_file() {
      if is_supported(entry) {
        convert_image(entry, output_path, keep_original_ext)?;
      } else {
        println!("Skipping unsupported file: {}", entry.display());
      }
    } else if entry.is_dir() {
      process_directory_internal(
        &entry.to_string_lossy(),
        output_path.map(|p| p.to_str().unwrap()),
        keep_original_ext,
      )?;
    }
    Ok(())
  })
}

/// Determines if an image format is supported.
fn is_supported(path: &Path) -> bool {
  matches!(
    path.extension().and_then(|ext| ext.to_str()),
    Some("jpg" | "jpeg" | "png" | "bmp" | "tiff")
  )
}
