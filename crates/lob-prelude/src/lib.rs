//! User-facing prelude for lob data pipelines
//!
//! This crate provides the public API that users interact with in their
//! generated code. It re-exports the core functionality and adds convenient
//! helpers like `input()` for reading from stdin.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{self, BufRead};

// Re-export core types and traits
pub use lob_core::{HashMap, HashSet, Lob, LobExt};

/// Creates a Lob iterator from stdin lines
///
/// This function reads lines from stdin and returns a `Lob` iterator over them.
/// Lines are trimmed and empty lines are filtered out by default.
///
/// # Examples
///
/// ```no_run
/// use lob_prelude::*;
///
/// // Read lines from stdin and filter
/// let result: Vec<_> = input()
///     .filter(|line| line.contains("ERROR"))
///     .collect();
/// ```
#[must_use]
pub fn input() -> Lob<impl Iterator<Item = String>> {
    let stdin = io::stdin();
    #[allow(clippy::lines_filter_map_ok)]
    Lob::new(
        stdin
            .lock()
            .lines()
            .filter_map(Result::ok)
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty()),
    )
}

/// Creates a Lob iterator from any iterable
///
/// This is a convenience function to convert any type that implements
/// `IntoIterator` into a `Lob` iterator.
///
/// # Examples
///
/// ```
/// use lob_prelude::*;
///
/// let result: Vec<_> = lob(vec![1, 2, 3, 4, 5])
///     .filter(|x| x % 2 == 0)
///     .collect();
///
/// assert_eq!(result, vec![2, 4]);
/// ```
#[must_use]
pub fn lob<I: IntoIterator>(iterable: I) -> Lob<I::IntoIter> {
    Lob::new(iterable.into_iter())
}

/// Creates a Lob iterator from a range
///
/// # Examples
///
/// ```
/// use lob_prelude::*;
///
/// let result: Vec<_> = range(0, 5)
///     .map(|x| x * 2)
///     .collect();
///
/// assert_eq!(result, vec![0, 2, 4, 6, 8]);
/// ```
#[must_use]
pub fn range(start: i64, end: i64) -> Lob<impl Iterator<Item = i64>> {
    Lob::new(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lob_from_vec() {
        let result: Vec<_> = lob(vec![1, 2, 3, 4, 5]).filter(|x| x % 2 == 0).collect();
        assert_eq!(result, vec![2, 4]);
    }

    #[test]
    fn range_basic() {
        let result: Vec<_> = range(0, 5).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn chained_operations() {
        let result: Vec<_> = lob(vec![1, 2, 3, 4, 5])
            .filter(|x| x % 2 == 0)
            .map(|x| x * 2)
            .take(2)
            .collect();
        assert_eq!(result, vec![4, 8]);
    }
}
