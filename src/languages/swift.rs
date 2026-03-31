use crate::languages::LanguageProfile;

pub struct SwiftProfile;

impl LanguageProfile for SwiftProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_declaration"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "while_statement",
            "repeat_while_statement",
            "switch_statement",
            "guard_statement",
            "catch_block",
            "else",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "while_statement",
            "repeat_while_statement",
            "switch_statement",
            "guard_statement",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        // In Swift, `else if` is `else` token + nested `if_statement` — no dedicated node.
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["lambda_literal"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment", "multiline_comment"]
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
        tree_sitter_swift::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".swift"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "class_body" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn boolean_expression_nodes(&self) -> &[&str] {
        // Swift grammar uses dedicated conjunction/disjunction expression nodes
        &["conjunction_expression", "disjunction_expression"]
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["switch_entry"]
    }
}
