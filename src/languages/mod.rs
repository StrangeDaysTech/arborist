use crate::error::ArboristError;
use crate::types::Language;

#[cfg(feature = "rust")]
pub mod rust;
#[cfg(feature = "python")]
pub mod python;
#[cfg(feature = "javascript")]
pub mod javascript;
#[cfg(feature = "typescript")]
pub mod typescript;
#[cfg(feature = "java")]
pub mod java;
#[cfg(feature = "csharp")]
pub mod csharp;
#[cfg(feature = "cpp")]
pub mod cpp;
#[cfg(feature = "c")]
pub mod c;
#[cfg(feature = "go")]
pub mod go;
#[cfg(feature = "php")]
pub mod php;

/// Trait that defines how a language's AST maps to control-flow concepts.
///
/// Each supported language implements this trait. The walker and metric
/// calculators are generic — they operate on the slices returned here.
/// Adding a new language means implementing this trait only; no changes
/// to core metric logic are needed.
pub trait LanguageProfile {
    /// AST node types that define function/method boundaries.
    fn function_nodes(&self) -> &[&str];

    /// AST node types that increment complexity (+1).
    fn control_flow_nodes(&self) -> &[&str];

    /// AST node types that increment the nesting level.
    fn nesting_nodes(&self) -> &[&str];

    /// Boolean operator node types.
    fn boolean_operators(&self) -> &[&str];

    /// Nodes treated as flat (e.g., `else if` — no nesting increment).
    fn else_if_nodes(&self) -> &[&str];

    /// Closure/lambda node types (increment nesting).
    fn lambda_nodes(&self) -> &[&str];

    /// Comment node types (excluded from SLOC).
    fn comment_nodes(&self) -> &[&str];

    /// Extract the function name from an AST node.
    fn extract_function_name(
        &self,
        node: &tree_sitter::Node,
        source: &[u8],
    ) -> Option<String>;

    /// Get the tree-sitter `Language` for parsing.
    fn parser_language(&self) -> tree_sitter::Language;

    /// File extensions associated with this language.
    fn extensions(&self) -> &[&str];

    /// Whether a function node represents a method (inside a class/impl block).
    fn is_method(&self, node: &tree_sitter::Node) -> bool;
}

/// Detect language from a file extension and return the corresponding profile.
///
/// Returns `Err(UnrecognizedExtension)` if the extension is unknown, or
/// `Err(LanguageNotEnabled)` if the language is known but its feature is off.
pub fn profile_for_extension(ext: &str) -> Result<(Language, Box<dyn LanguageProfile>), ArboristError> {
    let ext_lower = ext.to_lowercase();
    let ext_ref = ext_lower.as_str();

    // First, identify which language this extension belongs to
    let language = match ext_ref {
        "rs" => Some(Language::Rust),
        "py" | "pyi" => Some(Language::Python),
        "js" | "jsx" | "mjs" | "cjs" => Some(Language::JavaScript),
        "ts" | "tsx" | "mts" | "cts" => Some(Language::TypeScript),
        "java" => Some(Language::Java),
        "cs" => Some(Language::CSharp),
        "cpp" | "cc" | "cxx" | "hpp" | "hxx" | "hh" => Some(Language::Cpp),
        "c" | "h" => Some(Language::C),
        "go" => Some(Language::Go),
        "php" => Some(Language::Php),
        _ => None,
    };

    let language = language.ok_or_else(|| ArboristError::UnrecognizedExtension {
        extension: ext.to_string(),
    })?;

    profile_for_language(language)
}

/// Get the profile for a known language.
///
/// Returns `Err(LanguageNotEnabled)` if the feature flag is not compiled in.
pub fn profile_for_language(language: Language) -> Result<(Language, Box<dyn LanguageProfile>), ArboristError> {
    let profile: Box<dyn LanguageProfile> = match language {
        #[cfg(feature = "rust")]
        Language::Rust => Box::new(rust::RustProfile),
        #[cfg(feature = "python")]
        Language::Python => Box::new(python::PythonProfile),
        #[cfg(feature = "javascript")]
        Language::JavaScript => Box::new(javascript::JavaScriptProfile),
        #[cfg(feature = "typescript")]
        Language::TypeScript => Box::new(typescript::TypeScriptProfile),
        #[cfg(feature = "java")]
        Language::Java => Box::new(java::JavaProfile),
        #[cfg(feature = "csharp")]
        Language::CSharp => Box::new(csharp::CSharpProfile),
        #[cfg(feature = "cpp")]
        Language::Cpp => Box::new(cpp::CppProfile),
        #[cfg(feature = "c")]
        Language::C => Box::new(c::CProfile),
        #[cfg(feature = "go")]
        Language::Go => Box::new(go::GoProfile),
        #[cfg(feature = "php")]
        Language::Php => Box::new(php::PhpProfile),

        // When the feature is not enabled, fall through to LanguageNotEnabled
        #[allow(unreachable_patterns)]
        _ => {
            return Err(ArboristError::LanguageNotEnabled {
                language: language.to_string(),
            });
        }
    };

    Ok((language, profile))
}
