use crate::languages::LanguageProfile;

pub struct KotlinProfile;

impl LanguageProfile for KotlinProfile {
    fn function_nodes(&self) -> &[&str] {
        &["function_declaration"]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[
            "if_expression",
            "for_statement",
            "while_statement",
            "do_while_statement",
            "when_expression",
            "catch_block",
            "else",
        ]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[
            "if_expression",
            "for_statement",
            "while_statement",
            "do_while_statement",
            "when_expression",
        ]
    }

    fn boolean_operators(&self) -> &[&str] {
        &["&&", "||"]
    }

    fn else_if_nodes(&self) -> &[&str] {
        // In Kotlin, `else if` is `else` token + nested `if_expression` — no dedicated node.
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &["lambda_literal"]
    }

    fn comment_nodes(&self) -> &[&str] {
        &["line_comment", "multiline_comment"]
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
        tree_sitter_kotlin_ng::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".kt", ".kts"]
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

    fn match_construct_nodes(&self) -> &[&str] {
        &["when_expression"]
    }

    fn match_arm_nodes(&self) -> &[&str] {
        &["when_entry"]
    }
}
