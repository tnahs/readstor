#![allow(missing_docs)]

use assert_cmd::Command;
use once_cell::sync::Lazy;

use lib::defaults::NAME;

static OUTPUT_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let path = std::env::temp_dir().join(NAME).join("tests");
    std::fs::create_dir_all(&path).expect("could not create temp directory for testing");
    path.display().to_string()
});

static DATABASES_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "databases", "books-annotated"].iter());
    path.display().to_string()
});

static PLISTS_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.extend(["data", "plists", "books-annotated"].iter());
    path.display().to_string()
});

static TEMPLATES_DIRECTORY: Lazy<String> = Lazy::new(|| {
    let mut path = lib::defaults::CRATE_ROOT.to_owned();
    path.push("templates");
    path.display().to_string()
});

#[test]
fn version() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args(["--version"]).assert().code(0).success();
}

#[test]
fn help() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args(["--help"]).assert().code(0).success();
}

#[test]
fn default_render_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "render",
        "macos",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_render_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "render",
        "ios",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &PLISTS_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_export_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "macos",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_export_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "ios",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &PLISTS_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_backup_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "backup",
        "macos",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &DATABASES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn default_backup_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "backup",
        "ios",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &PLISTS_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn render_example_templates_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "render",
        "macos",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &DATABASES_DIRECTORY,
        "--templates-directory",
        &TEMPLATES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn render_example_templates_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "render",
        "ios",
        "--force",
        "--output-directory",
        &OUTPUT_DIRECTORY,
        "--data-directory",
        &PLISTS_DIRECTORY,
        "--templates-directory",
        &TEMPLATES_DIRECTORY,
    ])
    .assert()
    .code(0)
    .success();
}

#[test]
fn missing_output_directory_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "macos",
        "--force",
        "--output-directory",
        "./path/does/not/exist",
    ])
    .assert()
    .code(2)
    .failure();
}

#[test]
fn missing_output_directory_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "ios",
        "--force",
        "--output-directory",
        "./path/does/not/exist",
    ])
    .assert()
    .code(2)
    .failure();
}

#[test]
fn missing_data_directory_macos() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "macos",
        "--force",
        "--data-directory",
        "./path/does/not/exist",
    ])
    .assert()
    .code(2)
    .failure();
}

#[test]
fn missing_data_directory_ios() {
    let mut c = Command::cargo_bin(NAME).unwrap();
    c.args([
        "export",
        "ios",
        "--force",
        "--data-directory",
        "./path/does/not/exist",
    ])
    .assert()
    .code(2)
    .failure();
}
