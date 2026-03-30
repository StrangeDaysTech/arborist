#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

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
///
/// The language is determined from the file extension (e.g., `.rs` → Rust,
/// `.py` → Python). Uses default configuration (no threshold, methods included).
///
/// # Errors
///
/// - [`ArboristError::FileNotFound`] if the path does not exist.
/// - [`ArboristError::UnrecognizedExtension`] if the extension is unknown.
/// - [`ArboristError::LanguageNotEnabled`] if the language feature flag is off.
///
/// # Examples
///
/// ```no_run
/// use arborist::analyze_file;
///
/// let report = analyze_file("src/main.rs")?;
/// println!("{}: cognitive={}", report.path, report.file_cognitive);
/// for func in &report.functions {
///     println!("  {} cognitive={}", func.name, func.cognitive);
/// }
/// # Ok::<(), arborist::ArboristError>(())
/// ```
pub fn analyze_file(path: impl AsRef<Path>) -> Result<FileReport, ArboristError> {
    analyze_file_with_config(path, &AnalysisConfig::default())
}

/// Analyze a source file with custom configuration.
///
/// Like [`analyze_file`], but accepts an [`AnalysisConfig`] to control
/// threshold flagging and method inclusion.
///
/// # Errors
///
/// Same as [`analyze_file`].
///
/// # Examples
///
/// ```no_run
/// use arborist::{analyze_file_with_config, AnalysisConfig};
///
/// let config = AnalysisConfig {
///     cognitive_threshold: Some(8),
///     ..Default::default()
/// };
/// let report = analyze_file_with_config("src/lib.rs", &config)?;
/// for func in &report.functions {
///     if func.exceeds_threshold == Some(true) {
///         eprintln!("WARNING: {} has cognitive complexity {}", func.name, func.cognitive);
///     }
/// }
/// # Ok::<(), arborist::ArboristError>(())
/// ```
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
///
/// Use this when the source code is already in memory (e.g., from an editor
/// buffer or a network response). The returned [`FileReport`] will have an
/// empty `path`.
///
/// # Errors
///
/// - [`ArboristError::LanguageNotEnabled`] if the language feature flag is off.
///
/// # Examples
///
/// ```
/// use arborist::{analyze_source, Language};
///
/// let source = r#"
/// fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// "#;
///
/// let report = analyze_source(source, Language::Rust)?;
/// assert_eq!(report.functions.len(), 1);
/// assert_eq!(report.functions[0].name, "add");
/// assert_eq!(report.functions[0].cognitive, 0);
/// # Ok::<(), arborist::ArboristError>(())
/// ```
pub fn analyze_source(source: &str, language: Language) -> Result<FileReport, ArboristError> {
    analyze_source_with_config(source, language, &AnalysisConfig::default())
}

/// Analyze source code with explicit language and custom configuration.
///
/// Like [`analyze_source`], but accepts an [`AnalysisConfig`] to control
/// threshold flagging and method inclusion.
///
/// # Errors
///
/// Same as [`analyze_source`].
///
/// # Examples
///
/// ```
/// use arborist::{analyze_source_with_config, AnalysisConfig, Language};
///
/// let source = r#"
/// fn complex(x: i32) -> i32 {
///     if x > 0 {
///         if x > 10 {
///             x * 2
///         } else {
///             x + 1
///         }
///     } else {
///         0
///     }
/// }
/// "#;
///
/// let config = AnalysisConfig {
///     cognitive_threshold: Some(1),
///     ..Default::default()
/// };
/// let report = analyze_source_with_config(source, Language::Rust, &config)?;
/// assert_eq!(report.functions[0].exceeds_threshold, Some(true));
/// # Ok::<(), arborist::ArboristError>(())
/// ```
pub fn analyze_source_with_config(
    source: &str,
    language: Language,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError> {
    let (_lang, profile) = languages::profile_for_language(language)?;
    walker::walk_source(source, language, profile.as_ref(), config)
}
