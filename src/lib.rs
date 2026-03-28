#![forbid(unsafe_code)]

pub mod error;
pub mod languages;
pub mod metrics;
pub mod types;
pub mod walker;

pub use error::ArboristError;
pub use languages::LanguageProfile;
pub use types::{AnalysisConfig, FileReport, FunctionMetrics, Language};

use std::path::Path;

/// Analyze a source file, auto-detecting language from its extension.
pub fn analyze_file(path: impl AsRef<Path>) -> Result<FileReport, ArboristError> {
    analyze_file_with_config(path, &AnalysisConfig::default())
}

/// Analyze a source file with custom configuration.
pub fn analyze_file_with_config(
    path: impl AsRef<Path>,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ArboristError::FileNotFound {
            path: path.display().to_string(),
        });
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .ok_or_else(|| ArboristError::UnrecognizedExtension {
            extension: String::new(),
        })?;

    let (language, profile) = languages::profile_for_extension(ext)?;
    let source = std::fs::read_to_string(path)?;

    let mut report = walker::walk_source(&source, language, profile.as_ref(), config)?;
    report.path = path.display().to_string();
    Ok(report)
}

/// Analyze source code provided as a string, with explicit language.
pub fn analyze_source(source: &str, language: Language) -> Result<FileReport, ArboristError> {
    analyze_source_with_config(source, language, &AnalysisConfig::default())
}

/// Analyze source code with explicit language and custom configuration.
pub fn analyze_source_with_config(
    source: &str,
    language: Language,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError> {
    let (_lang, profile) = languages::profile_for_language(language)?;
    walker::walk_source(source, language, profile.as_ref(), config)
}
