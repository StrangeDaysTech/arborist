use crate::languages::LanguageProfile;

pub struct JavaProfile;

impl LanguageProfile for JavaProfile {
    fn function_nodes(&self) -> &[&str] {
        &["method_declaration", "constructor_declaration"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "enhanced_for_statement",
            "while_statement",
            "do_statement",
            "switch_expression",
            "catch_clause",
            "ternary_expression",
            "else",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "enhanced_for_statement",
            "while_statement",
            "do_statement",
            "switch_expression",
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
        tree_sitter_java::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".java"]
    }

    fn is_method(&self, _node: &tree_sitter::Node) -> bool {
        true
    }

    fn call_nodes(&self) -> &[&str] {
        &["method_invocation"]
    }

    fn call_function_field(&self) -> &str {
        "name"
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_expression"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["switch_block_statement_group"]
    }
}
