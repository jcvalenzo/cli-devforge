use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=DEVFORGE_VERSION");

    if Path::new(".git").exists() {
        println!("cargo:rerun-if-changed=.git/HEAD");
        println!("cargo:rerun-if-changed=.git/refs/tags");
    }

    let version = std::env::var("DEVFORGE_VERSION")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(git_describe)
        .unwrap_or_else(|| "dev".to_string());

    println!("cargo:rustc-env=DEVFORGE_VERSION={version}");
}

fn git_describe() -> Option<String> {
    let output = Command::new("git")
        .args(["describe", "--tags", "--always", "--dirty"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let version = String::from_utf8(output.stdout).ok()?.trim().to_string();
    (!version.is_empty()).then_some(version)
}
