# lob

**Embedded Rust Pipeline Tool** - A CLI for running Rust data pipeline one-liners with native performance.

[![CI](https://github.com/olirice/lob/workflows/CI/badge.svg)](https://github.com/olirice/lob/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

> **Inspired by [flupy](https://github.com/olirice/flupy)** - Python's fluent pipeline library

## Features

- **Native Rust Performance** - Compiled pipelines run at native speed
- **Smart Caching** - Compiled expressions cached for instant re-execution
- **Lazy Evaluation** - Memory-efficient streaming operations
- **Fluent API** - Chainable operations for readable data transformations
- **Rich Operations** - Filter, map, group, join, and 20+ operations

## Quick Start

### Installation

**Prerequisites:** Rust toolchain

```bash
cargo install --path crates/lob-cli
```

Or build from source:

```bash
cargo build --release
cp target/release/lob ~/.local/bin/  # or /usr/local/bin/
```

### Basic Usage

```bash
# From stdin
seq 1 100 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0).count()'
# Output: 50

# From file
lob data.txt '_.filter(|x| x.len() > 5).take(10)'
# Output: First 10 lines longer than 5 characters

# Multiple files
lob file1.txt file2.txt '_.unique().count()'
# Output: Number of unique lines across all files

# Parse CSV
lob users.csv --parse-csv '_.filter(|r| r["age"].parse::<i32>().unwrap() > 18)'
# Output: CSV rows where age > 18
```

## Examples

### Log Processing

```bash
# Extract error messages from file
lob app.log '_.filter(|x| x.contains("ERROR")).take(10)'
# Output: First 10 ERROR lines

# Count errors by type
cat app.log | lob '
  _.filter(|x| x.contains("ERROR"))
   .map(|x| x.split(":").nth(1).unwrap())
   .group_by(|x| x.clone())
   .map(|(k, v)| format!("{}: {}", k, v.len()))
'
```

### CSV Processing

```bash
# Filter CSV by column value
lob users.csv --parse-csv '_.filter(|r| r["status"] == "active")'
# Output: All active users

# Convert CSV to JSON
lob data.csv --parse-csv '_.take(100)' --format json > output.json

# Group by column and count
lob sales.csv --parse-csv '
  _.group_by(|r| r["category"].clone())
   .map(|(k, v)| format!("{}: {} items", k, v.len()))
'
# Output:
# "Electronics: 45 items"
# "Clothing: 32 items"
```

### Data Transformation

```bash
# Unique lines
cat data.txt | lob '_.unique()'
# Output: (unique lines from data.txt)

# Chunk into groups of 5
seq 1 20 | lob '_.chunk(5).map(|chunk| chunk.len()).sum::<usize>()'
# Output: 20

# Window analysis (sliding window of 3)
seq 1 10 | lob '_.window(3).take(3)'
# Output:
# ["1", "2", "3"]
# ["2", "3", "4"]
# ["3", "4", "5"]
```

## How It Works

1. **Generate** - Your expression is converted to a complete Rust program
2. **Compile** - The program is compiled with full optimizations (`-C opt-level=3`)
3. **Cache** - Compiled binary is cached (SHA256-based) for instant reuse
4. **Execute** - Native binary processes your data at full speed

### Performance

```bash
# First run: ~1-2s (compilation)
seq 1 1000000 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0).count()'
# Output: 500000

# Subsequent runs: <10ms (cached)
seq 1 1000000 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0).count()'
# Output: 500000 (instant)
```

**Result**: ~14 million items/second throughput

## Operations

### Selection
- `filter(predicate)` - Keep items matching condition
- `take(n)` - Take first n items
- `skip(n)` - Skip first n items
- `take_while(predicate)` - Take while condition holds
- `drop_while(predicate)` - Skip while condition holds
- `unique()` - Remove duplicates

### Transformation
- `map(f)` - Transform each item
- `enumerate()` - Add indices
- `zip(other)` - Pair with another iterator
- `flatten()` - Flatten nested iterators

### Grouping
- `chunk(n)` - Group into chunks of size n
- `window(n)` - Sliding window of size n
- `group_by(key_fn)` - Group by key function

### Joins
- `join_inner(other, left_key, right_key)` - Inner join
- `join_left(other, left_key, right_key)` - Left join

### Terminal
- `collect()` - Collect to vector
- `to_list()` - Collect to vector
- `count()` - Count items
- `sum()` - Sum items
- `min()` / `max()` - Find extrema
- `first()` / `last()` - Get first/last
- `reduce(f)` - Reduce with function
- `fold(init, f)` - Fold with initial value

## Input Formats

### CSV Parsing

```bash
# Parse CSV with headers (each row becomes HashMap<String, String>)
lob data.csv --parse-csv '_.filter(|r| r["age"].parse::<i32>().unwrap() > 18)'

# Access columns by name
lob users.csv --parse-csv '_.map(|r| format!("{}: {}", r["name"], r["email"]))'
```

### TSV Parsing

```bash
# Parse tab-separated values
lob data.tsv --parse-tsv '_.filter(|r| r["status"] == "active")'
```

### JSON Lines

```bash
# Parse newline-delimited JSON
lob logs.jsonl --parse-json '_.filter(|obj| obj["level"] == "ERROR")'
```

## Output Formats

### JSON Output

```bash
# Pretty JSON array
lob data.csv --parse-csv '_.take(5)' --format json
# Output:
# [
#   {"name": "Alice", "age": "30"},
#   {"name": "Bob", "age": "25"}
# ]
```

### JSON Lines

```bash
# Newline-delimited JSON (one per line)
lob data.csv --parse-csv '_.filter(...)' --format jsonl | jq '.name'
# Output:
# {"name":"Alice","age":"30"}
# {"name":"Bob","age":"25"}
```

### CSV Output

```bash
# Output as CSV
lob data.csv --parse-csv '_.filter(|r| r["age"].parse::<i32>().unwrap() > 25)' --format csv
```

### Table Output

```bash
# Display as formatted table (requires CSV/JSON input)
lob users.csv --parse-csv '_.take(5)' --format table
# Output:
# ╭─────┬─────────────────────┬─────────╮
# │ age │ email               │ name    │
# ├─────┼─────────────────────┼─────────┤
# │ 30  │ alice@example.com   │ Alice   │
# │ 25  │ bob@example.com     │ Bob     │
# │ 35  │ charlie@example.com │ Charlie │
# ╰─────┴─────────────────────┴─────────╯
```

## Advanced Features

### Cache Management

```bash
# Show cache stats
lob --cache-stats

# Clear cache
lob --clear-cache
```

### Performance Statistics

```bash
# Show live execution statistics (opt-in)
seq 1 100000 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0).count()' --stats
# Output:
# [Stats] Items: 10000 | Throughput: 12832160 items/s | Elapsed: 0.0s
# [Stats] Items: 20000 | Throughput: 12570047 items/s | Elapsed: 0.0s
# ...
# [Final Stats] Total items: 100000 | Throughput: 6990441 items/s | Total time: 0.014s
# 50000
#
# Statistics:
#   Compilation time: 360.37ms
#   Execution time:   260.73ms
#   Total time:       621.10ms
#   Cache:            Miss (compiled)
```

### Debug Mode

```bash
# Show generated source
lob --show-source '_.take(5)'

# Show generated source for CSV input
lob --show-source --parse-csv '_.filter(|r| r["age"] > "25")'

# Verbose output
lob -v '_.take(5)'
```

## Architecture

```
Input Data � lob Expression � Generated Rust Code � Compiled Binary (cached) � Output
```

**Caching Strategy:**
- Expression hashed with SHA256
- Compiled binary stored in `~/.cache/lob/binaries/`
- Source saved in `~/.cache/lob/sources/` for debugging

## Development

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test --all
```

### Pre-commit Hooks

```bash
git config core.hooksPath .githooks
```

## Documentation

- [Performance Analysis](PERFORMANCE.md) - Detailed performance characteristics
- [Lazy Evaluation](LAZY_EVALUATION.md) - How lazy evaluation works
- [Error Formatting](ERROR_FORMATTING.md) - Pretty error messages
- [Embedded Toolchain](EMBEDDED_TOOLCHAIN_README.md) - Self-contained binaries

## Inspiration

lob is inspired by [flupy](https://github.com/olirice/flupy), bringing Python's fluent pipeline style to Rust with native performance.

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.

