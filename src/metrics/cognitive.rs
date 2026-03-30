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

    // Check for direct recursion
    if let Some(fn_name) = function_name
        && is_recursive_call(node, source, fn_name, profile)
    {
        *complexity += 1;
    }

    // Boolean expression sequences (SonarSource: same-operator chain = +1, each switch = +1)
    // Work at expression level (binary_expression / boolean_operator) not token level (&&/||)
    if let Some(op) = get_boolean_op(node, source, profile) {
        // Check if parent is also a boolean expression → this node is a continuation, skip
        let parent_op = node.parent().and_then(|p| get_boolean_op(&p, source, profile));
        if parent_op.is_none() {
            // Root of boolean chain: +1 for the sequence
            *complexity += 1;
            // Count operator switches in the subtree
            *complexity += count_expression_switches(node, source, profile, &op);
        }

        // Recurse into non-boolean-expression children only
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if get_boolean_op(&child, source, profile).is_none() {
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
    let child_nesting =
        if (nesting_nodes.contains(&kind) && !else_if.contains(&kind)) || lambda.contains(&kind) {
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

/// Extract the boolean operator from an expression node, if present.
/// Returns the operator text (e.g., "&&", "or") if the node is a boolean expression,
/// or None if it's not (e.g., a `binary_expression` with `+` operator).
fn get_boolean_op(node: &Node, source: &[u8], profile: &dyn LanguageProfile) -> Option<String> {
    if !profile.boolean_expression_nodes().contains(&node.kind()) {
        return None;
    }
    let boolean_ops = profile.boolean_operators();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if boolean_ops.contains(&child.kind()) {
            return child.utf8_text(source).ok().map(|s| s.to_string());
        }
    }
    None
}

/// Count operator switches within a boolean expression tree.
/// e.g., `a && b || c` has 1 switch (from && to ||).
fn count_expression_switches(
    node: &Node,
    source: &[u8],
    profile: &dyn LanguageProfile,
    current_op: &str,
) -> u64 {
    let mut switches = 0u64;
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if let Some(child_op) = get_boolean_op(&child, source, profile) {
            if child_op != current_op {
                switches += 1;
            }
            switches += count_expression_switches(&child, source, profile, &child_op);
        }
    }
    switches
}

/// Check if a node is a direct recursive call to the current function.
fn is_recursive_call(
    node: &Node,
    source: &[u8],
    function_name: &str,
    profile: &dyn LanguageProfile,
) -> bool {
    if profile.call_nodes().contains(&node.kind())
        && let Some(func_node) = node.child_by_field_name(profile.call_function_field())
    {
        let text = func_node.utf8_text(source).unwrap_or("");
        return text == function_name;
    }
    false
}
