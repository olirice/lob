//! Input source handling (files and stdin)

use crate::error::{LobError, Result};
use std::path::PathBuf;

/// Input format for parsing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputFormat {
    /// Plain text lines
    Lines,
    /// CSV with headers
    Csv,
    /// TSV with headers
    Tsv,
    /// JSON lines (one JSON object per line)
    JsonLines,
}

impl InputFormat {
    /// Get the format name for code generation
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lines => "lines",
            Self::Csv => "csv",
            Self::Tsv => "tsv",
            Self::JsonLines => "jsonlines",
        }
    }
}

/// Input source configuration
#[derive(Debug, Clone)]
pub struct InputSource {
    /// Files to read (empty = stdin)
    pub files: Vec<PathBuf>,
    /// Input format
    pub format: InputFormat,
}

impl InputSource {
    /// Create new input source from files
    pub fn new(files: Vec<PathBuf>, format: InputFormat) -> Self {
        Self { files, format }
    }

    /// Check if reading from stdin
    pub fn is_stdin(&self) -> bool {
        self.files.is_empty()
    }

    /// Validate that files exist
    pub fn validate(&self) -> Result<()> {
        for file in &self.files {
            if !file.exists() {
                return Err(LobError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("File not found: {}", file.display()),
                )));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_format_as_str() {
        assert_eq!(InputFormat::Lines.as_str(), "lines");
        assert_eq!(InputFormat::Csv.as_str(), "csv");
        assert_eq!(InputFormat::Tsv.as_str(), "tsv");
        assert_eq!(InputFormat::JsonLines.as_str(), "jsonlines");
    }

    #[test]
    fn test_input_source_is_stdin() {
        let source = InputSource::new(vec![], InputFormat::Lines);
        assert!(source.is_stdin());

        let source = InputSource::new(vec![PathBuf::from("test.txt")], InputFormat::Lines);
        assert!(!source.is_stdin());
    }

    #[test]
    fn test_input_source_validate_missing_file() {
        let source = InputSource::new(
            vec![PathBuf::from("/nonexistent/file.txt")],
            InputFormat::Lines,
        );
        assert!(source.validate().is_err());
    }

    #[test]
    fn test_input_source_validate_empty() {
        let source = InputSource::new(vec![], InputFormat::Lines);
        assert!(source.validate().is_ok());
    }
}
