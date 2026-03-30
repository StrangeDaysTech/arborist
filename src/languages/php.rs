use crate::languages::LanguageProfile;

pub struct PhpProfile;

impl LanguageProfile for PhpProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_definition", "method_declaration"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "foreach_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
            "match_expression",
            "catch_clause",
            "else_if_clause",
            "else_clause",
            "conditional_expression",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_statement",
            "for_statement",
            "foreach_statement",
            "while_statement",
            "do_statement",
            "switch_statement",
            "match_expression",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &["else_if_clause"]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["anonymous_function", "arrow_function"]
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
        tree_sitter_php::LANGUAGE_PHP.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".php"]
    }

    fn is_method(&self, node: &tree_sitter::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.kind() == "declaration_list" {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn call_nodes(&self) -> &[&str] {
        &["function_call_expression"]
    }

    fn match_construct_nodes(&self) -> &[&str] {
        &["switch_statement", "match_expression"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["case_statement", "default_statement", "match_conditional_expression", "match_default_expression"]
    }
}
