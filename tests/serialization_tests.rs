// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

#![cfg(feature = "rust")]

use arborist::{
    analyze_file, analyze_file_with_config, analyze_source, AnalysisConfig, FileReport,
    FunctionMetrics, Language,
};

fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/rust/{name}")
}

#[test]
fn file_report_round_trip_json() {
    let report = analyze_file(fixture_path("nested_control_flow.rs")).unwrap();

    let json = serde_json::to_string_pretty(&report).unwrap();
    let deserialized: FileReport = serde_json::from_str(&json).unwrap();

    assert_eq!(report, deserialized);
}

#[test]
fn file_report_with_threshold_round_trip() {
    let config = AnalysisConfig {
        cognitive_threshold: Some(3),
        include_methods: true,
    };
    let report = analyze_file_with_config(fixture_path("nested_control_flow.rs"), &config).unwrap();

    // Verify threshold is populated before round-trip
    assert!(report.functions.iter().all(|f| f.exceeds_threshold.is_some()));

    let json = serde_json::to_string(&report).unwrap();
    let deserialized: FileReport = serde_json::from_str(&json).unwrap();

    assert_eq!(report, deserialized);
}

#[test]
fn file_report_without_threshold_round_trip() {
    let report = analyze_file(fixture_path("simple_function.rs")).unwrap();

    // exceeds_threshold should be None
    assert!(report.functions.iter().all(|f| f.exceeds_threshold.is_none()));

    let json = serde_json::to_string(&report).unwrap();
    let deserialized: FileReport = serde_json::from_str(&json).unwrap();

    assert_eq!(report, deserialized);
}

#[test]
fn source_analysis_round_trip() {
    let source = r#"
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
"#;

    let report = analyze_source(source, Language::Rust).unwrap();

    let json = serde_json::to_string(&report).unwrap();
    let deserialized: FileReport = serde_json::from_str(&json).unwrap();

    assert_eq!(report, deserialized);
    assert_eq!(deserialized.path, "");
    assert_eq!(deserialized.language, Language::Rust);
}

#[test]
fn multiple_functions_round_trip() {
    let report = analyze_file(fixture_path("boolean_operators.rs")).unwrap();

    let json = serde_json::to_string_pretty(&report).unwrap();
    let deserialized: FileReport = serde_json::from_str(&json).unwrap();

    assert_eq!(report, deserialized);
    assert_eq!(report.functions.len(), deserialized.functions.len());
}

#[test]
fn json_fields_present() {
    let config = AnalysisConfig {
        cognitive_threshold: Some(5),
        include_methods: true,
    };
    let report = analyze_file_with_config(fixture_path("nested_control_flow.rs"), &config).unwrap();

    let json = serde_json::to_string_pretty(&report).unwrap();

    // Verify all expected field names are in the JSON output
    assert!(json.contains("\"path\""), "missing path field");
    assert!(json.contains("\"language\""), "missing language field");
    assert!(json.contains("\"functions\""), "missing functions field");
    assert!(json.contains("\"file_cognitive\""), "missing file_cognitive field");
    assert!(json.contains("\"file_cyclomatic\""), "missing file_cyclomatic field");
    assert!(json.contains("\"file_sloc\""), "missing file_sloc field");
    assert!(json.contains("\"name\""), "missing name field");
    assert!(json.contains("\"start_line\""), "missing start_line field");
    assert!(json.contains("\"end_line\""), "missing end_line field");
    assert!(json.contains("\"cognitive\""), "missing cognitive field");
    assert!(json.contains("\"cyclomatic\""), "missing cyclomatic field");
    assert!(json.contains("\"sloc\""), "missing sloc field");
    assert!(json.contains("\"exceeds_threshold\""), "missing exceeds_threshold field");
}

#[test]
fn language_enum_serialization() {
    let languages = vec![
        (Language::Rust, "\"Rust\""),
        (Language::Python, "\"Python\""),
        (Language::JavaScript, "\"JavaScript\""),
        (Language::TypeScript, "\"TypeScript\""),
        (Language::Java, "\"Java\""),
        (Language::CSharp, "\"CSharp\""),
        (Language::Cpp, "\"Cpp\""),
        (Language::C, "\"C\""),
        (Language::Go, "\"Go\""),
        (Language::Php, "\"Php\""),
    ];

    for (lang, expected_json) in languages {
        let json = serde_json::to_string(&lang).unwrap();
        assert_eq!(json, expected_json, "serialization of {lang}");

        let deserialized: Language = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, lang, "round-trip of {lang}");
    }
}

#[test]
fn function_metrics_standalone_round_trip() {
    let metrics = FunctionMetrics {
        name: "complex_function".to_string(),
        start_line: 10,
        end_line: 50,
        cognitive: 15,
        cyclomatic: 8,
        sloc: 35,
        exceeds_threshold: Some(true),
    };

    let json = serde_json::to_string(&metrics).unwrap();
    let deserialized: FunctionMetrics = serde_json::from_str(&json).unwrap();

    assert_eq!(metrics, deserialized);
}

#[test]
fn analysis_config_round_trip() {
    let configs = vec![
        AnalysisConfig {
            cognitive_threshold: Some(10),
            include_methods: true,
        },
        AnalysisConfig {
            cognitive_threshold: None,
            include_methods: false,
        },
        AnalysisConfig::default(),
    ];

    for config in configs {
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: AnalysisConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, deserialized);
    }
}
