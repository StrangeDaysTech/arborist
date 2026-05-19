// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2026 Strange Days Tech S.A.S. de C.V. <https://strangedays.tech>

#![cfg(feature = "rust")]

use arborist::{AnalysisConfig, Language, analyze_source_with_config};

#[test]
fn include_methods_true_includes_impl_methods() {
    let source = r#"
fn top_level() -> i32 {
    42
}

struct Counter;

impl Counter {
    fn increment(&mut self) {
        // method inside impl block
    }

    fn get(&self) -> i32 {
        0
    }
}
"#;
    let config = AnalysisConfig {
        include_methods: true,
        ..Default::default()
    };
    let report = analyze_source_with_config(source, Language::Rust, &config).unwrap();

    let names: Vec<&str> = report.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(
        names.contains(&"top_level"),
        "should include top_level function"
    );
    assert!(
        names.contains(&"increment"),
        "should include method increment"
    );
    assert!(names.contains(&"get"), "should include method get");
    assert_eq!(report.functions.len(), 3, "should have 3 functions total");
}

#[test]
fn include_methods_false_excludes_impl_methods() {
    let source = r#"
fn top_level() -> i32 {
    42
}

struct Counter;

impl Counter {
    fn increment(&mut self) {
        // method inside impl block
    }

    fn get(&self) -> i32 {
        0
    }
}
"#;
    let config = AnalysisConfig {
        include_methods: false,
        ..Default::default()
    };
    let report = analyze_source_with_config(source, Language::Rust, &config).unwrap();

    let names: Vec<&str> = report.functions.iter().map(|f| f.name.as_str()).collect();
    assert!(
        names.contains(&"top_level"),
        "should include top_level function"
    );
    assert!(
        !names.contains(&"increment"),
        "should NOT include method increment"
    );
    assert!(!names.contains(&"get"), "should NOT include method get");
    assert_eq!(
        report.functions.len(),
        1,
        "should have only 1 top-level function"
    );
}

#[test]
fn include_methods_default_is_true() {
    let config = AnalysisConfig::default();
    assert!(
        config.include_methods,
        "default include_methods should be true"
    );
}

#[test]
fn include_methods_false_only_affects_methods() {
    // Multiple top-level functions should all appear regardless of include_methods
    let source = r#"
fn alpha() -> i32 { 1 }
fn beta() -> i32 { 2 }
fn gamma() -> i32 { 3 }
"#;
    let config = AnalysisConfig {
        include_methods: false,
        ..Default::default()
    };
    let report = analyze_source_with_config(source, Language::Rust, &config).unwrap();
    assert_eq!(
        report.functions.len(),
        3,
        "all top-level functions should appear"
    );
}

#[test]
fn include_methods_false_with_threshold() {
    // Combine include_methods=false with threshold to verify both configs work together
    let source = r#"
fn simple() -> i32 {
    42
}

struct Processor;

impl Processor {
    fn complex(&self, items: &[i32]) -> i32 {
        let mut sum = 0;
        if !items.is_empty() {
            for item in items {
                if *item > 0 {
                    sum += item;
                }
            }
        }
        sum
    }
}
"#;
    let config = AnalysisConfig {
        cognitive_threshold: Some(5),
        include_methods: false,
    };
    let report = analyze_source_with_config(source, Language::Rust, &config).unwrap();

    // Only top_level function should appear (complex is a method, excluded)
    assert_eq!(report.functions.len(), 1, "only top-level functions");
    let f = &report.functions[0];
    assert_eq!(f.name, "simple");
    assert_eq!(
        f.exceeds_threshold,
        Some(false),
        "simple should not exceed threshold"
    );
}

#[test]
fn include_methods_false_file_sloc_still_counts_all_lines() {
    // Even when methods are excluded from functions list,
    // file_sloc should count all source lines in the file
    let source = r#"
fn top_level() -> i32 {
    42
}

struct Foo;

impl Foo {
    fn method(&self) -> i32 {
        1 + 2
    }
}
"#;
    let config_with = AnalysisConfig {
        include_methods: true,
        ..Default::default()
    };
    let config_without = AnalysisConfig {
        include_methods: false,
        ..Default::default()
    };

    let report_with = analyze_source_with_config(source, Language::Rust, &config_with).unwrap();
    let report_without =
        analyze_source_with_config(source, Language::Rust, &config_without).unwrap();

    // file_sloc should be the same regardless of include_methods
    assert_eq!(
        report_with.file_sloc, report_without.file_sloc,
        "file_sloc should be identical regardless of include_methods"
    );

    // But file_cognitive may differ since excluded methods don't contribute
    // (file_cognitive = sum of reported functions' cognitive)
    assert!(
        report_with.functions.len() > report_without.functions.len(),
        "with methods should have more functions listed"
    );
}
