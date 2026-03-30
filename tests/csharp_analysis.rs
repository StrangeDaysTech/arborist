#![cfg(feature = "csharp")]

use arborist::analyze_file;

/// Helper to get the project root directory for fixture paths.
fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/csharp/{name}")
}

#[test]
fn simple_function_metrics() {
    let report = analyze_file(fixture_path("simple_function.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "Add");
    assert_eq!(f.cognitive, 0, "cognitive complexity for Add");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for Add");
    assert_eq!(f.sloc, 4, "sloc for Add");

    // File-level aggregates
    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 1);
    assert_eq!(report.file_sloc, 6);
}

#[test]
fn nested_control_flow_metrics() {
    let report = analyze_file(fixture_path("nested_control_flow.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "Process");
    assert_eq!(f.cognitive, 6, "cognitive complexity for Process");
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for Process");
    assert_eq!(f.sloc, 11, "sloc for Process");

    // File-level aggregates
    assert_eq!(report.file_cognitive, 6);
    assert_eq!(report.file_cyclomatic, 4);
    assert_eq!(report.file_sloc, 13);
}

#[test]
fn boolean_operators_metrics() {
    let report = analyze_file(fixture_path("boolean_operators.cs")).unwrap();
    assert_eq!(report.functions.len(), 2);

    let check_all = &report.functions[0];
    assert_eq!(check_all.name, "CheckAll");
    assert_eq!(check_all.cognitive, 2, "cognitive complexity for CheckAll");
    assert_eq!(check_all.cyclomatic, 4, "cyclomatic complexity for CheckAll");
    assert_eq!(check_all.sloc, 6, "sloc for CheckAll");

    let check_mixed = &report.functions[1];
    assert_eq!(check_mixed.name, "CheckMixed");
    assert_eq!(check_mixed.cognitive, 3, "cognitive complexity for CheckMixed");
    assert_eq!(check_mixed.cyclomatic, 4, "cyclomatic complexity for CheckMixed");
    assert_eq!(check_mixed.sloc, 6, "sloc for CheckMixed");

    // File-level aggregates (sum of both functions)
    assert_eq!(report.file_cognitive, 5);
    assert_eq!(report.file_cyclomatic, 8);
    assert_eq!(report.file_sloc, 14);
}

#[test]
fn else_if_chain_metrics() {
    let report = analyze_file(fixture_path("else_if_chain.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "Classify");
    assert_eq!(f.cognitive, 9, "cognitive complexity for Classify");
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for Classify");
    assert_eq!(f.sloc, 11, "sloc for Classify");
}

#[test]
fn lambda_delegates_metrics() {
    let report = analyze_file(fixture_path("lambda_delegates.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "ApplyFilter");
    assert_eq!(f.cognitive, 0, "cognitive complexity for ApplyFilter");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for ApplyFilter");
    assert_eq!(f.sloc, 4, "sloc for ApplyFilter");
}

#[test]
fn recursion_metrics() {
    let report = analyze_file(fixture_path("recursion.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "Factorial");
    assert_eq!(f.cognitive, 3, "cognitive complexity for Factorial");
    assert_eq!(f.cyclomatic, 2, "cyclomatic complexity for Factorial");
    assert_eq!(f.sloc, 7, "sloc for Factorial");
}

#[test]
fn match_switch_metrics() {
    let report = analyze_file(fixture_path("match_switch.cs")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "Categorize");
    assert_eq!(f.cognitive, 3, "cognitive complexity for Categorize");
    assert_eq!(f.cyclomatic, 6, "cyclomatic complexity for Categorize");
    assert_eq!(f.sloc, 20, "sloc for Categorize");
}

#[test]
fn language_detected_as_csharp() {
    let report = analyze_file(fixture_path("simple_function.cs")).unwrap();
    assert_eq!(report.language, arborist::Language::CSharp);
}

#[test]
fn path_populated_in_report() {
    let path = fixture_path("simple_function.cs");
    let report = analyze_file(&path).unwrap();
    assert!(
        report.path.contains("simple_function.cs"),
        "report path should contain the fixture filename, got: {}",
        report.path
    );
}

#[test]
fn functions_sorted_by_start_line() {
    let report = analyze_file(fixture_path("boolean_operators.cs")).unwrap();
    assert_eq!(report.functions.len(), 2);
    assert!(
        report.functions[0].start_line < report.functions[1].start_line,
        "functions should be sorted by start_line"
    );
}
