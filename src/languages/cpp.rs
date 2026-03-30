use crate::languages::LanguageProfile;

/// Recursively find a `function_declarator` inside a node tree
/// (handles pointer_declarator, reference_declarator wrappers).
fn find_function_declarator_name(node: &tree_sitter::Node, source: &[u8]) -> Option<String> {
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "function_declarator" {
            return child
                .child_by_field_name("declarator")
                .and_then(|n| n.utf8_text(source).ok())
                .map(|s| s.to_string());
        }
        if (child.kind() == "pointer_declarator" || child.kind() == "reference_declarator")
            && let Some(name) = find_function_declarator_name(&child, source)
        {
            return Some(name);
        }
    }
    None
}

pub struct CppProfile;

impl LanguageProfile for CppProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_definition"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_range_loop",
            "while_statement",
            "do_statement",
            "switch_statement",
            "catch_clause",
            "else_clause",
            "conditional_expression",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "for_range_loop",
            "while_statement",
            "do_statement",
            "switch_statement",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["lambda_expression"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment"]
    }

    fn extract_function_name(
        &self,
        node: &tree_sitter::Node,
        source: &[u8],
    ) -> Option<String> {
        find_function_declarator_name(node, source)
    }

    fn parser_language(&self) -> tree_sitter::Language {
        tree_sitter_cpp::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".cpp", ".cc", ".cxx", ".hpp", ".hxx", ".hh"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "field_declaration_list" {
                return true;
            }
            current = parent.parent();
        }
        false
    }
}
