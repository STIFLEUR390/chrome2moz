//! Package extraction and building

pub mod extractor;
pub mod builder;

use crate::models::{Extension, ConversionResult};
use anyhow::Result;
use std::path::Path;

/// Load extension from file or directory
pub fn load_extension(path: &Path) -> Result<Extension> {
    if path.is_dir() {
        extractor::load_from_directory(path)
    } else if path.extension().and_then(|e| e.to_str()) == Some("zip") 
        || path.extension().and_then(|e| e.to_str()) == Some("crx") {
        extractor::load_from_archive(path)
    } else {
        anyhow::bail!("Unsupported input format. Expected directory, .zip, or .crx file")
    }
}

/// Build Firefox extension package with all files
fn derive_xpi_path(output_path: &Path) -> std::path::PathBuf {
    let stem = output_path
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "extension".to_string());
    output_path.with_file_name(format!("{}.xpi", stem))
}

pub fn build_complete_extension(
    source: &Extension,
    result: &ConversionResult,
    output_path: &Path
) -> Result<()> {
    builder::build_complete_directory(source, result, output_path)?;
    
    // Create XPI from directory
    let xpi_path = derive_xpi_path(output_path);
    builder::create_zip_from_directory(output_path, &xpi_path)?;
    
    Ok(())
}

/// Build Firefox extension package (simple version)
pub fn build_extension(result: &ConversionResult, output_path: &Path) -> Result<()> {
    builder::build_xpi(result, output_path)
}