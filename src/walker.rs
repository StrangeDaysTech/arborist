use crate::error::ArboristError;
use crate::languages::LanguageProfile;
use crate::metrics;
use crate::types::{AnalysisConfig, FileReport, FunctionMetrics, Language};

/// Walk the AST of parsed source code and produce a `FileReport`.
pub fn walk_source(
    source: &str,
    language: Language,
    profile: &dyn LanguageProfile,
    config: &AnalysisConfig,
) -> Result<FileReport, ArboristError> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&profile.parser_language())
        .map_err(|e| ArboristError::ParseError {
            details: format!("failed to set parser language: {e}"),
        })?;

    let tree = parser.parse(source, None).ok_or_else(|| ArboristError::ParseError {
        details: "tree-sitter returned no parse tree".to_string(),
    })?;

    let root = tree.root_node();
    let source_bytes = source.as_bytes();

    // Collect function nodes
    let mut functions = Vec::new();
    collect_functions(
        &root,
        source_bytes,
        profile,
        config,
        &mut functions,
    );

    // Sort by start line
    functions.sort_by_key(|f| f.start_line);

    // Aggregate file-level metrics
    let file_cognitive: u64 = functions.iter().map(|f| f.cognitive).sum();
    let file_cyclomatic: u64 = functions.iter().map(|f| f.cyclomatic).sum();
    let file_sloc = metrics::loc::compute_file_sloc(&root, source_bytes, profile);

    Ok(FileReport {
        path: String::new(),
        language,
        functions,
        file_cognitive,
        file_cyclomatic,
        file_sloc,
    })
}

fn collect_functions(
    node: &tree_sitter::Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    config: &AnalysisConfig,
    functions: &mut Vec<FunctionMetrics>,
) {
    let kind = node.kind();

    if profile.function_nodes().contains(&kind) {
        // Check include_methods filter
        if !config.include_methods && profile.is_method(node) {
            // Skip methods when include_methods is false, but still recurse
            // to find nested top-level functions (unlikely but possible)
        } else if let Some(name) = profile.extract_function_name(node, source) {
            let m = metrics::compute_metrics(node, source, profile, Some(&name));

            let exceeds_threshold = config.cognitive_threshold.map(|t| m.cognitive > t);

            functions.push(FunctionMetrics {
                name,
                start_line: node.start_position().row + 1, // 1-based
                end_line: node.end_position().row + 1,
                cognitive: m.cognitive,
                cyclomatic: m.cyclomatic,
                sloc: m.sloc,
                exceeds_threshold,
            });

            // Don't recurse into this function's children for more functions
            // (nested functions inside a function are not top-level)
            return;
        }
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_functions(&child, source, profile, config, functions);
    }
}
