use crate::languages::LanguageProfile;

pub struct PythonProfile;

impl LanguageProfile for PythonProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_definition"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "while_statement",
            "elif_clause",
            "else_clause",
            "except_clause",
            "match_statement",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &["if_statement", "for_statement", "while_statement", "match_statement"]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["and", "or"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &["elif_clause"]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["lambda"]
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
        tree_sitter_python::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".py", ".pyi"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "class_definition" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn boolean_expression_nodes(&self) -> &[&str] {
        &["boolean_operator"]
    }

    fn call_nodes(&self) -> &[&str] {
        &["call"]
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["match_statement"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["case_clause"]
    }
}
