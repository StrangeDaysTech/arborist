#[cfg(feature = "rust")]
#[test]
fn enabled_rust_works() {
    let result = arborist::analyze_source("fn foo() {}", arborist::Language::Rust);
    assert!(result.is_ok(), "Rust analysis should work when feature is enabled");
}

#[cfg(feature = "python")]
#[test]
fn enabled_python_works() {
    let result = arborist::analyze_source("def foo(): pass", arborist::Language::Python);
    assert!(result.is_ok(), "Python analysis should work when feature is enabled");
}

#[cfg(not(feature = "csharp"))]
#[test]
fn disabled_csharp_returns_error() {
    let result = arborist::analyze_source("class Foo {}", arborist::Language::CSharp);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}

#[cfg(not(feature = "cpp"))]
#[test]
fn disabled_cpp_returns_error() {
    let result = arborist::analyze_source("int main() {}", arborist::Language::Cpp);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}

#[cfg(not(feature = "c"))]
#[test]
fn disabled_c_returns_error() {
    let result = arborist::analyze_source("int add(int a, int b) { return a + b; }", arborist::Language::C);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}

#[cfg(not(feature = "php"))]
#[test]
fn disabled_php_returns_error() {
    let result = arborist::analyze_source("<?php function foo() {}", arborist::Language::Php);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}

#[cfg(feature = "kotlin")]
#[test]
fn enabled_kotlin_works() {
    let result = arborist::analyze_source("fun foo() {}", arborist::Language::Kotlin);
    assert!(result.is_ok(), "Kotlin analysis should work when feature is enabled");
}

#[cfg(not(feature = "kotlin"))]
#[test]
fn disabled_kotlin_returns_error() {
    let result = arborist::analyze_source("fun foo() {}", arborist::Language::Kotlin);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}

#[cfg(feature = "swift")]
#[test]
fn enabled_swift_works() {
    let result = arborist::analyze_source("func foo() {}", arborist::Language::Swift);
    assert!(result.is_ok(), "Swift analysis should work when feature is enabled");
}

#[cfg(not(feature = "swift"))]
#[test]
fn disabled_swift_returns_error() {
    let result = arborist::analyze_source("func foo() {}", arborist::Language::Swift);
    assert!(matches!(result, Err(arborist::ArboristError::LanguageNotEnabled { .. })));
}
