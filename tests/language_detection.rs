#![cfg(feature = "all")]

use arborist::{analyze_file, Language};

/// Helper to get a fixture path for a given language and filename.
fn fixture_path(lang: &str, name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/{lang}/{name}")
}

#[test]
fn detects_rust_from_rs_extension() {
    let report = analyze_file(fixture_path("rust", "simple_function.rs")).unwrap();
    assert_eq!(report.language, Language::Rust);
}

#[test]
fn detects_python_from_py_extension() {
    let report = analyze_file(fixture_path("python", "simple_function.py")).unwrap();
    assert_eq!(report.language, Language::Python);
}

#[test]
fn detects_javascript_from_js_extension() {
    let report = analyze_file(fixture_path("javascript", "simple_function.js")).unwrap();
    assert_eq!(report.language, Language::JavaScript);
}

#[test]
fn detects_typescript_from_ts_extension() {
    let report = analyze_file(fixture_path("typescript", "simple_function.ts")).unwrap();
    assert_eq!(report.language, Language::TypeScript);
}

#[test]
fn detects_java_from_java_extension() {
    let report = analyze_file(fixture_path("java", "SimpleFunction.java")).unwrap();
    assert_eq!(report.language, Language::Java);
}

#[test]
fn detects_csharp_from_cs_extension() {
    let report = analyze_file(fixture_path("csharp", "simple_function.cs")).unwrap();
    assert_eq!(report.language, Language::CSharp);
}

#[test]
fn detects_cpp_from_cpp_extension() {
    let report = analyze_file(fixture_path("cpp", "simple_function.cpp")).unwrap();
    assert_eq!(report.language, Language::Cpp);
}

#[test]
fn detects_c_from_c_extension() {
    let report = analyze_file(fixture_path("c", "simple_function.c")).unwrap();
    assert_eq!(report.language, Language::C);
}

#[test]
fn detects_go_from_go_extension() {
    let report = analyze_file(fixture_path("go", "simple_function.go")).unwrap();
    assert_eq!(report.language, Language::Go);
}

#[test]
fn detects_php_from_php_extension() {
    let report = analyze_file(fixture_path("php", "simple_function.php")).unwrap();
    assert_eq!(report.language, Language::Php);
}

#[test]
fn detects_kotlin_from_kt_extension() {
    let report = analyze_file(fixture_path("kotlin", "simple_function.kt")).unwrap();
    assert_eq!(report.language, Language::Kotlin);
}

#[test]
fn detects_swift_from_swift_extension() {
    let report = analyze_file(fixture_path("swift", "simple_function.swift")).unwrap();
    assert_eq!(report.language, Language::Swift);
}

#[test]
fn h_extension_defaults_to_c() {
    // .h files should be detected as C
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let tmp_path = format!("{manifest_dir}/tests/fixtures/c/test_header.h");

    // Create a temporary .h file
    std::fs::write(&tmp_path, "int add(int a, int b);").unwrap();
    let result = analyze_file(&tmp_path);
    // Clean up before asserting
    let _ = std::fs::remove_file(&tmp_path);

    let report = result.unwrap();
    assert_eq!(report.language, Language::C, ".h files should be detected as C");
}

#[test]
fn unknown_extension_returns_error() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let tmp_path = format!("{manifest_dir}/tests/fixtures/test_unknown.xyz");

    // Create a temporary file with unknown extension
    std::fs::write(&tmp_path, "some content").unwrap();
    let result = analyze_file(&tmp_path);
    // Clean up before asserting
    let _ = std::fs::remove_file(&tmp_path);

    assert!(
        matches!(result, Err(arborist::ArboristError::UnrecognizedExtension { .. })),
        "Unknown extension should return UnrecognizedExtension error"
    );
}
