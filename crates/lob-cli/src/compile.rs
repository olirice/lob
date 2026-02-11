//! Compilation of generated Rust code

use crate::cache::Cache;
use crate::error::{LobError, Result};
use std::path::PathBuf;
use std::process::Command;

/// Compiler for lob expressions
pub struct Compiler {
    /// Path to rustc executable
    rustc_path: PathBuf,
    /// Path to sysroot (for embedded toolchain)
    sysroot: Option<PathBuf>,
}

impl Compiler {
    /// Find the target directory containing compiled lob libraries
    fn find_target_dir() -> Option<PathBuf> {
        // Strategy 1: Use CARGO_MANIFEST_DIR (works during cargo test/run)
        if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
            let workspace_root = PathBuf::from(manifest_dir)
                .parent()
                .and_then(|p| p.parent())
                .map(|p| p.to_path_buf());

            if let Some(root) = workspace_root {
                let target_dir = root.join("target").join("debug");
                if target_dir.exists() {
                    return Some(target_dir);
                }
            }
        }

        // Strategy 2: Look relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                if let Some(target_parent) = exe_dir.parent() {
                    // Try debug first (avoids LTO linking issues)
                    let debug_dir = target_parent.join("debug");
                    let prelude_lib = debug_dir.join("liblob_prelude.rlib");
                    if prelude_lib.exists() {
                        return Some(debug_dir);
                    }

                    // Fall back to release
                    let release_dir = target_parent.join("release");
                    let prelude_lib = release_dir.join("liblob_prelude.rlib");
                    if prelude_lib.exists() {
                        return Some(release_dir);
                    }
                }

                // Try same directory as executable
                let target_dir = exe_dir.to_path_buf();
                let prelude_lib = target_dir.join("liblob_prelude.rlib");
                if prelude_lib.exists() {
                    return Some(target_dir);
                }
            }
        }

        // Strategy 3: Look in current working directory
        if let Ok(cwd) = std::env::current_dir() {
            let target_dir = cwd.join("target").join("debug");
            if target_dir.exists() {
                return Some(target_dir);
            }
        }

        None
    }

    /// Create a new compiler using system rustc
    pub fn system() -> Result<Self> {
        // Check if rustc is available
        let output = Command::new("rustc")
            .arg("--version")
            .output()
            .map_err(|_| {
                LobError::Toolchain(
                    "rustc not found. Please install Rust from https://rustup.rs/".to_string(),
                )
            })?;

        if !output.status.success() {
            return Err(LobError::Toolchain(
                "rustc not working properly".to_string(),
            ));
        }

        Ok(Self {
            rustc_path: PathBuf::from("rustc"),
            sysroot: None,
        })
    }

    /// Create a compiler with custom rustc path and sysroot
    #[allow(dead_code)]
    pub fn custom(rustc_path: PathBuf, sysroot: Option<PathBuf>) -> Self {
        Self {
            rustc_path,
            sysroot,
        }
    }

    /// Compile source code to binary
    pub fn compile(
        &self,
        source_path: &PathBuf,
        output_path: &PathBuf,
        user_expr: Option<&str>,
    ) -> Result<()> {
        let mut cmd = Command::new(&self.rustc_path);

        cmd.arg("--edition=2021")
            .arg("-C")
            .arg("opt-level=3")
            .arg("--crate-type")
            .arg("bin")
            .arg("-o")
            .arg(output_path)
            .arg(source_path);

        // Add extern crate paths for lob-prelude and its dependencies
        if let Some(target_dir) = Self::find_target_dir() {
            cmd.arg("--extern")
                .arg(format!(
                    "lob_prelude={}/liblob_prelude.rlib",
                    target_dir.display()
                ))
                .arg("--extern")
                .arg(format!(
                    "lob_core={}/liblob_core.rlib",
                    target_dir.display()
                ))
                .arg("-L")
                .arg(format!("dependency={}", target_dir.join("deps").display()));
        }

        // Add sysroot if provided (for embedded toolchain)
        if let Some(sysroot) = &self.sysroot {
            cmd.arg("--sysroot").arg(sysroot);
        }

        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let formatted = LobError::format_compilation_error(&stderr, user_expr);
            return Err(LobError::Compilation(formatted));
        }

        Ok(())
    }

    /// Compile and cache a generated program
    pub fn compile_and_cache(
        &self,
        source: &str,
        cache: &Cache,
        user_expr: Option<&str>,
    ) -> Result<PathBuf> {
        let hash = cache.hash_source(source);

        // Check cache first
        if let Some(binary_path) = cache.get_binary(&hash) {
            return Ok(binary_path);
        }

        // Cache miss - compile
        let source_path = cache.store_source(&hash, source)?;
        let binary_path = cache.binary_path(&hash);

        self.compile(&source_path, &binary_path, user_expr)?;

        Ok(binary_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_compiler_available() {
        // This test will pass if rustc is installed
        match Compiler::system() {
            Ok(_) => (),
            Err(e) => panic!("System compiler not available: {}", e),
        }
    }
}
