# Public API Contract: Arborist v0.1.0

**Branch**: `001-code-metrics-library` | **Date**: 2026-03-27

This document defines the public API surface of the `arborist` crate. These are the items exposed from `lib.rs` that constitute the semver-governed contract.

## Public Functions

### `analyze_file`

```rust
/// Analyze a source file, auto-detecting language from its extension.
///
/// Returns a `FileReport` with per-function metrics and file-level aggregates.
///
/// # Errors
/// - `ArboristError::FileNotFound` if the path doesn't exist
/// - `ArboristError::UnrecognizedExtension` if the extension isn't mapped
/// - `ArboristError::LanguageNotEnabled` if the language's feature flag is off
/// - `ArboristError::Io` for other I/O errors
pub fn analyze_file(path: impl AsRef<Path>) -> Result<FileReport, ArboristError>
```

### `analyze_file_with_config`

```rust
/// Analyze a source file with custom configuration.
///
/// Same as `analyze_file` but applies the given `AnalysisConfig`
/// (e.g., cognitive threshold).
pub fn analyze_file_with_config(
    path: impl AsRef<Path>,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError>
```

### `analyze_source`

```rust
/// Analyze source code provided as a string, with explicit language.
///
/// # Errors
/// - `ArboristError::UnsupportedLanguage` if the language is not recognized
/// - `ArboristError::LanguageNotEnabled` if the language's feature flag is off
pub fn analyze_source(source: &str, language: Language) -> Result<FileReport, ArboristError>
```

### `analyze_source_with_config`

```rust
/// Analyze source code with explicit language and custom configuration.
pub fn analyze_source_with_config(
    source: &str,
    language: Language,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError>
```

## Public Types

### `FileReport`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileReport {
    pub path: String,
    pub language: Language,
    pub functions: Vec<FunctionMetrics>,
    pub file_cognitive: u64,
    pub file_cyclomatic: u64,
    pub file_sloc: u64,
}
```

### `FunctionMetrics`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionMetrics {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub cognitive: u64,
    pub cyclomatic: u64,
    pub sloc: u64,
    pub exceeds_threshold: Option<bool>,
}
```

### `Language`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Java,
    CSharp,
    Cpp,
    C,
    Go,
    Php,
}
```

### `AnalysisConfig`

```rust
#[derive(Debug, Clone, Default)]
pub struct AnalysisConfig {
    pub cognitive_threshold: Option<u64>,
    pub include_methods: bool, // default: true
}
```

### `ArboristError`

```rust
#[derive(Debug)]
pub enum ArboristError {
    FileNotFound { path: String },
    UnsupportedLanguage { language: String },
    UnrecognizedExtension { extension: String },
    LanguageNotEnabled { language: String },
    ParseError { details: String },
    Io(std::io::Error),
}

impl std::fmt::Display for ArboristError { /* ... */ }
impl std::error::Error for ArboristError { /* ... */ }
```

## Re-exports from `lib.rs`

```rust
pub use types::{FileReport, FunctionMetrics, Language, AnalysisConfig};
pub use error::ArboristError;
pub use languages::LanguageProfile; // For consumers adding custom languages (future)
```

## Semver Guarantees

- All items listed above are part of the public API and follow semver
- Adding new `Language` variants is a MINOR change (non-exhaustive enum)
- Adding new fields to `FileReport` or `FunctionMetrics` is a MINOR change (structs implement `#[non_exhaustive]` or consumers use `..` patterns)
- Removing or renaming any public item is a MAJOR change
- Adding new `ArboristError` variants is a MINOR change (non-exhaustive enum)

## Not Public (internal)

- `walker.rs` internals
- `metrics/cognitive.rs`, `metrics/cyclomatic.rs`, `metrics/loc.rs` internals
- Individual `LanguageProfile` implementations (e.g., `languages::rust::RustProfile`)
- tree-sitter types are not exposed in the public API
