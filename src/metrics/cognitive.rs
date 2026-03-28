use crate::languages::LanguageProfile;
use tree_sitter::Node;

/// Compute cognitive complexity for a function body following the SonarSource spec.
///
/// Rules:
/// 1. +1 for each control flow node
/// 2. Nested control flow adds (nesting_level) instead of just +1
/// 3. Same-operator boolean sequences count as +1; operator switches add +1
/// 4. else-if is flat (no nesting increment)
/// 5. Lambdas/closures increment nesting level
/// 6. +1 for direct recursive calls
pub fn compute_cognitive(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    function_name: Option<&str>,
) -> u64 {
    let mut complexity: u64 = 0;
    walk_cognitive(node, source, profile, 0, function_name, &mut complexity);
    complexity
}

fn walk_cognitive(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    nesting: u64,
    function_name: Option<&str>,
    complexity: &mut u64,
) {
    let kind = node.kind();
    let control_flow = profile.control_flow_nodes();
    let nesting_nodes = profile.nesting_nodes();
    let else_if = profile.else_if_nodes();
    let lambda = profile.lambda_nodes();
    let boolean_ops = profile.boolean_operators();

    // Check for direct recursion
    if let Some(fn_name) = function_name {
        if is_recursive_call(node, source, fn_name, profile) {
            *complexity += 1;
        }
    }

    // Boolean operator sequences
    if boolean_ops.contains(&kind) {
        // Only count at the top of a boolean chain.
        // If parent is the same boolean operator, skip (already counted).
        let parent_kind = node.parent().map(|p| p.kind());
        let is_continuation = parent_kind.is_some_and(|pk| pk == kind);

        if !is_continuation {
            // +1 for a new boolean sequence
            *complexity += 1;

            // Count operator switches within this sequence
            *complexity += count_operator_switches(node, boolean_ops);
        }

        // Don't recurse into children for boolean — handled by switch counting
        // But we do need to visit non-boolean children
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if !boolean_ops.contains(&child.kind()) {
                walk_cognitive(&child, source, profile, nesting, function_name, complexity);
            }
        }
        return;
    }

    // Control flow nodes
    if control_flow.contains(&kind) {
        if else_if.contains(&kind) {
            // else-if is flat: +1 but no nesting increment
            *complexity += 1;
        } else if nesting_nodes.contains(&kind) {
            // Nesting: +1 + nesting_level
            *complexity += 1 + nesting;
        } else {
            // Simple control flow: +1
            *complexity += 1;
        }
    }

    // Determine if this node increases nesting for children
    let child_nesting = if nesting_nodes.contains(&kind) && !else_if.contains(&kind) {
        nesting + 1
    } else if lambda.contains(&kind) {
        nesting + 1
    } else {
        nesting
    };

    // Skip nested functions — they get their own metrics
    if profile.function_nodes().contains(&kind) && nesting > 0 {
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        walk_cognitive(&child, source, profile, child_nesting, function_name, complexity);
    }
}

/// Count operator switches within a boolean expression tree.
/// e.g., `a && b || c` has 1 switch (from && to ||).
fn count_operator_switches(node: &Node, boolean_ops: &[&str]) -> u64 {
    let mut switches = 0u64;
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if boolean_ops.contains(&child.kind()) && child.kind() != node.kind() {
            switches += 1;
            // Recurse to find further switches
            switches += count_operator_switches(&child, boolean_ops);
        } else if boolean_ops.contains(&child.kind()) {
            // Same operator, continue chain
            switches += count_operator_switches(&child, boolean_ops);
        }
    }

    switches
}

/// Check if a node is a direct recursive call to the current function.
fn is_recursive_call(
    node: &Node,
    source: &[u8],
    function_name: &str,
    _profile: &dyn LanguageProfile,
) -> bool {
    if node.kind() == "call_expression" || node.kind() == "call" {
        if let Some(func_node) = node.child_by_field_name("function") {
            let text = func_node
                .utf8_text(source)
                .unwrap_or("");
            return text == function_name;
        }
    }
    false
}
