//! Integration tests for new features

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn temp_file(name: &str, content: &str) -> PathBuf {
    let path = std::env::temp_dir().join(name);
    fs::write(&path, content).unwrap();
    path
}

#[test]
fn test_file_input() -> Result<()> {
    let file = temp_file("test_file_input.txt", "hello\nworld\ntest\n");

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("_.filter(|x| x.len() > 4)")
        .arg(file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("\"hello\""))
        .stdout(predicate::str::contains("\"world\""));

    Ok(())
}

#[test]
fn test_multiple_files() -> Result<()> {
    let file1 = temp_file("test_multi1.txt", "a\nb\nc\n");
    let file2 = temp_file("test_multi2.txt", "c\nd\ne\n");

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("_.unique().count()")
        .arg(file1.to_str().unwrap())
        .arg(file2.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("5"));

    Ok(())
}

#[test]
fn test_csv_parsing() -> Result<()> {
    let csv = temp_file(
        "test_csv.csv",
        "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,SF\n",
    );

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--parse-csv")
        .arg("_.count()")
        .arg(csv.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("3"));

    Ok(())
}

#[test]
fn test_csv_filtering() -> Result<()> {
    let csv = temp_file(
        "test_csv_filter.csv",
        "name,age,city\nAlice,30,NYC\nBob,25,LA\nCharlie,35,SF\n",
    );

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--parse-csv")
        .arg("_.filter(|r| r[\"age\"].parse::<i32>().unwrap() > 26)")
        .arg(csv.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Charlie"))
        .stdout(predicate::str::contains("Bob").not());

    Ok(())
}

#[test]
fn test_json_output() -> Result<()> {
    let file = temp_file("test_json.txt", "1\n2\n3\n");

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--format")
        .arg("json")
        .arg("_.take(2)")
        .arg(file.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("["))
        .stdout(predicate::str::contains("]"))
        .stdout(predicate::str::contains("\"1\""))
        .stdout(predicate::str::contains("\"2\""));

    Ok(())
}

#[test]
fn test_jsonl_output() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--format")
        .arg("jsonl")
        .write_stdin("1\n2\n3\n")
        .arg("_.take(2)")
        .assert()
        .success()
        .stdout(predicate::str::contains("\"1\""))
        .stdout(predicate::str::contains("\"2\""))
        .stdout(predicate::str::contains("\"3\"").not());

    Ok(())
}

#[test]
fn test_error_suggestions() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("lob");
    cmd.write_stdin("1\n2\n3\n")
        .arg("_.filter(|x| x > 1)")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Problem:"))
        .stderr(predicate::str::contains("How to fix:"))
        .stderr(predicate::str::contains("parse"));

    Ok(())
}

#[test]
fn test_show_source_with_csv() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--show-source")
        .arg("--parse-csv")
        .arg("_.take(5)")
        .assert()
        .success()
        .stdout(predicate::str::contains("input_csv"))
        .stdout(predicate::str::contains("use lob_prelude::*"));

    Ok(())
}

#[test]
fn test_csv_with_json_output() -> Result<()> {
    let csv = temp_file("test_csv_json.csv", "name,age\nAlice,30\nBob,25\n");

    let mut cmd = cargo_bin_cmd!("lob");
    cmd.arg("--parse-csv")
        .arg("--format")
        .arg("json")
        .arg("_.take(2)")
        .arg(csv.to_str().unwrap())
        .assert()
        .success()
        .stdout(predicate::str::contains("["))
        .stdout(predicate::str::contains("Alice"))
        .stdout(predicate::str::contains("Bob"));

    Ok(())
}

#[test]
fn test_stdin_still_works() -> Result<()> {
    let mut cmd = cargo_bin_cmd!("lob");
    cmd.write_stdin("hello\nworld\n")
        .arg("_.count()")
        .assert()
        .success()
        .stdout(predicate::str::contains("2"));

    Ok(())
}
