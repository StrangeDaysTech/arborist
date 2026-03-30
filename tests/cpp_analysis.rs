#![cfg(feature = "cpp")]

use arborist::analyze_file;

/// Helper to get the project root directory for fixture paths.
fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/cpp/{name}")
}

#[test]
fn simple_function_metrics() {
    let report = analyze_file(fixture_path("simple_function.cpp")).unwrap();
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
    let report = analyze_file(fixture_path("nested_control_flow.cpp")).unwrap();
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
    let report = analyze_file(fixture_path("boolean_operators.cpp")).unwrap();
    assert_eq!(report.functions.len(), 2);

    let check_all = &report.functions[0];
    assert_eq!(check_all.name, "checkAll");
    assert_eq!(check_all.cognitive, 2, "cognitive complexity for checkAll");
    assert_eq!(check_all.cyclomatic, 4, "cyclomatic complexity for checkAll");
    assert_eq!(check_all.sloc, 6, "sloc for checkAll");

    let check_mixed = &report.functions[1];
    assert_eq!(check_mixed.name, "checkMixed");
    assert_eq!(check_mixed.cognitive, 3, "cognitive complexity for checkMixed");
    assert_eq!(check_mixed.cyclomatic, 4, "cyclomatic complexity for checkMixed");
    assert_eq!(check_mixed.sloc, 6, "sloc for checkMixed");

    // File-level aggregates (sum of both functions)
    assert_eq!(report.file_cognitive, 5);
    assert_eq!(report.file_cyclomatic, 8);
    assert_eq!(report.file_sloc, 12);
}

#[test]
fn else_if_chain_metrics() {
    let report = analyze_file(fixture_path("else_if_chain.cpp")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "classify");
    assert_eq!(f.cognitive, 9, "cognitive complexity for classify");
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for classify");
    assert_eq!(f.sloc, 11, "sloc for classify");
}

#[test]
fn lambda_captures_metrics() {
    let report = analyze_file(fixture_path("lambda_captures.cpp")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "applyFilter");
    assert_eq!(f.cognitive, 0, "cognitive complexity for applyFilter");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for applyFilter");
    assert_eq!(f.sloc, 7, "sloc for applyFilter");
}

#[test]
fn recursion_metrics() {
    let report = analyze_file(fixture_path("recursion.cpp")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "factorial");
    assert_eq!(f.cognitive, 3, "cognitive complexity for factorial");
    assert_eq!(f.cyclomatic, 2, "cyclomatic complexity for factorial");
    assert_eq!(f.sloc, 7, "sloc for factorial");
}

#[test]
fn match_switch_metrics() {
    let report = analyze_file(fixture_path("match_switch.cpp")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "categorize");
    assert_eq!(f.cognitive, 3, "cognitive complexity for categorize");
    assert_eq!(f.cyclomatic, 6, "cyclomatic complexity for categorize");
    assert_eq!(f.sloc, 20, "sloc for categorize");
}

#[test]
fn language_detected_as_cpp() {
    let report = analyze_file(fixture_path("simple_function.cpp")).unwrap();
    assert_eq!(report.language, arborist::Language::Cpp);
}

#[test]
fn path_populated_in_report() {
    let path = fixture_path("simple_function.cpp");
    let report = analyze_file(&path).unwrap();
    assert!(
        report.path.contains("simple_function.cpp"),
        "report path should contain the fixture filename, got: {}",
        report.path
    );
}

#[test]
fn functions_sorted_by_start_line() {
    let report = analyze_file(fixture_path("boolean_operators.cpp")).unwrap();
    assert_eq!(report.functions.len(), 2);
    assert!(
        report.functions[0].start_line < report.functions[1].start_line,
        "functions should be sorted by start_line"
    );
}
