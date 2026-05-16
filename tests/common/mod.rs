#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use osakit::Language;

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn apple_script_language() -> Language {
    Language::for_name("AppleScript")
        .expect("AppleScript lookup should succeed")
        .expect("AppleScript should be available")
}

pub fn artifact_path(name: &str, extension: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-artifacts");
    fs::create_dir_all(&dir).expect("test-artifact directory should be creatable");
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    dir.join(format!("{name}-{id}.{extension}"))
}

pub fn write_text_fixture(name: &str, extension: &str, contents: &str) -> PathBuf {
    let path = artifact_path(name, extension);
    fs::write(&path, contents).expect("fixture write should succeed");
    path
}

pub fn run_example(name: &str) {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("debug")
        .join("examples")
        .join(format!("{name}{}", env::consts::EXE_SUFFIX));
    let output = Command::new(&path).output().unwrap_or_else(|error| {
        panic!(
            "failed to launch example {name} at {}: {error}",
            path.display()
        )
    });
    assert!(
        output.status.success(),
        "example {name} failed with stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
