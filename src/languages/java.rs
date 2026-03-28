use crate::languages::LanguageProfile;

pub struct JavaProfile;

impl LanguageProfile for JavaProfile {
    fn function_nodes(&self) -> &[&str] {
        &[]
    }

    fn control_flow_nodes(&self) -> &[&str] {
        &[]
    }

    fn nesting_nodes(&self) -> &[&str] {
        &[]
    }

    fn boolean_operators(&self) -> &[&str] {
        &[]
    }

    fn else_if_nodes(&self) -> &[&str] {
        &[]
    }

    fn lambda_nodes(&self) -> &[&str] {
        &[]
    }

    fn comment_nodes(&self) -> &[&str] {
        &[]
    }

    fn extract_function_name(
        &self,
        _node: &tree_sitter::Node,
        _source: &[u8],
    ) -> Option<String> {
        None
    }

    fn parser_language(&self) -> tree_sitter::Language {
        tree_sitter_java::LANGUAGE.into()
    }

    fn extensions(&self) -> &[&str] {
        &[".java"]
    }

    fn is_method(&self, _node: &tree_sitter::Node) -> bool {
        false
    }
}
