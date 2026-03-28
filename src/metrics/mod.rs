pub mod cognitive;
pub mod cyclomatic;
pub mod loc;

use crate::languages::LanguageProfile;
use tree_sitter::Node;

/// Result of computing all three metrics for a function node.
pub struct MetricsResult {
    pub cognitive: u64,
    pub cyclomatic: u64,
    pub sloc: u64,
}

/// Compute all three metrics for a function body node.
pub fn compute_metrics(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    function_name: Option<&str>,
) -> MetricsResult {
    MetricsResult {
        cognitive: cognitive::compute_cognitive(node, source, profile, function_name),
        cyclomatic: cyclomatic::compute_cyclomatic(node, source, profile),
        sloc: loc::compute_sloc(node, source, profile),
    }
}
