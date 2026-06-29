//! Structural validation

use crate::models::ConversionResult;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Manifest name is required")]
    MissingName,

    #[error("Manifest version is required")]
    MissingVersion,

    #[error("Only Manifest V3 is supported, got V{0}")]
    UnsupportedManifestVersion(u8),

    #[error("browser_specific_settings.gecko.id is required for Firefox")]
    MissingGeckoId,

    #[error("Validation failed: {0}")]
    Other(String),
}

pub fn validate_structure(result: &ConversionResult) -> Result<(), ValidationError> {
    // Validate manifest
    validate_manifest(&result.manifest)?;

    // Validate files exist
    validate_files(result)?;

    Ok(())
}

fn validate_manifest(manifest: &crate::models::Manifest) -> Result<(), ValidationError> {
    // Check required fields
    if manifest.name.is_empty() {
        return Err(ValidationError::MissingName);
    }

    if manifest.version.is_empty() {
        return Err(ValidationError::MissingVersion);
    }

    if manifest.manifest_version != 3 {
        return Err(ValidationError::UnsupportedManifestVersion(
            manifest.manifest_version,
        ));
    }

    // Check Firefox-specific requirements
    if manifest.browser_specific_settings.is_none() {
        return Err(ValidationError::MissingGeckoId);
    }

    Ok(())
}

fn validate_files(_result: &ConversionResult) -> Result<(), ValidationError> {
    // TODO(#42): Validate that referenced files exist
    Ok(())
}
