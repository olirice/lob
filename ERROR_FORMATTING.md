# Error Formatting Guide

## Color Scheme (Light & Dark Mode Compatible)

The error formatting uses colors that provide good contrast in both light and dark terminal themes:

### Color Choices

| Element | Color | Reason |
|---------|-------|--------|
| **Header** (`✗ Compilation Error`) | Red, Bold | Universally indicates errors |
| **Expression Label** (`Your expression:`) | Cyan, Bold | Clear contrast in both modes |
| **Expression Value** | Yellow | High visibility in both themes |
| **Error Keywords** (`error:`, `error[E0599]:`) | Red, Bold | Standard error color |
| **Warning Keywords** | Yellow, Bold | Standard warning color |
| **File Locations** (`--> file:line:col`) | Cyan | Distinguishes metadata |
| **Code Snippets** | Default terminal color | Readable in any theme |
| **Annotations** (pipes, dashes) | Cyan | Subtle but visible |
| **Error Carets** (`^^^`) | Red, Bold | Points to exact error location |
| **Help Messages** (`= help:`) | Blue | Informational, not alarming |
| **Notes** (`= note:`) | Cyan | Secondary information |
| **Tips** (`💡 Tip:`) | Blue | Friendly suggestions |

### Design Principles

1. **High Contrast**: Colors chosen for visibility on both white and black backgrounds
2. **Semantic Meaning**: Colors convey meaning (red=error, yellow=warning, blue=info)
3. **No Extreme Colors**: Avoiding `bright_white` and `bright_black` which fail in certain themes
4. **Standard Terminal Colors**: Using the basic 8 ANSI colors that terminals handle well

### Examples

#### Syntax Error

```
✗ Compilation Error

  Your expression: _.map(|x| x.parse::<i32>().unwrap( ).take(200)

  error: mismatched closing delimiter: `}`
  --> 9f7f0ad4...122.rs:5:32
    |
  5 |     let result = stdin_data.map(|x| x.parse::<i32>().unwrap( ).take(200);
    |                                ^ unclosed delimiter

  error: aborting due to 1 previous error


💡 Tip: Check your expression syntax and ensure all parentheses match
```

#### Method Not Found Error

```
✗ Compilation Error

  Your expression: _.map(|x| x.foo())

  error[E0599]: no method named `foo` found for struct `String` in the current scope
  --> 949711828...f5.rs:5:39
    |
  5 |     let result = stdin_data.map(|x| x.foo());
    |                                       ^^^ method not found in `String`

  error: aborting due to 1 previous error

  For more information about this error, try `rustc --explain E0599`.

💡 Tip: Check your expression syntax and ensure all parentheses match
```

## Testing in Different Themes

The color scheme has been designed to work well in:
- **Dark themes**: iTerm2 Dark, VS Code Dark+, Solarized Dark
- **Light themes**: iTerm2 Light, VS Code Light+, Solarized Light
- **High contrast modes**: For accessibility

## Implementation

The error formatting is implemented in `crates/flu-cli/src/error.rs` using the `colored` crate:

```rust
// Header
output.push(format!("{}", "✗ Compilation Error".red().bold()));

// User expression
output.push(format!(
    "  {} {}",
    "Your expression:".cyan().bold(),
    expr.yellow()
));

// Error carets
output.push(format!("  {}", line.red().bold()));

// Help messages
output.push(format!("  {}", line.blue()));

// Tips
output.push(format!(
    "{}",
    "💡 Tip: Check your expression syntax...".blue()
));
```

## Future Enhancements

Possible improvements:
- Add `NO_COLOR` environment variable support for CI/testing
- Smart detection of terminal color support
- Optional JSON error output for tool integration
- Line/column highlighting in user expression
- Suggestion system for common mistakes
