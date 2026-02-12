//! Comprehensive tests for join operations

use lob_core::LobExt;

#[test]
fn inner_join_basic() {
    let left = vec![(1, "a"), (2, "b"), (3, "c")];
    let right = vec![(1, "x"), (2, "y"), (4, "z")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
    assert!(result.contains(&((1, "a"), (1, "x"))));
    assert!(result.contains(&((2, "b"), (2, "y"))));
}

#[test]
fn inner_join_no_matches() {
    let left = vec![(1, "a"), (2, "b")];
    let right = vec![(3, "x"), (4, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 0);
}

#[test]
fn inner_join_all_match() {
    let left = vec![(1, "a"), (2, "b")];
    let right = vec![(1, "x"), (2, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
}

#[test]
fn inner_join_multiple_matches() {
    let left = vec![(1, "a"), (1, "b")];
    let right = vec![(1, "x"), (1, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    // 2 left * 2 right = 4 results
    assert_eq!(result.len(), 4);
}

#[test]
fn inner_join_empty_left() {
    let left: Vec<(i32, &str)> = vec![];
    let right = vec![(1, "x"), (2, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 0);
}

#[test]
fn inner_join_empty_right() {
    let left = vec![(1, "a"), (2, "b")];
    let right: Vec<(i32, &str)> = vec![];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 0);
}

#[test]
fn inner_join_strings() {
    let left = vec![("key1", 1), ("key2", 2)];
    let right = vec![("key1", "a"), ("key2", "b"), ("key3", "c")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
}

#[test]
fn left_join_basic() {
    let left = vec![(1, "a"), (2, "b"), (3, "c")];
    let right = vec![(1, "x"), (2, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0], ((1, "a"), Some((1, "x"))));
    assert_eq!(result[1], ((2, "b"), Some((2, "y"))));
    assert_eq!(result[2], ((3, "c"), None));
}

#[test]
fn left_join_all_match() {
    let left = vec![(1, "a"), (2, "b")];
    let right = vec![(1, "x"), (2, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|(_, r)| r.is_some()));
}

#[test]
fn left_join_no_matches() {
    let left = vec![(1, "a"), (2, "b")];
    let right = vec![(3, "x"), (4, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|(_, r)| r.is_none()));
}

#[test]
fn left_join_empty_left() {
    let left: Vec<(i32, &str)> = vec![];
    let right = vec![(1, "x"), (2, "y")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 0);
}

#[test]
fn inner_join_with_iteration() {
    // Test that explicitly iterates through results one by one
    let left = vec![(1, "a"), (2, "b"), (3, "c")];
    let right = vec![(1, "x"), (2, "y"), (2, "z")];

    let mut iter = left
        .into_iter()
        .lob()
        .join_inner(right, |x| x.0, |x| x.0)
        .into_iter();

    let first = iter.next();
    assert!(first.is_some());

    let second = iter.next();
    assert!(second.is_some());

    let third = iter.next();
    assert!(third.is_some());

    let fourth = iter.next();
    assert!(fourth.is_none());
}

#[test]
fn left_join_with_iteration() {
    // Test that explicitly iterates through results one by one
    let left = vec![(1, "a"), (2, "b"), (3, "c")];
    let right = vec![(1, "x"), (2, "y")];

    let iter = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .into_iter();

    // Should get all left items
    let mut count = 0;
    for _ in iter {
        count += 1;
    }
    assert_eq!(count, 3);
}

#[test]
fn left_join_empty_right() {
    let left = vec![(1, "a"), (2, "b")];
    let right: Vec<(i32, &str)> = vec![];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 2);
    assert!(result.iter().all(|(_, r)| r.is_none()));
}

#[test]
fn left_join_multiple_matches() {
    let left = vec![(1, "a")];
    let right = vec![(1, "x"), (1, "y"), (1, "z")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    // One left item matched with 3 right items
    assert_eq!(result.len(), 3);
    assert!(result.iter().all(|(_, r)| r.is_some()));
}

#[test]
fn left_join_multiple_left_multiple_right() {
    // Multiple left items each matching multiple right items.
    // Exercises the state reset (current_left = None, idx = 0, emitted = false)
    // after exhausting right matches for each left key.
    let left = vec![(1, "a"), (2, "b")];
    let right = vec![(1, "x"), (1, "y"), (2, "p"), (2, "q")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    // 2 right matches for key=1 + 2 right matches for key=2 = 4 total
    assert_eq!(result.len(), 4);
    assert!(result.iter().all(|(_, r)| r.is_some()));
    // Verify ordering: all key=1 matches come before key=2 matches
    assert_eq!(result[0].0, (1, "a"));
    assert_eq!(result[1].0, (1, "a"));
    assert_eq!(result[2].0, (2, "b"));
    assert_eq!(result[3].0, (2, "b"));
}

#[test]
fn left_join_mixed_match_and_no_match() {
    // Interleave matching and non-matching left keys to exercise
    // both the "emit with None" and "exhaust right matches" paths.
    let left = vec![(1, "a"), (2, "b"), (3, "c"), (4, "d")];
    let right = vec![(1, "x"), (1, "y"), (3, "z")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .join_left(right, |x| x.0, |x| x.0)
        .collect();

    // key=1: 2 matches, key=2: None, key=3: 1 match, key=4: None => 5 total
    assert_eq!(result.len(), 5);
    assert_eq!(result[0], ((1, "a"), Some((1, "x"))));
    assert_eq!(result[1], ((1, "a"), Some((1, "y"))));
    assert_eq!(result[2], ((2, "b"), None));
    assert_eq!(result[3], ((3, "c"), Some((3, "z"))));
    assert_eq!(result[4], ((4, "d"), None));
}

#[test]
fn join_with_filter() {
    let left = vec![(1, "a"), (2, "b"), (3, "c"), (4, "d")];
    let right = vec![(1, "x"), (2, "y"), (3, "z")];

    let result: Vec<_> = left
        .into_iter()
        .lob()
        .filter(|x| x.0 <= 3)
        .join_inner(right, |x| x.0, |x| x.0)
        .collect();

    assert_eq!(result.len(), 3);
}
