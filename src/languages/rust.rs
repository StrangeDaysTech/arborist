use crate::languages::LanguageProfile;

pub struct RustProfile;

impl LanguageProfile for RustProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_item"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_expression",
            "for_expression",
            "while_expression",
            "loop_expression",
            "match_expression",
            "else_clause",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_expression",
            "for_expression",
            "while_expression",
            "loop_expression",
            "match_expression",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["closure_expression"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["line_comment", "block_comment"]
    }

    fn extract_function_name(
        &self,
        node: &tree_sitter::Node,
        source: &[u8],
    ) -> Option<String> {
        node.child_by_field_name("name")
            .and_then(|n| n.utf8_text(source).ok())
            .map(|s| s.to_string())
    }

    fn parser_language(&self) -> tree_sitter::Language {
        tree_sitter_rust::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".rs"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "impl_item" {
                return true;
            }
            current = parent.parent();
        }
        false
    }
}
