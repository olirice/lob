# CLI Usage

Complete reference for lob command-line interface.

## Basic Syntax

```bash
lob [OPTIONS] <EXPRESSION>
```

## Arguments

### `<EXPRESSION>`

The lob expression to execute. Must be quoted to prevent shell interpretation.

```bash
lob '_.filter(|x| x.len() > 5)'
```

## Options

### `--show-source` / `-s`

Display the generated Rust source code without executing it.

```bash
lob --show-source '_.take(3)'
```

Output:
```rust
use lob_prelude::*;

fn main() {
    let stdin_data = input();
    let result = stdin_data.take(3);
    for item in result {
        println!("{:?}", item);
    }
}
```

### `--cache-stats`

Show cache statistics including number of cached binaries and total size.

```bash
lob --cache-stats
```

Output:
```
Cache statistics:
  Cached binaries: 5
  Total size: 2.34 MB
  Cache directory: "/Users/you/.cache/lob"
```

### `--clear-cache`

Clear all cached compiled binaries.

```bash
lob --clear-cache
```

### `--verbose` / `-v`

Enable verbose output showing compilation and execution details.

```bash
lob -v '_.take(3)'
```

### `--version`

Display version information.

```bash
lob --version
```

### `--help` / `-h`

Display help information.

```bash
lob --help
```

## Input/Output

### Standard Input

lob reads from stdin by default when using `_`:

```bash
cat file.txt | lob '_.take(10)'
seq 1 100 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0)'
```

### Standard Output

Results are printed to stdout:

```bash
# Redirect output to file
cat input.txt | lob '_.filter(|x| x.contains("ERROR"))' > errors.txt

# Pipe to another command
seq 1 100 | lob '_.take(10)' | wc -l
```

### Working Without stdin

Use `lob()` helper for in-memory data:

```bash
lob 'lob(vec![1, 2, 3]).map(|x| x * 2).to_list()'
```

## Cache Location

Cache location varies by platform:

- **Linux**: `~/.cache/lob/`
- **macOS**: `~/Library/Caches/lob/`
- **Windows**: `%LOCALAPPDATA%\lob\cache\`

Cache structure:
```
lob/
 binaries/          # Compiled executables
‚    a3f2e1b...    # Hash of source code
‚    b7c4d9a...
 sources/           # Generated source (for debugging)
     a3f2e1b.rs
     b7c4d9a.rs
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error (compilation, execution, etc.) |

## Environment

lob respects these environment variables:

- `CARGO_MANIFEST_DIR` - Used to locate lob libraries during compilation

## Examples

### Development Workflow

```bash
# Test with verbose output
lob -v '_.take(3)' < input.txt

# Check generated code
lob --show-source '_.filter(|x| x.len() > 10)'

# Clear cache before testing
lob --clear-cache

# Check cache growth
lob --cache-stats
```

### Production Use

```bash
# Process logs in production
tail -f /var/log/app.log | lob '_.filter(|x| x.contains("ERROR"))'

# One-liner data processing
cat data.csv | lob '_.skip(1).filter(|x| !x.is_empty()).count()'
```

## Troubleshooting

### Compilation Errors

Use `--show-source` to debug:

```bash
lob --show-source '_.map(|x| x.invalid_method())'
```

### Performance Issues

Clear cache if binaries are stale:

```bash
lob --clear-cache
```

Check cache size:

```bash
lob --cache-stats
```

### Shell Quoting

Always quote expressions to prevent shell interpretation:

```bash
# Wrong - shell interprets |
lob _.filter(|x| x > 5)

# Correct - quoted
lob '_.filter(|x| x > 5)'
```
