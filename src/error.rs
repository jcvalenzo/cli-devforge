use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid project name {name:?}: must follow <prefix>-<name> convention")]
    InvalidProjectName { name: String },

    #[error("unknown prefix {prefix:?} in {full_name:?}")]
    UnknownPrefix { prefix: String, full_name: String },

    #[error("language {lang:?} not supported for prefix {prefix:?} (available: [{available}])")]
    UnsupportedLanguage {
        lang: String,
        prefix: String,
        available: String,
    },

    #[error("no templates found for {prefix}/{lang}")]
    NoTemplates { prefix: String, lang: String },

    #[error("listing templates: {0}")]
    ListTemplates(String),

    #[error("reading template {path}: {source}")]
    ReadTemplate {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("template {path}: unknown field {field:?}")]
    UnknownField { path: String, field: String },

    #[error("template {path}: unsupported syntax {snippet:?}")]
    UnsupportedSyntax { path: String, snippet: String },

    #[error("creating directory {path}: {source}")]
    CreateDir {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("writing {path}: {source}")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
