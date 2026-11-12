use std::fs;
use std::path::Path;

use chrono::Local;

use crate::prefix::Prefix;
use crate::template::embedded;
use crate::template::{Data, GoSubstitutionRenderer, Renderer};
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct Options {
    pub full_name: String,
    pub prefix: String,
    pub project_name: String,
    pub lang: String,
    pub author: String,
    pub module_path: String,
    pub license: String,
    pub output_dir: String,
    pub description: String,
}

pub fn generate(prefix: &Prefix, options: &Options) -> Result<()> {
    let module_path = if options.module_path.is_empty() && options.lang == "go" {
        format!("github.com/example/{}", options.project_name)
    } else {
        options.module_path.clone()
    };

    let data = Data {
        full_name: options.full_name.clone(),
        prefix: options.prefix.clone(),
        project_name: options.project_name.clone(),
        lang: options.lang.clone(),
        author: options.author.clone(),
        module_path,
        license: options.license.clone(),
        date: Local::now().format("%Y-%m-%d").to_string(),
        description: options.description.clone(),
    };

    let files = embedded::files(prefix.template_dir, &options.lang)?;
    let project_dir = Path::new(&options.output_dir).join(&options.full_name);
    let renderer = GoSubstitutionRenderer;

    for file in files {
        let template_path = file.path().to_string_lossy().replace('\\', "/");
        let source = file.contents_utf8().ok_or_else(|| Error::ReadTemplate {
            path: template_path.clone(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "template is not valid UTF-8",
            ),
        })?;

        let rendered = renderer.render(&template_path, source, &data)?;
        let relative_path =
            embedded::output_relative_path(file.path(), prefix.template_dir, &options.lang)?;
        let full_path = project_dir.join(&relative_path);

        write_file(&full_path, &rendered)?;
        println!("  created  {relative_path}");
    }

    Ok(())
}

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent).map_err(|source| Error::CreateDir {
                path: parent.display().to_string(),
                source,
            })?;
        }
    }

    write_contents(path, content).map_err(|source| Error::Write {
        path: path.display().to_string(),
        source,
    })
}

fn create_dir_all(path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;

        fs::DirBuilder::new()
            .recursive(true)
            .mode(0o755)
            .create(path)
    }

    #[cfg(not(unix))]
    {
        fs::create_dir_all(path)
    }
}

fn write_contents(path: &Path, content: &str) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o644)
            .open(path)?;
        file.write_all(content.as_bytes())
    }

    #[cfg(not(unix))]
    {
        fs::write(path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prefix;

    fn options(output_dir: &Path, lang: &str) -> Options {
        Options {
            full_name: "api-test".to_string(),
            prefix: "api".to_string(),
            project_name: "test".to_string(),
            lang: lang.to_string(),
            author: "test".to_string(),
            module_path: String::new(),
            license: "MIT".to_string(),
            output_dir: output_dir.display().to_string(),
            description: "Test API".to_string(),
        }
    }

    #[test]
    fn generates_api_go_project() {
        let temp = tempfile::tempdir().expect("tempdir");
        let prefix = prefix::find("api").expect("api prefix");

        generate(prefix, &options(temp.path(), "go")).expect("generate");

        let project = temp.path().join("api-test");
        for relative in [
            "go.mod",
            "cmd/server/main.go",
            "internal/handler/handler.go",
            "Makefile",
            "Dockerfile",
            "README.md",
            ".gitignore",
        ] {
            assert!(project.join(relative).is_file(), "missing {relative}");
        }

        let go_mod = fs::read_to_string(project.join("go.mod")).expect("go.mod");
        assert!(go_mod.contains("github.com/example/test"));
    }

    #[test]
    fn honours_custom_module_path() {
        let temp = tempfile::tempdir().expect("tempdir");
        let prefix = prefix::find("api").expect("api prefix");
        let mut options = options(temp.path(), "go");
        options.module_path = "github.com/acme/catalog".to_string();

        generate(prefix, &options).expect("generate");

        let go_mod = fs::read_to_string(temp.path().join("api-test/go.mod")).expect("go.mod");
        assert!(go_mod.contains("github.com/acme/catalog"));
    }

    #[test]
    fn reports_missing_templates() {
        let temp = tempfile::tempdir().expect("tempdir");
        let prefix = prefix::find("worker").expect("worker prefix");
        let mut options = options(temp.path(), "python");
        options.prefix = "worker".to_string();
        options.full_name = "worker-test".to_string();

        let error = generate(prefix, &options).expect_err("missing templates");
        assert!(matches!(error, Error::NoTemplates { .. }));
    }
}
