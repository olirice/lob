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
# Filter and count
seq 1 100 | lob '_.filter(|x| x.parse::<i32>().unwrap() % 2 == 0).count()'
# Output: 50

# Map and take first 5
echo -e "hello\nworld" | lob '_.map(|x| x.to_uppercase()).take(5)'
# Output:
# "HELLO"
# "WORLD"

# Parse, filter, sum
seq 1 1000 | lob '_.map(|x| x.parse::<i32>().unwrap()).filter(|&x| x > 500).sum::<i32>()'
# Output: 375250
```

## Examples

### Log Processing

```bash
# Extract error messages
cat app.log | lob '_.filter(|x| x.contains("ERROR")).take(10)'

# Count errors by type
cat app.log | lob '
  _.filter(|x| x.contains("ERROR"))
   .map(|x| x.split(":").nth(1).unwrap())
   .group_by(|x| x.clone())
   .map(|(k, v)| format!("{}: {}", k, v.len()))
'
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

## Advanced Features

### Cache Management

```bash
# Show cache stats
lob --cache-stats

# Clear cache
lob --clear-cache
```

### Debug Mode

```bash
# Show generated source
lob --show-source '_.take(5)'

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

