use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;

fn golden_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/golden")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
}

fn collect_tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, output: &mut BTreeMap<String, Vec<u8>>) {
        let mut entries = fs::read_dir(dir)
            .unwrap_or_else(|error| panic!("reading {}: {error}", dir.display()))
            .collect::<Result<Vec<_>, _>>()
            .expect("directory entries");
        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                walk(base, &path, output);
            } else {
                let relative = path
                    .strip_prefix(base)
                    .expect("relative path")
                    .to_string_lossy()
                    .replace('\\', "/");
                output.insert(relative, fs::read(&path).expect("read file"));
            }
        }
    }

    let mut output = BTreeMap::new();
    if root.exists() {
        walk(root, root, &mut output);
    }
    output
}

fn expected_error_message(stderr: &str) -> Option<String> {
    stderr
        .lines()
        .find_map(|line| line.strip_prefix("Error: ").map(str::to_string))
}

#[test]
fn matches_go_golden_fixtures() {
    let root = golden_root();
    let mut cases = fs::read_dir(&root)
        .expect("golden fixtures")
        .map(|entry| entry.expect("entry").path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    cases.sort();

    assert!(!cases.is_empty(), "no golden fixtures found");

    for case in cases {
        let name = case
            .file_name()
            .expect("case name")
            .to_string_lossy()
            .to_string();

        let args = read(&case.join("args.txt"))
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let expected_code = read(&case.join("exit_code.txt"))
            .trim()
            .parse::<i32>()
            .expect("exit code");
        let expected_stdout = read(&case.join("stdout.txt"));
        let expected_stderr = read(&case.join("stderr.txt"));

        let temp = tempfile::tempdir().expect("tempdir");
        let output = Command::cargo_bin("devforge")
            .expect("devforge binary")
            .args(&args)
            .current_dir(temp.path())
            .output()
            .expect("run devforge");

        let code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
        let stderr = String::from_utf8(output.stderr).expect("stderr utf-8");

        assert_eq!(code, expected_code, "{name}: exit code");

        if name == "version" {
            assert!(
                stdout.starts_with("devforge "),
                "{name}: unexpected version output {stdout:?}"
            );
        } else {
            assert_eq!(stdout, expected_stdout, "{name}: stdout");
        }

        let expected_tree = collect_tree(&case.join("tree"));
        let actual_tree = collect_tree(temp.path());
        assert_eq!(actual_tree, expected_tree, "{name}: generated tree");

        if expected_code == 0 {
            assert_eq!(stderr, "", "{name}: stderr should be empty");
            continue;
        }

        if name == "create-worker-python" || name == "error-no-templates" {
            assert!(
                stderr.contains("no templates found for worker/python"),
                "{name}: unexpected stderr {stderr:?}"
            );
            continue;
        }

        let expected_message =
            expected_error_message(&expected_stderr).expect("expected error message");
        assert!(
            stderr.contains(&expected_message),
            "{name}: stderr {stderr:?} does not contain {expected_message:?}"
        );
    }
}
