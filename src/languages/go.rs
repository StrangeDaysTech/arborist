use crate::languages::LanguageProfile;

pub struct GoProfile;

impl LanguageProfile for GoProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_declaration", "method_declaration"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "expression_switch_statement",
            "type_switch_statement",
            "select_statement",
            "else",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "expression_switch_statement",
            "type_switch_statement",
            "select_statement",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["func_literal"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["comment"]
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
        tree_sitter_go::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".go"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        node.kind() == "method_declaration"
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["expression_switch_statement", "type_switch_statement", "select_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["expression_case", "default_case", "type_case", "communication_case"]
    }
}
