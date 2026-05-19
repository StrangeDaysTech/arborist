// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

#![cfg(feature = "rust")]

use arborist::{AnalysisConfig, Language, analyze_file_with_config, analyze_source_with_config};

/// Helper to get the project root directory for fixture paths.
fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/rust/{name}")
}

#[test]
fn threshold_flags_complex_function_above() {
    // classify has cognitive=9, threshold=8 → exceeds_threshold = Some(true)
    let config = AnalysisConfig {
        cognitive_threshold: Some(8),
        ..Default::default()
    };
    let report = analyze_file_with_config(fixture_path("else_if_chain.rs"), &config).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "classify");
    assert_eq!(
        f.exceeds_threshold,
        Some(true),
        "cognitive {} should exceed threshold 8",
        f.cognitive
    );
}

#[test]
fn threshold_flags_simple_function_below() {
    // add has cognitive=0, threshold=8 → exceeds_threshold = Some(false)
    let config = AnalysisConfig {
        cognitive_threshold: Some(8),
        ..Default::default()
    };
    let report = analyze_file_with_config(fixture_path("simple_function.rs"), &config).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "add");
    assert_eq!(
        f.exceeds_threshold,
        Some(false),
        "cognitive {} should not exceed threshold 8",
        f.cognitive
    );
}

#[test]
fn threshold_mixed_file_flags_each_function() {
    // boolean_operators.rs: check_same=2, check_mixed=3
    // With threshold=1: both exceed
    let config = AnalysisConfig {
        cognitive_threshold: Some(1),
        ..Default::default()
    };
    let report = analyze_file_with_config(fixture_path("boolean_operators.rs"), &config).unwrap();
    assert_eq!(report.functions.len(), 2);

    for f in &report.functions {
        assert_eq!(
            f.exceeds_threshold,
            Some(true),
            "{} cognitive={} should exceed threshold 1",
            f.name,
            f.cognitive
        );
    }
}

#[test]
fn threshold_exact_boundary_does_not_exceed() {
    // process has cognitive=6, threshold=6 → exceeds_threshold = Some(false) because 6 > 6 is false
    let config = AnalysisConfig {
        cognitive_threshold: Some(6),
        ..Default::default()
    };
    let report = analyze_file_with_config(fixture_path("nested_control_flow.rs"), &config).unwrap();
    let f = &report.functions[0];
    assert_eq!(f.name, "process");
    assert_eq!(
        f.exceeds_threshold,
        Some(false),
        "cognitive {} at exact threshold 6 should not exceed",
        f.cognitive
    );
}

#[test]
fn no_threshold_returns_none() {
    // Default config has no threshold → exceeds_threshold = None
    let config = AnalysisConfig::default();
    assert!(config.cognitive_threshold.is_none());

    let report = analyze_file_with_config(fixture_path("else_if_chain.rs"), &config).unwrap();
    let f = &report.functions[0];
    assert_eq!(
        f.exceeds_threshold, None,
        "without threshold configured, exceeds_threshold should be None"
    );
}

#[test]
fn threshold_with_analyze_source() {
    let source = r#"
fn simple() -> i32 {
    42
}

fn complex(x: i32) -> i32 {
    if x > 0 {
        if x > 10 {
            if x > 100 {
                return x;
            }
        }
    }
    0
}
"#;
    let config = AnalysisConfig {
        cognitive_threshold: Some(3),
        ..Default::default()
    };
    let report = analyze_source_with_config(source, Language::Rust, &config).unwrap();
    assert_eq!(report.functions.len(), 2);

    let simple = report
        .functions
        .iter()
        .find(|f| f.name == "simple")
        .unwrap();
    assert_eq!(
        simple.exceeds_threshold,
        Some(false),
        "simple should not exceed threshold 3"
    );

    let complex = report
        .functions
        .iter()
        .find(|f| f.name == "complex")
        .unwrap();
    assert_eq!(
        complex.exceeds_threshold,
        Some(true),
        "complex should exceed threshold 3"
    );
}

#[test]
fn threshold_zero_all_functions_with_any_complexity_exceed() {
    // threshold=0: any function with cognitive > 0 exceeds
    let config = AnalysisConfig {
        cognitive_threshold: Some(0),
        ..Default::default()
    };
    let report = analyze_file_with_config(fixture_path("simple_function.rs"), &config).unwrap();
    let f = &report.functions[0];
    assert_eq!(f.name, "add");
    assert_eq!(f.cognitive, 0);
    assert_eq!(
        f.exceeds_threshold,
        Some(false),
        "cognitive 0 with threshold 0: 0 > 0 is false"
    );

    let report2 = analyze_file_with_config(fixture_path("recursion.rs"), &config).unwrap();
    let f2 = &report2.functions[0];
    assert_eq!(
        f2.exceeds_threshold,
        Some(true),
        "cognitive {} with threshold 0 should exceed",
        f2.cognitive
    );
}
