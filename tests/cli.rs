use std::fs;

use assert_cmd::Command;

#[test]
fn version_prints_name_and_version() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .arg("version")
        .output()
        .expect("run version");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(
        stdout.starts_with("devforge "),
        "unexpected output: {stdout:?}"
    );
}

#[test]
fn home_screen_shows_title_and_commands() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .env("NO_COLOR", "1")
        .output()
        .expect("run devforge");

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("devforge"));
    assert!(stdout.contains("devforge list"));
    assert!(stdout.contains("devforge --help"));
    assert!(
        !stdout.contains('\u{1b}'),
        "unexpected ANSI output: {stdout:?}"
    );
}

#[test]
fn home_screen_uses_color_when_forced() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .env_remove("NO_COLOR")
        .env("CLICOLOR_FORCE", "1")
        .output()
        .expect("run devforge");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(
        stdout.contains("\u{1b}["),
        "expected ANSI output: {stdout:?}"
    );
}

#[test]
fn help_does_not_show_home_banner() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .arg("--help")
        .output()
        .expect("run help");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("Usage: devforge <COMMAND>"));
    assert!(!stdout.contains("██████"));
    assert!(!stdout.contains("Generador de scaffolds"));
}

#[test]
fn list_contains_known_prefixes() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .arg("list")
        .output()
        .expect("run list");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.starts_with("PREFIX"));
    assert!(stdout.contains("api-"));
    assert!(stdout.contains("monitoring-"));
}

#[test]
fn create_generates_a_go_project() {
    let temp = tempfile::tempdir().expect("tempdir");
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .args(["create", "api-demo", "--lang", "go", "--author", "Test"])
        .current_dir(temp.path())
        .output()
        .expect("run create");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("Project \"api-demo\" created successfully in ././api-demo"));
    assert!(temp.path().join("api-demo/go.mod").is_file());
    assert!(temp.path().join("api-demo/.gitignore").is_file());

    let go_mod = fs::read_to_string(temp.path().join("api-demo/go.mod")).expect("go.mod");
    assert!(go_mod.contains("github.com/example/demo"));
}

#[test]
fn create_uses_default_language() {
    let temp = tempfile::tempdir().expect("tempdir");
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .args(["create", "web-demo"])
        .current_dir(temp.path())
        .output()
        .expect("run create");

    assert!(output.status.success());
    assert!(temp.path().join("web-demo/package.json").is_file());
}

#[test]
fn rejects_invalid_project_name() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .args(["create", "apicatalog"])
        .output()
        .expect("run create");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");
    assert!(stderr.contains("invalid project name \"apicatalog\""));
}

#[test]
fn rejects_unsupported_language() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .args(["create", "api-demo", "--lang", "python"])
        .output()
        .expect("run create");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");
    assert!(stderr.contains("language \"python\" not supported for prefix \"api\""));
}

#[test]
fn reports_missing_templates() {
    let output = Command::cargo_bin("devforge")
        .expect("devforge binary")
        .args(["create", "worker-demo", "--lang", "python"])
        .output()
        .expect("run create");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");
    assert!(stderr.contains("no templates found for worker/python"));
}
