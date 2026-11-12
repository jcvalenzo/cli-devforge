use super::{find, Prefix};
use crate::{Error, Result};

/// Extracts the prefix and project name from a full project name.
///
/// Example: `api-catalog` -> prefix `api`, project `catalog`.
pub fn detect(full_name: &str) -> Result<(&'static Prefix, &str)> {
    let Some((prefix_name, project_name)) = full_name.split_once('-') else {
        return Err(Error::InvalidProjectName {
            name: full_name.to_string(),
        });
    };

    let prefix = find(prefix_name).ok_or_else(|| Error::UnknownPrefix {
        prefix: prefix_name.to_string(),
        full_name: full_name.to_string(),
    })?;

    Ok((prefix, project_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_valid_prefixes() {
        let cases = [
            ("api-catalog", "api", "catalog"),
            ("ms-auth", "ms", "auth"),
            ("app-blog", "app", "blog"),
            ("api-user-service", "api", "user-service"),
        ];

        for (input, expected_prefix, expected_project) in cases {
            let (prefix, project) = detect(input).expect(input);
            assert_eq!(prefix.name, expected_prefix);
            assert_eq!(project, expected_project);
        }
    }

    #[test]
    fn rejects_names_without_dash() {
        assert!(detect("apicatalog").is_err());
        assert!(detect("").is_err());
    }

    #[test]
    fn rejects_unknown_prefix() {
        assert!(detect("unknown-name").is_err());
    }

    #[test]
    fn preserves_empty_project_name() {
        let (prefix, project) = detect("api-").expect("valid prefix");
        assert_eq!(prefix.name, "api");
        assert_eq!(project, "");
    }
}
