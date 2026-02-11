//! Error types for lob CLI

use colored::Colorize;
use thiserror::Error;

/// Errors that can occur during lob execution
#[derive(Error, Debug)]
pub enum LobError {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Compilation error
    #[error("Compilation failed:\n{0}")]
    Compilation(String),

    /// Cache error
    #[error("Cache error: {0}")]
    Cache(String),

    /// Code generation error
    #[allow(dead_code)]
    #[error("Code generation error: {0}")]
    CodeGen(String),

    /// Toolchain error
    #[error("Toolchain error: {0}")]
    Toolchain(String),

    /// Invalid expression
    #[error("Invalid expression: {0}")]
    InvalidExpression(String),
}

/// Result type for lob operations
pub type Result<T> = std::result::Result<T, LobError>;

impl LobError {
    /// Format a compilation error with colors and context
    pub fn format_compilation_error(stderr: &str, user_expression: Option<&str>) -> String {
        let mut output = Vec::new();

        // Header
        output.push(format!("{}", "✗ Compilation Error".red().bold()));
        output.push(String::new());

        // Show user's expression if provided
        if let Some(expr) = user_expression {
            output.push(format!(
                "  {} {}",
                "Your expression:".cyan().bold(),
                expr.yellow()
            ));
            output.push(String::new());
        }

        // Parse rustc error output
        let lines: Vec<&str> = stderr.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i];

            // Error/warning headers
            if line.starts_with("error:") || line.starts_with("error[") {
                output.push(format!("  {}", line.red().bold()));
            } else if line.starts_with("warning:") || line.starts_with("warning[") {
                output.push(format!("  {}", line.yellow().bold()));
            }
            // File location lines (e.g., "--> path:line:col")
            else if line.trim_start().starts_with("-->") {
                // Extract and simplify the path
                if let Some(simplified) = Self::simplify_error_location(line) {
                    output.push(format!("  {}", simplified.cyan()));
                } else {
                    output.push(format!("  {}", line.cyan()));
                }
            }
            // Code lines with line numbers
            else if let Some(stripped) = line.trim_start().strip_prefix('|') {
                let trimmed = line.trim_start();
                // Check if it's a line number
                if let Some(num_end) = trimmed.find('|') {
                    let num_part = &trimmed[..num_end];
                    if num_part.trim().parse::<usize>().is_ok() {
                        // Code line - show as-is (neutral)
                        output.push(format!("  {}", line));
                    } else {
                        // Continuation or annotation line
                        output.push(format!("  {}", line.cyan()));
                    }
                } else {
                    output.push(format!("  {}", stripped));
                }
            }
            // Highlight/caret lines (^, ^^^, etc.)
            else if line
                .trim_start()
                .chars()
                .all(|c| c == '^' || c == ' ' || c == '|')
                && line.contains('^')
            {
                output.push(format!("  {}", line.red().bold()));
            }
            // Help/note lines
            else if line.trim_start().starts_with("= help:") {
                output.push(format!("  {}", line.blue()));
            } else if line.trim_start().starts_with("= note:") {
                output.push(format!("  {}", line.cyan()));
            }
            // Summary lines
            else if line.starts_with("error: aborting")
                || line.starts_with("error: could not compile")
            {
                output.push(String::new());
                output.push(format!("  {}", line.red()));
            }
            // Other lines (blank or context)
            else if !line.trim().is_empty() {
                output.push(format!("  {}", line));
            } else {
                output.push(String::new());
            }

            i += 1;
        }

        output.push(String::new());
        output.push(format!(
            "{}",
            "Tip: Check your expression syntax and ensure all parentheses match".blue()
        ));

        output.join("\n")
    }

    /// Simplify error location by removing cache path
    fn simplify_error_location(line: &str) -> Option<String> {
        // Try to extract just the filename from the full path
        if let Some(arrow_pos) = line.find("-->") {
            let rest = &line[arrow_pos + 3..].trim();
            if let Some(colon_pos) = rest.find(':') {
                let path = &rest[..colon_pos];
                if let Some(filename) = path.rsplit('/').next() {
                    let location = &rest[colon_pos..];
                    return Some(format!("--> {}{}", filename, location));
                }
            }
        }
        None
    }
}
