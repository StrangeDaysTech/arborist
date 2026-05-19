// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

// Most tests require "rust"; Python-specific tests are individually gated.
#![cfg(feature = "rust")]

use arborist::{analyze_file, analyze_source, analyze_source_with_config, AnalysisConfig, Language};

/// Helper to get the project root directory for fixture paths.
fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/rust/{name}")
}

// --- T022: analyze_source produces identical metrics to analyze_file ---

#[test]
fn source_simple_function_matches_file() {
    let source = include_str!("fixtures/rust/simple_function.rs");
    let file_report = analyze_file(fixture_path("simple_function.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    assert_eq!(source_report.language, Language::Rust);
    assert_eq!(source_report.path, "", "analyze_source should produce empty path");
    assert_eq!(source_report.functions.len(), file_report.functions.len());

    let sf = &source_report.functions[0];
    let ff = &file_report.functions[0];
    assert_eq!(sf.name, ff.name);
    assert_eq!(sf.cognitive, ff.cognitive);
    assert_eq!(sf.cyclomatic, ff.cyclomatic);
    assert_eq!(sf.sloc, ff.sloc);
    assert_eq!(source_report.file_cognitive, file_report.file_cognitive);
    assert_eq!(source_report.file_cyclomatic, file_report.file_cyclomatic);
    assert_eq!(source_report.file_sloc, file_report.file_sloc);
}

#[test]
fn source_nested_control_flow_matches_file() {
    let source = include_str!("fixtures/rust/nested_control_flow.rs");
    let file_report = analyze_file(fixture_path("nested_control_flow.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    assert_eq!(source_report.functions.len(), file_report.functions.len());
    let sf = &source_report.functions[0];
    let ff = &file_report.functions[0];
    assert_eq!(sf.name, ff.name);
    assert_eq!(sf.cognitive, ff.cognitive);
    assert_eq!(sf.cyclomatic, ff.cyclomatic);
    assert_eq!(sf.sloc, ff.sloc);
}

#[test]
fn source_boolean_operators_matches_file() {
    let source = include_str!("fixtures/rust/boolean_operators.rs");
    let file_report = analyze_file(fixture_path("boolean_operators.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    assert_eq!(source_report.functions.len(), file_report.functions.len());
    for (sf, ff) in source_report.functions.iter().zip(file_report.functions.iter()) {
        assert_eq!(sf.name, ff.name);
        assert_eq!(sf.cognitive, ff.cognitive);
        assert_eq!(sf.cyclomatic, ff.cyclomatic);
        assert_eq!(sf.sloc, ff.sloc);
    }
}

#[test]
fn source_else_if_chain_matches_file() {
    let source = include_str!("fixtures/rust/else_if_chain.rs");
    let file_report = analyze_file(fixture_path("else_if_chain.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    let sf = &source_report.functions[0];
    let ff = &file_report.functions[0];
    assert_eq!(sf.name, ff.name);
    assert_eq!(sf.cognitive, ff.cognitive);
    assert_eq!(sf.cyclomatic, ff.cyclomatic);
    assert_eq!(sf.sloc, ff.sloc);
}

#[test]
fn source_closures_lambdas_matches_file() {
    let source = include_str!("fixtures/rust/closures_lambdas.rs");
    let file_report = analyze_file(fixture_path("closures_lambdas.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    let sf = &source_report.functions[0];
    let ff = &file_report.functions[0];
    assert_eq!(sf.name, ff.name);
    assert_eq!(sf.cognitive, ff.cognitive);
    assert_eq!(sf.cyclomatic, ff.cyclomatic);
    assert_eq!(sf.sloc, ff.sloc);
}

#[test]
fn source_recursion_matches_file() {
    let source = include_str!("fixtures/rust/recursion.rs");
    let file_report = analyze_file(fixture_path("recursion.rs")).unwrap();
    let source_report = analyze_source(source, Language::Rust).unwrap();

    let sf = &source_report.functions[0];
    let ff = &file_report.functions[0];
    assert_eq!(sf.name, ff.name);
    assert_eq!(sf.cognitive, ff.cognitive);
    assert_eq!(sf.cyclomatic, ff.cyclomatic);
    assert_eq!(sf.sloc, ff.sloc);
}

#[test]
fn source_empty_string_no_functions() {
    let report = analyze_source("", Language::Rust).unwrap();
    assert_eq!(report.functions.len(), 0);
    assert_eq!(report.file_sloc, 0);
    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 0);
}

#[test]
fn source_with_config_default_matches_no_config() {
    let source = include_str!("fixtures/rust/simple_function.rs");
    let report_no_config = analyze_source(source, Language::Rust).unwrap();
    let report_with_config =
        analyze_source_with_config(source, Language::Rust, &AnalysisConfig::default()).unwrap();

    assert_eq!(report_no_config.functions.len(), report_with_config.functions.len());
    assert_eq!(
        report_no_config.functions[0].cognitive,
        report_with_config.functions[0].cognitive
    );
    assert_eq!(
        report_no_config.functions[0].exceeds_threshold,
        report_with_config.functions[0].exceeds_threshold
    );
}

// --- I2 remediation: Python-based US2 test (spec US2:AS1) ---

#[cfg(feature = "python")]
#[test]
fn source_python_two_functions() {
    // Spec US2:AS1: "Given a Python source string containing 2 functions with known
    // complexity, When the user calls the source analysis function specifying Python,
    // Then the report contains exactly 2 function entries with correct metrics."
    let source = concat!(
        include_str!("fixtures/python/simple_function.py"),
        "\n",
        include_str!("fixtures/python/nested_control_flow.py"),
    );
    let report = analyze_source(source, Language::Python).unwrap();

    assert_eq!(report.language, Language::Python);
    assert_eq!(report.path, "", "analyze_source should produce empty path");
    assert_eq!(report.functions.len(), 2, "expected 2 functions (add + process)");

    let add = &report.functions[0];
    assert_eq!(add.name, "add");
    assert_eq!(add.cognitive, 0);
    assert_eq!(add.cyclomatic, 1);
    assert_eq!(add.sloc, 3);

    let process = &report.functions[1];
    assert_eq!(process.name, "process");
    assert_eq!(process.cognitive, 6);
    assert_eq!(process.cyclomatic, 4);
}

// --- T025: Error cases for analyze_source ---

#[test]
fn source_syntax_error_best_effort() {
    // Malformed Rust code — tree-sitter still produces a partial AST
    let source = "fn broken( { if true { } }";
    let result = analyze_source(source, Language::Rust);
    // tree-sitter is error-tolerant; it should not fail but produce best-effort results
    assert!(result.is_ok(), "syntax errors should produce best-effort results, got: {:?}", result.err());
}

#[test]
fn source_language_not_enabled() {
    // This test only makes sense when compiled WITHOUT a specific language feature.
    // Since we compile with default features (includes rust), we test with a
    // cfg-gated block that only runs when a non-default feature is disabled.
    #[cfg(not(feature = "csharp"))]
    {
        let result = analyze_source("class Foo {}", Language::CSharp);
        assert!(result.is_err());
        let err = result.unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("not enabled"),
            "should mention 'not enabled', got: {msg}"
        );
    }
}
