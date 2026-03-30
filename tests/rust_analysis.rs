#![cfg(feature = "rust")]

use arborist::analyze_file;

/// Helper to get the project root directory for fixture paths.
fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/rust/{name}")
}

#[test]
fn simple_function_metrics() {
    let report = analyze_file(fixture_path("simple_function.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "add");
    assert_eq!(f.cognitive, 0, "cognitive complexity for add");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for add");
    assert_eq!(f.sloc, 4, "sloc for add");

    // File-level aggregates
    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 1);
    assert_eq!(report.file_sloc, 4);
}

#[test]
fn nested_control_flow_metrics() {
    let report = analyze_file(fixture_path("nested_control_flow.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "process");
    assert_eq!(f.cognitive, 6, "cognitive complexity for process");
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for process");
    assert_eq!(f.sloc, 11, "sloc for process");

    // File-level aggregates
    assert_eq!(report.file_cognitive, 6);
    assert_eq!(report.file_cyclomatic, 4);
    assert_eq!(report.file_sloc, 11);
}

#[test]
fn boolean_operators_metrics() {
    let report = analyze_file(fixture_path("boolean_operators.rs")).unwrap();
    assert_eq!(report.functions.len(), 2);

    let check_same = &report.functions[0];
    assert_eq!(check_same.name, "check_same");
    assert_eq!(check_same.cognitive, 2, "cognitive complexity for check_same");
    assert_eq!(check_same.cyclomatic, 4, "cyclomatic complexity for check_same");
    assert_eq!(check_same.sloc, 6, "sloc for check_same");

    let check_mixed = &report.functions[1];
    assert_eq!(check_mixed.name, "check_mixed");
    assert_eq!(check_mixed.cognitive, 3, "cognitive complexity for check_mixed");
    assert_eq!(check_mixed.cyclomatic, 4, "cyclomatic complexity for check_mixed");
    assert_eq!(check_mixed.sloc, 6, "sloc for check_mixed");

    // File-level aggregates (sum of both functions)
    assert_eq!(report.file_cognitive, 5);
    assert_eq!(report.file_cyclomatic, 8);
    assert_eq!(report.file_sloc, 12);
}

#[test]
fn else_if_chain_metrics() {
    let report = analyze_file(fixture_path("else_if_chain.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "classify");
    assert_eq!(f.cognitive, 9, "cognitive complexity for classify");
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for classify");
    assert_eq!(f.sloc, 11, "sloc for classify");
}

#[test]
fn closures_lambdas_metrics() {
    let report = analyze_file(fixture_path("closures_lambdas.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "apply_filter");
    assert_eq!(f.cognitive, 3, "cognitive complexity for apply_filter");
    assert_eq!(f.cyclomatic, 2, "cyclomatic complexity for apply_filter");
    assert_eq!(f.sloc, 14, "sloc for apply_filter");
}

#[test]
fn recursion_metrics() {
    let report = analyze_file(fixture_path("recursion.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "factorial");
    assert_eq!(f.cognitive, 3, "cognitive complexity for factorial");
    assert_eq!(f.cyclomatic, 2, "cyclomatic complexity for factorial");
    assert_eq!(f.sloc, 7, "sloc for factorial");
}

#[test]
fn empty_file_metrics() {
    let report = analyze_file(fixture_path("empty.rs")).unwrap();
    assert_eq!(report.functions.len(), 0, "empty file should have no functions");
    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 0);
    assert_eq!(report.file_sloc, 0);
}

#[test]
fn functions_sorted_by_start_line() {
    let report = analyze_file(fixture_path("boolean_operators.rs")).unwrap();
    assert_eq!(report.functions.len(), 2);
    assert!(
        report.functions[0].start_line < report.functions[1].start_line,
        "functions should be sorted by start_line"
    );
}

#[test]
fn language_detected_as_rust() {
    let report = analyze_file(fixture_path("simple_function.rs")).unwrap();
    assert_eq!(report.language, arborist::Language::Rust);
}

#[test]
fn path_populated_in_report() {
    let path = fixture_path("simple_function.rs");
    let report = analyze_file(&path).unwrap();
    assert!(
        report.path.contains("simple_function.rs"),
        "report path should contain the fixture filename, got: {}",
        report.path
    );
}

#[test]
fn match_switch_metrics() {
    let report = analyze_file(fixture_path("match_switch.rs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "categorize");
    assert_eq!(f.cognitive, 4, "cognitive complexity for categorize");
    assert_eq!(f.cyclomatic, 6, "cyclomatic complexity for categorize");
    assert_eq!(f.sloc, 14, "sloc for categorize");
}
