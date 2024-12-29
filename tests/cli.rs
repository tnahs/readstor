#![allow(missing_docs)]

use assert_cmd::Command;
use once_cell::sync::Lazy;

const BINARY: &str = env!("CARGO_PKG_NAME");

static OUTPUT_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let path = std::env::temp_dir().join("readstor").join("tests");
    std::fs::create_dir_all(&path).expect("could not create temp directory for testing");
    path.display().to_string()
});

static DATABASES_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases", "books-annotated"].iter());
    path.display().to_string()
});

static TEMPLATES_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.push("templates");
    path.display().to_string()
});

#[test]
fn version() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args(["--version"]).assert().code(0).success();
}

#[test]
fn help() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args(["--help"]).assert().code(0).success();
}

#[test]
fn default_render() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args([
        "render",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--databases-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_export() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args([
        "export",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--databases-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_backup() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args([
        "backup",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--databases-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn render_example_templates() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args([
        "render",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--databases-directory",
        &DATABASES_DIRECTORY,
        "--templates-directory",
        &TEMPLATES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn missing_output_directory() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args(["--force", "--output-directory", "./path/does/not/exist"])
        .assert()
        .code(2)
        .failure();
}

#[test]
fn missing_databases_directory() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args(["--force", "--databases-directory", "./path/does/not/exist"])
        .assert()
        .code(2)
        .failure();
}

#[test]
fn missing_plists_directory() {
    let mut c = Command::cargo_bin(BINARY).unwrap();
    c.args(["--force", "--plists-directory", "./path/does/not/exist"])
        .assert()
        .code(2)
        .failure();
}
