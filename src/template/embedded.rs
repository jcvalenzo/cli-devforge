use std::path::Path;

use include_dir::{include_dir, Dir, File};

use crate::{Error, Result};

static TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates");

/// Returns every template file for a prefix/language pair, sorted by path.
pub fn files(prefix: &str, lang: &str) -> Result<Vec<&'static File<'static>>> {
    let root = TEMPLATES
        .get_dir(format!("{prefix}/{lang}"))
        .ok_or_else(|| Error::NoTemplates {
            prefix: prefix.to_string(),
            lang: lang.to_string(),
        })?;

    let mut files = Vec::new();
    collect(root, &mut files);

    if files.is_empty() {
        return Err(Error::NoTemplates {
            prefix: prefix.to_string(),
            lang: lang.to_string(),
        });
    }

    files.sort_by(|left, right| left.path().cmp(right.path()));
    Ok(files)
}

pub fn read(path: &str) -> Result<&'static str> {
    let file = TEMPLATES
        .get_file(path)
        .ok_or_else(|| Error::ReadTemplate {
            path: path.to_string(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "file does not exist"),
        })?;

    file.contents_utf8().ok_or_else(|| Error::ReadTemplate {
        path: path.to_string(),
        source: std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "template is not valid UTF-8",
        ),
    })
}

pub fn all_files() -> Vec<&'static File<'static>> {
    let mut files = Vec::new();
    collect(&TEMPLATES, &mut files);
    files.sort_by(|left, right| left.path().cmp(right.path()));
    files
}

/// Converts `api/go/cmd/server/main.go.tmpl` into `cmd/server/main.go`.
pub fn output_relative_path(file_path: &Path, prefix: &str, lang: &str) -> Result<String> {
    let relative = file_path
        .strip_prefix(Path::new(prefix))
        .and_then(|path| path.strip_prefix(Path::new(lang)))
        .map_err(|_| {
            Error::ListTemplates(format!("unexpected template path {}", file_path.display()))
        })?;

    let relative = relative.to_string_lossy().replace('\\', "/");
    Ok(relative
        .strip_suffix(".tmpl")
        .unwrap_or(&relative)
        .to_string())
}

fn collect<'a>(dir: &'a Dir<'a>, files: &mut Vec<&'a File<'a>>) {
    files.extend(dir.files());
    for child in dir.dirs() {
        collect(child, files);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::{Data, GoSubstitutionRenderer, Renderer};

    fn sample_data() -> Data {
        Data {
            full_name: "api-catalog".to_string(),
            prefix: "api".to_string(),
            project_name: "catalog".to_string(),
            lang: "go".to_string(),
            author: "test".to_string(),
            module_path: "github.com/test/catalog".to_string(),
            license: "MIT".to_string(),
            date: "2026-01-01".to_string(),
            description: "Test API".to_string(),
        }
    }

    fn relative_paths(prefix: &str, lang: &str) -> Vec<String> {
        files(prefix, lang)
            .expect("templates")
            .iter()
            .map(|file| output_relative_path(file.path(), prefix, lang).expect("output path"))
            .collect()
    }

    #[test]
    fn finds_api_go_templates() {
        assert_eq!(files("api", "go").expect("api/go").len(), 7);
    }

    #[test]
    fn finds_cli_java_templates() {
        assert_eq!(files("cli", "java").expect("cli/java").len(), 2);
    }

    #[test]
    fn reports_missing_language() {
        let error = files("worker", "python").expect_err("worker/python is empty");
        assert!(matches!(error, Error::NoTemplates { .. }));
    }

    #[test]
    fn strips_prefix_and_suffix_from_output_paths() {
        let paths = relative_paths("api", "go");
        assert!(paths.contains(&".gitignore".to_string()));
        assert!(paths.contains(&"cmd/server/main.go".to_string()));
        assert!(paths.contains(&"go.mod".to_string()));
    }

    #[test]
    fn embeds_dotfiles() {
        assert!(relative_paths("api", "go").contains(&".gitignore".to_string()));
        assert!(relative_paths("cli", "java").contains(&"src/main/java/.gitkeep".to_string()));
    }

    #[test]
    fn every_template_renders_with_known_fields() {
        let data = sample_data();
        let renderer = GoSubstitutionRenderer;

        for file in all_files() {
            if file.path().extension().and_then(|ext| ext.to_str()) != Some("tmpl") {
                continue;
            }

            let path = file.path().to_string_lossy().replace('\\', "/");
            let source = file.contents_utf8().expect("utf-8 template");
            let rendered = renderer
                .render(&path, source, &data)
                .unwrap_or_else(|error| panic!("{path}: {error}"));

            assert!(!rendered.contains("{{"), "{path} has unresolved actions");
        }
    }

    #[test]
    fn every_registered_language_has_templates() {
        for prefix in crate::prefix::all() {
            for lang in prefix.langs {
                if prefix.name == "worker" && *lang == "python" {
                    assert!(files(prefix.name, lang).is_err());
                    continue;
                }

                assert!(
                    files(prefix.name, lang).is_ok(),
                    "missing templates for {}/{}",
                    prefix.name,
                    lang
                );
            }
        }
    }
}
