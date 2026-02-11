//! Output format handling

use std::io::{stdout, IsTerminal};

/// Output format for results
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Rust debug format (current default)
    Debug,
    /// JSON array
    Json,
    /// JSON lines (newline-delimited)
    JsonLines,
    /// CSV (requires CSV input)
    Csv,
}

impl OutputFormat {
    /// Parse from string
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "debug" => Some(Self::Debug),
            "json" => Some(Self::Json),
            "jsonl" | "jsonlines" => Some(Self::JsonLines),
            "csv" => Some(Self::Csv),
            _ => None,
        }
    }

    /// Get default format based on context
    pub fn default(is_terminal: bool) -> Self {
        if is_terminal {
            Self::Debug
        } else {
            Self::JsonLines
        }
    }

    /// Get the format name for code generation
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Debug => "debug",
            Self::Json => "json",
            Self::JsonLines => "jsonlines",
            Self::Csv => "csv",
        }
    }
}

/// Detect if stdout is a terminal
pub fn is_terminal() -> bool {
    stdout().is_terminal()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_from_str() {
        assert_eq!(OutputFormat::from_str("debug"), Some(OutputFormat::Debug));
        assert_eq!(OutputFormat::from_str("json"), Some(OutputFormat::Json));
        assert_eq!(
            OutputFormat::from_str("jsonl"),
            Some(OutputFormat::JsonLines)
        );
        assert_eq!(
            OutputFormat::from_str("jsonlines"),
            Some(OutputFormat::JsonLines)
        );
        assert_eq!(OutputFormat::from_str("csv"), Some(OutputFormat::Csv));
        assert_eq!(OutputFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_output_format_as_str() {
        assert_eq!(OutputFormat::Debug.as_str(), "debug");
        assert_eq!(OutputFormat::Json.as_str(), "json");
        assert_eq!(OutputFormat::JsonLines.as_str(), "jsonlines");
        assert_eq!(OutputFormat::Csv.as_str(), "csv");
    }

    #[test]
    fn test_output_format_default() {
        assert_eq!(OutputFormat::default(true), OutputFormat::Debug);
        assert_eq!(OutputFormat::default(false), OutputFormat::JsonLines);
    }
}
