use image::{DynamicImage, ImageFormat, ImageReader};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rayon::prelude::*;
use std::io::Cursor;
use std::{
  fs,
  path::{Path, PathBuf},
};
use webp::Encoder;

/// Encodes a `DynamicImage` to WebP format with a specified quality.
fn encode_webp(image: &DynamicImage, quality: f32) -> Result<Vec<u8>> {
  let encoded = Encoder::from_image(image)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .encode(quality)
    .to_vec();
  Ok(encoded)
}

/// Converts an image to WebP format and saves it to the output directory.
fn convert_image(
  input_path: &Path,
  output_dir: Option<&Path>,
  keep_original_ext: bool,
  quality: f32,
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
  let image_data = encode_webp(&image, quality)?;

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
pub fn process_directory(
  input_dir: String,
  output_dir: Option<String>,
  quality: Option<u8>,
) -> Result<()> {
  process_directory_internal(
    &input_dir,
    output_dir.as_deref(),
    false,
    quality.unwrap_or(100) as f32,
  )
}

/// Processes all images in a directory recursively, converting them to WebP format and optionally keeping original file names.
#[napi]
pub fn process_directory_destructive(
  input_dir: String,
  keep_original_names: Option<bool>,
  quality: Option<u8>,
) -> Result<()> {
  process_directory_internal(
    &input_dir,
    None,
    keep_original_names.unwrap_or(false),
    quality.unwrap_or(100) as f32,
  )
}

/// Internal function to handle both destructive and non-destructive processing
fn process_directory_internal(
  input_dir: &str,
  output_dir: Option<&str>,
  keep_original_ext: bool,
  quality: f32,
) -> Result<()> {
  let input_path = Path::new(input_dir);
  let output_path = output_dir.map(Path::new);

  // Check if the directory exists, if not, simply return Ok
  if !input_path.exists() || !input_path.is_dir() {
    return Ok(());
  }

  let entries: Vec<PathBuf> = fs::read_dir(input_path)
    .map_err(|e| Error::from_reason(e.to_string()))?
    .filter_map(|entry| entry.ok())
    .map(|entry| entry.path())
    .collect();

  entries.par_iter().try_for_each(|entry| {
    if entry.is_file() {
      if is_supported(entry) {
        convert_image(entry, output_path, keep_original_ext, quality)?;
      } else {
        //println!("Skipping unsupported file: {}", entry.display());
      }
    } else if entry.is_dir() {
      process_directory_internal(
        &entry.to_string_lossy(),
        output_path.map(|p| p.to_str().unwrap()),
        keep_original_ext,
        quality,
      )?;
    }
    Ok(())
  })
}

/// Determines if an image format is supported.
fn is_supported(path: &Path) -> bool {
  path
    .extension()
    .and_then(|ext| ext.to_str())
    .map(|ext| is_supported_extension(ext))
    .unwrap_or(false)
}

/// Checks if the given extension is supported.
fn is_supported_extension(ext: &str) -> bool {
  matches!(
    ext.to_lowercase().as_str(),
    "jpg" | "jpeg" | "png" | "bmp" | "tiff"
  )
}

/// Options for converting images to WebP format.
#[derive(Debug, Clone, Copy)]
pub struct WebpOptions {
  pub quality: Option<u8>,                 // Quality of the WebP image (0-100)
  pub dimensions: Option<(u32, u32)>,      // Desired dimensions (width, height)
  pub maintain_aspect_ratio: Option<bool>, // Whether to maintain the aspect ratio
}

impl FromNapiValue for WebpOptions {
  unsafe fn from_napi_value(
    env: napi::sys::napi_env,
    napi_val: napi::sys::napi_value,
  ) -> Result<Self> {
    let obj = Object::from_napi_value(env, napi_val)?;
    Ok(WebpOptions {
      quality: obj.get("quality")?,
      dimensions: obj.get("dimensions")?,
      maintain_aspect_ratio: obj.get("maintain_aspect_ratio").unwrap_or(Some(false)),
    })
  }
}

/// Converts a buffer containing image data to WebP format with options.
#[napi]
pub fn convert_to_webp(buffer: Buffer, extension: String, options: WebpOptions) -> Result<Buffer> {
  // Determine the image format from the extension
  let format = match extension.to_lowercase().as_str() {
    "jpg" | "jpeg" => ImageFormat::Jpeg,
    "png" => ImageFormat::Png,
    "bmp" => ImageFormat::Bmp,
    "tiff" => ImageFormat::Tiff,
    _ => return Err(Error::from_reason("Unsupported image format")),
  };

  // Create an image reader from the buffer
  let image_reader = ImageReader::with_format(Cursor::new(buffer), format);

  // Decode the image
  let mut image = image_reader
    .decode()
    .map_err(|e| Error::from_reason(e.to_string()))?;

  // Resize the image if dimensions are provided
  if let Some((width, height)) = options.dimensions {
    if options.maintain_aspect_ratio.unwrap_or(false) {
      image = image.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3);
    } else {
      image = image.resize_exact(width, height, image::imageops::FilterType::Lanczos3);
    }
  }

  // Encode the image to WebP format with the specified quality
  let quality = options.quality.unwrap_or(100) as f32;
  let webp_data = encode_webp(&image, quality)?;

  // Return the WebP data as a Buffer
  Ok(Buffer::from(webp_data))
}
