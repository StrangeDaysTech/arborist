use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Supported programming languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
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

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
            Language::Python => write!(f, "Python"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Java => write!(f, "Java"),
            Language::CSharp => write!(f, "C#"),
            Language::Cpp => write!(f, "C++"),
            Language::C => write!(f, "C"),
            Language::Go => write!(f, "Go"),
            Language::Php => write!(f, "PHP"),
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" => Ok(Language::Rust),
            "python" => Ok(Language::Python),
            "javascript" | "js" => Ok(Language::JavaScript),
            "typescript" | "ts" => Ok(Language::TypeScript),
            "java" => Ok(Language::Java),
            "csharp" | "c#" => Ok(Language::CSharp),
            "cpp" | "c++" => Ok(Language::Cpp),
            "c" => Ok(Language::C),
            "go" => Ok(Language::Go),
            "php" => Ok(Language::Php),
            _ => Err(format!("Unknown language: {s}")),
        }
    }
}

/// Metrics for a single function or method.
///
/// Closures and lambdas do not produce their own entries; they contribute
/// to the metrics of their containing function.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionMetrics {
    /// Function or method name (e.g., `"process"` or `"MyStruct::method"`).
    pub name: String,
    /// 1-based start line number.
    pub start_line: usize,
    /// 1-based end line number.
    pub end_line: usize,
    /// Cognitive complexity (SonarSource algorithm).
    pub cognitive: u64,
    /// Cyclomatic complexity (McCabe).
    pub cyclomatic: u64,
    /// Source lines of code within the function.
    pub sloc: u64,
    /// `Some(true)` if cognitive complexity exceeds configured threshold,
    /// `Some(false)` if within threshold, `None` if no threshold configured.
    pub exceeds_threshold: Option<bool>,
}

/// Analysis report for a complete source file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileReport {
    /// File path (empty string for in-memory analysis).
    pub path: String,
    /// Detected or specified language.
    pub language: Language,
    /// Functions found, ordered by `start_line` ascending.
    pub functions: Vec<FunctionMetrics>,
    /// Sum of all functions' cognitive complexity.
    pub file_cognitive: u64,
    /// Sum of all functions' cyclomatic complexity.
    pub file_cyclomatic: u64,
    /// Total source lines in the entire file (includes top-level code).
    pub file_sloc: u64,
}

/// User-configurable analysis parameters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// When set, populates `exceeds_threshold` on each `FunctionMetrics`.
    pub cognitive_threshold: Option<u64>,
    /// Whether to include class/struct methods (default: `true`).
    pub include_methods: bool,
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            cognitive_threshold: None,
            include_methods: true,
        }
    }
}
