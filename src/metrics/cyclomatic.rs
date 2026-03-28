use crate::languages::LanguageProfile;
use tree_sitter::Node;

/// Compute cyclomatic complexity for a function body.
///
/// Starts at 1 (base path). Each decision point adds +1:
/// if, else if, for, while, do-while, match/switch arm,
/// catch/except, &&, ||, ternary operator.
///
/// Note: `else` is NOT a decision point and does not increment cyclomatic complexity.
pub fn compute_cyclomatic(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
) -> u64 {
    let mut complexity: u64 = 1; // base path
    walk_cyclomatic(node, source, profile, &mut complexity);
    complexity
}

fn walk_cyclomatic(
    node: &Node,
    _source: &[u8],
    profile: &dyn LanguageProfile,
    complexity: &mut u64,
) {
    let kind = node.kind();
    let control_flow = profile.control_flow_nodes();
    let boolean_ops = profile.boolean_operators();

    // Count control flow nodes as decision points, but NOT any else clauses.
    // `else` is not a decision point in McCabe cyclomatic complexity.
    // `else if` is handled by counting the inner `if` — the else_clause wrapper
    // itself is never a decision point.
    if control_flow.contains(&kind) && !is_any_else(kind) {
        *complexity += 1;
    }

    if boolean_ops.contains(&kind) {
        *complexity += 1;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_cyclomatic(&child, _source, profile, complexity);
    }
}

/// Check if a node kind is any form of `else` clause.
/// No else clause is a decision point in cyclomatic complexity.
fn is_any_else(kind: &str) -> bool {
    matches!(kind, "else_clause" | "else_statement" | "else")
}
