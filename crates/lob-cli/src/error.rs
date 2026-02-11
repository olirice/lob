//! Error types for lob CLI

use crate::suggestion;
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

        // Show helpful suggestions for common errors
        if let Some(sug) = suggestion::get_suggestion(stderr, user_expression) {
            output.push(format!("  {}", "Problem:".red().bold()));
            output.push(format!("    {}", sug.problem));
            output.push(String::new());
            output.push(format!("  {}", "How to fix:".blue().bold()));
            for fix in sug.fixes {
                output.push(format!("    • {}", fix));
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_error_with_user_expression() {
        let stderr = "error: expected `;`";
        let formatted = LobError::format_compilation_error(stderr, Some("_.map(|x| x"));

        assert!(formatted.contains("Your expression:"));
        assert!(formatted.contains("_.map(|x| x"));
        assert!(formatted.contains("error: expected `;`"));
    }

    #[test]
    fn format_error_without_user_expression() {
        let stderr = "error: something went wrong";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(!formatted.contains("Your expression:"));
        assert!(formatted.contains("error: something went wrong"));
    }

    #[test]
    fn format_error_with_suggestion() {
        let stderr = "error: mismatched types: expected `&String`, found integer";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("Problem:"));
        assert!(formatted.contains("How to fix:"));
    }

    #[test]
    fn format_error_with_code_line() {
        let stderr = "error: test\n  12 | let x = y;";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("let x = y;"));
    }

    #[test]
    fn format_error_with_caret_line() {
        let stderr = "error: test\n       ^^^^^^";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("^^^^^^"));
    }

    #[test]
    fn format_error_with_help() {
        let stderr = "error: test\n  = help: try this instead";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("= help: try this instead"));
    }

    #[test]
    fn format_error_with_note() {
        let stderr = "error: test\n  = note: some context";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("= note: some context"));
    }

    #[test]
    fn format_error_with_aborting() {
        let stderr = "error: aborting due to previous error";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("aborting due to previous error"));
    }

    #[test]
    fn format_error_with_location() {
        let stderr = "  --> /path/to/file.rs:10:5";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("file.rs:10:5"));
        assert!(!formatted.contains("/path/to/"));
    }

    #[test]
    fn simplify_error_location_basic() {
        let line = "  --> /some/long/path/source.rs:12:5";
        let result = LobError::simplify_error_location(line);

        assert!(result.is_some());
        assert_eq!(result.unwrap(), "--> source.rs:12:5");
    }

    #[test]
    fn simplify_error_location_no_arrow() {
        let line = "some line without arrow";
        let result = LobError::simplify_error_location(line);

        assert!(result.is_none());
    }

    #[test]
    fn simplify_error_location_no_colon() {
        let line = "  --> path_without_colon";
        let result = LobError::simplify_error_location(line);

        assert!(result.is_none());
    }

    #[test]
    fn format_error_with_warning() {
        let stderr = "warning: unused variable";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("warning: unused variable"));
    }

    #[test]
    fn format_error_with_continuation_line() {
        let stderr = "error: test\n  | something";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("something"));
    }

    #[test]
    fn format_error_multiline() {
        let stderr = "error[E0308]: mismatched types\n  --> file.rs:1:1\n   |\n 1 | let x: i32 = \"string\";\n   |              ^^^^^^^^ expected `i32`, found `&str`";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("mismatched types"));
        assert!(formatted.contains("file.rs:1:1"));
        assert!(formatted.contains("let x: i32"));
    }

    #[test]
    fn format_error_blank_lines() {
        let stderr = "error: test\n\n\nanother line";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("error: test"));
        assert!(formatted.contains("another line"));
    }

    #[test]
    fn format_error_could_not_compile() {
        let stderr = "error: could not compile `project` due to 1 previous error";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("could not compile"));
    }

    #[test]
    fn format_error_location_no_simplify() {
        // Location line that can't be simplified (no colon after arrow)
        let stderr = "  --> invalid-path-format";
        let formatted = LobError::format_compilation_error(stderr, None);

        // Should still include the line even if not simplified
        assert!(formatted.contains("invalid-path-format"));
    }

    #[test]
    fn format_error_pipe_without_line_number() {
        // Pipe line without a line number
        let stderr = "error: test\n  | annotation line here";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("annotation line here"));
    }

    #[test]
    fn format_error_pipe_line_stripped() {
        // Test stripped pipe format
        let stderr = "error: test\n |more content";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("more content"));
    }

    #[test]
    fn format_error_numbered_code_line() {
        // Code line with valid line number
        let stderr = "error: test\n   12| let x = y;";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("let x = y"));
    }

    #[test]
    fn format_error_pipe_only() {
        // Just a pipe line without number
        let stderr = "error: test\n   |";
        let formatted = LobError::format_compilation_error(stderr, None);

        // Should be handled without panic
        assert!(formatted.contains("Compilation Error"));
    }

    #[test]
    fn format_error_aborting_due_to() {
        // Full "aborting due to" message
        let stderr = "error: aborting due to 2 previous errors";
        let formatted = LobError::format_compilation_error(stderr, None);

        assert!(formatted.contains("aborting due to"));
    }
}
