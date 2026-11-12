use crate::{Error, Result};

#[derive(Debug, Clone, Default)]
pub struct Data {
    pub full_name: String,
    pub prefix: String,
    pub project_name: String,
    pub lang: String,
    pub author: String,
    pub module_path: String,
    pub license: String,
    pub date: String,
    pub description: String,
}

pub trait Renderer {
    fn render(&self, path: &str, source: &str, data: &Data) -> Result<String>;
}

/// Minimal renderer compatible with the subset of Go's text/template syntax
/// used by the templates: `{{.Field}}` with optional surrounding whitespace.
pub struct GoSubstitutionRenderer;

impl Renderer for GoSubstitutionRenderer {
    fn render(&self, path: &str, source: &str, data: &Data) -> Result<String> {
        let mut output = String::with_capacity(source.len());
        let mut remaining = source;

        while let Some(start) = remaining.find("{{") {
            output.push_str(&remaining[..start]);

            let after_start = &remaining[start + 2..];
            let Some(end) = after_start.find("}}") else {
                return Err(Error::UnsupportedSyntax {
                    path: path.to_string(),
                    snippet: remaining[start..].chars().take(40).collect(),
                });
            };

            let expression = after_start[..end].trim();
            let field = expression
                .strip_prefix('.')
                .filter(|field| is_identifier(field))
                .ok_or_else(|| Error::UnsupportedSyntax {
                    path: path.to_string(),
                    snippet: format!("{{{{{expression}}}}}"),
                })?;

            let value = field_value(data, field).ok_or_else(|| Error::UnknownField {
                path: path.to_string(),
                field: field.to_string(),
            })?;

            output.push_str(value);
            remaining = &after_start[end + 2..];
        }

        output.push_str(remaining);
        Ok(output)
    }
}

fn is_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() || first == '_' => {}
        _ => return false,
    }
    chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn field_value<'a>(data: &'a Data, field: &str) -> Option<&'a str> {
    Some(match field {
        "FullName" => &data.full_name,
        "Prefix" => &data.prefix,
        "ProjectName" => &data.project_name,
        "Lang" => &data.lang,
        "Author" => &data.author,
        "ModulePath" => &data.module_path,
        "License" => &data.license,
        "Date" => &data.date,
        "Description" => &data.description,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn renders_all_fields() {
        let source = "{{.FullName}}|{{.Prefix}}|{{.ProjectName}}|{{.Lang}}|{{.Author}}|{{.ModulePath}}|{{.License}}|{{.Date}}|{{.Description}}";
        let rendered = GoSubstitutionRenderer
            .render("test", source, &sample_data())
            .expect("render");

        assert_eq!(
            rendered,
            "api-catalog|api|catalog|go|test|github.com/test/catalog|MIT|2026-01-01|Test API"
        );
    }

    #[test]
    fn supports_whitespace_inside_actions() {
        let rendered = GoSubstitutionRenderer
            .render("test", "name: {{ .ProjectName }}", &sample_data())
            .expect("render");

        assert_eq!(rendered, "name: catalog");
    }

    #[test]
    fn leaves_plain_text_untouched() {
        let source = "package main\n\nfunc main() {}\n";
        let rendered = GoSubstitutionRenderer
            .render("test", source, &sample_data())
            .expect("render");

        assert_eq!(rendered, source);
    }

    #[test]
    fn rejects_unknown_fields() {
        let error = GoSubstitutionRenderer
            .render("test", "{{.Unknown}}", &sample_data())
            .expect_err("unknown field");

        assert!(matches!(error, Error::UnknownField { .. }));
    }

    #[test]
    fn rejects_control_structures() {
        for source in [
            "{{if .FullName}}x{{end}}",
            "{{range .Langs}}x{{end}}",
            "{{.}}",
        ] {
            let error = GoSubstitutionRenderer
                .render("test", source, &sample_data())
                .expect_err(source);

            assert!(matches!(error, Error::UnsupportedSyntax { .. }));
        }
    }
}
