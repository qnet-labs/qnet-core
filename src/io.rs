use crate::api::request::QNetFile;
use crate::validation::{QNetValidator, ValidationResult};
use std::fs;
use std::io;

#[derive(Debug)]
pub enum QNetError {
    IoError(io::Error),
    ParseError(serde_json::Error),
    ValidationError(ValidationResult),
    VersionNotSupported(String),
}

impl std::fmt::Display for QNetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QNetError::IoError(e) => write!(f, "IO error: {}", e),
            QNetError::ParseError(e) => write!(f, "JSON parse error: {}", e),
            QNetError::ValidationError(result) => {
                write!(f, "Validation failed:\n{}", result.generate_report())
            }
            QNetError::VersionNotSupported(v) => {
                write!(f, "Unsupported qnet version: {}", v)
            }
        }
    }
}

impl std::error::Error for QNetError {}

pub fn load_qnet_file(filepath: &str) -> Result<QNetFile, QNetError> {
     // Read file
    let json = fs::read_to_string(filepath)
          .map_err(QNetError::IoError)?;

     // Parse JSON
    let file: QNetFile = serde_json::from_str(&json)
          .map_err(QNetError::ParseError)?;

     // Validate version
    if file.version != "1.0" {
        return Err(QNetError::VersionNotSupported(file.version));
     }

     // Validate file
    let validation = QNetValidator::validate_all(&file);
    if !validation.is_valid {
        return Err(QNetError::ValidationError(validation));
     }

    Ok(file)
}

pub fn save_qnet_file(filepath: &str, file: &QNetFile) -> Result<(), QNetError> {
    let json = serde_json::to_string_pretty(file)
          .map_err(QNetError::ParseError)?;

    fs::write(filepath, json)
          .map_err(QNetError::IoError)?;

    Ok(())
}
