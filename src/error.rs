use std::fmt;

/// Errors returned by arborist analysis functions.
#[derive(Debug)]
#[non_exhaustive]
pub enum ArboristError {
    /// The specified file path does not exist.
    FileNotFound { path: String },
    /// The language identifier is not recognized.
    UnsupportedLanguage { language: String },
    /// The file extension does not map to any known language.
    UnrecognizedExtension { extension: String },
    /// The language is recognized but its compile-time feature flag is not enabled.
    LanguageNotEnabled { language: String },
    /// tree-sitter failed to parse the source (rare due to error tolerance).
    ParseError { details: String },
    /// Underlying I/O error.
    ///
    /// Non-UTF-8 files surface here as `std::io::Error` with
    /// `ErrorKind::InvalidData`, since `std::fs::read_to_string` enforces UTF-8.
    Io(std::io::Error),
}

impl fmt::Display for ArboristError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArboristError::FileNotFound { path } => {
                write!(f, "file not found: {path}")
            }
            ArboristError::UnsupportedLanguage { language } => {
                write!(f, "unsupported language: {language}")
            }
            ArboristError::UnrecognizedExtension { extension } => {
                write!(f, "unrecognized file extension: {extension}")
            }
            ArboristError::LanguageNotEnabled { language } => {
                write!(
                    f,
                    "language '{language}' is recognized but its feature flag is not enabled"
                )
            }
            ArboristError::ParseError { details } => {
                write!(f, "parse error: {details}")
            }
            ArboristError::Io(err) => write!(f, "I/O error: {err}"),
        }
    }
}

impl std::error::Error for ArboristError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ArboristError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ArboristError {
    fn from(err: std::io::Error) -> Self {
        ArboristError::Io(err)
    }
}
