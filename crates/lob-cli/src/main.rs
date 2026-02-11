//! Lob - Embedded Rust Pipeline Tool
//!
//! A self-contained CLI for running Rust data pipeline one-liners.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod cache;
mod codegen;
mod compile;
mod error;
mod toolchain;

use cache::Cache;
use clap::Parser;
use codegen::CodeGenerator;
use compile::Compiler;
use error::{LobError, Result};
use std::process::Command;
use toolchain::EmbeddedToolchain;

/// Lob - Embedded Rust Pipeline Tool
#[derive(Parser, Debug)]
#[command(name = "lob")]
#[command(about = "Run Rust data pipeline one-liners", long_about = None)]
#[command(version)]
struct Args {
    /// Lob expression to execute
    #[arg(value_name = "EXPRESSION")]
    expression: Option<String>,

    /// Show generated source code without executing
    #[arg(short = 's', long)]
    show_source: bool,

    /// Clear the compilation cache
    #[arg(long)]
    clear_cache: bool,

    /// Show cache statistics
    #[arg(long)]
    cache_stats: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    if let Err(e) = run() {
        // Compilation errors are already formatted nicely
        match &e {
            LobError::Compilation(msg) => eprintln!("{}", msg),
            _ => eprintln!("Error: {}", e),
        }
        std::process::exit(1);
    }
}

/// Initialize the compiler, trying embedded toolchain first, then system rustc
fn initialize_compiler(verbose: bool) -> Result<Compiler> {
    // Try embedded toolchain first
    match EmbeddedToolchain::ensure_extracted() {
        Ok(toolchain) => {
            if toolchain.is_valid() {
                if verbose {
                    eprintln!("Using embedded Rust toolchain");
                }
                return Ok(Compiler::custom(
                    toolchain.rustc_path(),
                    Some(toolchain.sysroot()),
                ));
            } else if verbose {
                eprintln!("Embedded toolchain invalid, falling back to system rustc");
            }
        }
        Err(e) => {
            if verbose {
                eprintln!("Embedded toolchain not available: {}", e);
                eprintln!("Falling back to system rustc");
            }
        }
    }

    // Fall back to system rustc
    Compiler::system()
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Handle cache management commands
    if args.clear_cache {
        let cache = Cache::new()?;
        cache.clear()?;
        println!("Cache cleared successfully");
        return Ok(());
    }

    if args.cache_stats {
        let cache = Cache::new()?;
        let stats = cache.stats()?;
        println!("Cache statistics:");
        println!("  Cached binaries: {}", stats.binary_count);
        println!("  Total size: {}", stats.format_size());
        println!("  Cache directory: {:?}", cache.cache_dir());
        return Ok(());
    }

    // Get expression or show help
    let expression = args.expression.ok_or_else(|| {
        LobError::InvalidExpression("No expression provided. Use --help for usage.".to_string())
    })?;

    // Generate code
    let expression_clone = expression.clone();
    let generator = CodeGenerator::new(expression);
    let source = generator.generate()?;

    if args.show_source {
        println!("{}", source);
        return Ok(());
    }

    // Initialize cache and compiler
    let cache = Cache::new()?;
    let compiler = initialize_compiler(args.verbose)?;

    // Compile (with caching)
    if args.verbose {
        eprintln!("Compiling expression...");
    }

    let binary_path = compiler.compile_and_cache(&source, &cache, Some(&expression_clone))?;

    if args.verbose {
        eprintln!("Compiled binary: {:?}", binary_path);
        eprintln!("Executing...");
    }

    // Execute the compiled binary, passing stdin through
    let mut child = Command::new(&binary_path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    let status = child.wait()?;

    if !status.success() {
        return Err(LobError::Compilation(format!(
            "Execution failed with status: {}",
            status
        )));
    }

    Ok(())
}
