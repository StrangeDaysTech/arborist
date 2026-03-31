#![cfg(feature = "swift")]

use arborist::analyze_file;

fn fixture_path(name: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{manifest_dir}/tests/fixtures/swift/{name}")
}

#[test]
fn simple_function_metrics() {
    let report = analyze_file(fixture_path("simple_function.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "add");
    assert_eq!(f.cognitive, 0, "cognitive complexity for add");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for add");
    assert_eq!(f.sloc, 3, "sloc for add");

    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 1);
    assert_eq!(report.file_sloc, 3);
}

#[test]
fn nested_control_flow_metrics() {
    let report = analyze_file(fixture_path("nested_control_flow.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "processItems");
    // if (+1) + for (+1+1 nesting) + if (+1+2 nesting) = 1+2+3 = 6
    assert_eq!(f.cognitive, 6, "cognitive complexity for processItems");
    // base(1) + if + for + if = 4
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for processItems");
    assert_eq!(f.sloc, 11, "sloc for processItems");

    assert_eq!(report.file_cognitive, 6);
    assert_eq!(report.file_cyclomatic, 4);
    assert_eq!(report.file_sloc, 11);
}

#[test]
fn boolean_operators_metrics() {
    let report = analyze_file(fixture_path("boolean_operators.swift")).unwrap();
    assert_eq!(report.functions.len(), 2);

    let check_all = &report.functions[0];
    assert_eq!(check_all.name, "checkAll");
    // a && b && c — one homogeneous sequence = +1 cognitive
    assert_eq!(check_all.cognitive, 1, "cognitive complexity for checkAll");
    // base(1) + 2 && operators = 3
    assert_eq!(check_all.cyclomatic, 3, "cyclomatic complexity for checkAll");
    assert_eq!(check_all.sloc, 3, "sloc for checkAll");

    let check_mixed = &report.functions[1];
    assert_eq!(check_mixed.name, "checkMixed");
    // a && b || c — one sequence + one operator switch = +2 cognitive
    assert_eq!(check_mixed.cognitive, 2, "cognitive complexity for checkMixed");
    // base(1) + && + || = 3
    assert_eq!(check_mixed.cyclomatic, 3, "cyclomatic complexity for checkMixed");
    assert_eq!(check_mixed.sloc, 3, "sloc for checkMixed");

    assert_eq!(report.file_cognitive, 3);
    assert_eq!(report.file_cyclomatic, 6);
    assert_eq!(report.file_sloc, 6);
}

#[test]
fn else_if_chain_metrics() {
    let report = analyze_file(fixture_path("else_if_chain.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "classify");
    // if(+1) else(+1) if(+2,nest=1) else(+1) if(+3,nest=2) else(+1) = 9
    assert_eq!(f.cognitive, 9, "cognitive complexity for classify");
    // base(1) + 3 if_statements = 4
    assert_eq!(f.cyclomatic, 4, "cyclomatic complexity for classify");
    assert_eq!(f.sloc, 11, "sloc for classify");
}

#[test]
fn lambda_nested_metrics() {
    let report = analyze_file(fixture_path("lambda_nested.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "transform");
    assert_eq!(f.cognitive, 0, "cognitive complexity for transform");
    assert_eq!(f.cyclomatic, 1, "cyclomatic complexity for transform");
    assert_eq!(f.sloc, 3, "sloc for transform");
}

#[test]
fn match_switch_metrics() {
    let report = analyze_file(fixture_path("match_switch.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "classify");
    // switch(+1, nesting=0) = 1
    assert_eq!(f.cognitive, 1, "cognitive complexity for classify");
    // base(1) + 4 switch_entry arms = 5
    assert_eq!(f.cyclomatic, 5, "cyclomatic complexity for classify");
    assert_eq!(f.sloc, 12, "sloc for classify");
}

#[test]
fn recursion_metrics() {
    let report = analyze_file(fixture_path("recursion.swift")).unwrap();
    assert_eq!(report.functions.len(), 1);

    let f = &report.functions[0];
    assert_eq!(f.name, "fibonacci");
    // if(+1) — recursion not detected (Swift grammar has no "function" field on call_expression)
    assert_eq!(f.cognitive, 1, "cognitive complexity for fibonacci");
    // base(1) + if = 2
    assert_eq!(f.cyclomatic, 2, "cyclomatic complexity for fibonacci");
    assert_eq!(f.sloc, 4, "sloc for fibonacci");
}

#[test]
fn language_detected_as_swift() {
    let report = analyze_file(fixture_path("simple_function.swift")).unwrap();
    assert_eq!(report.language, arborist::Language::Swift);
}

#[test]
fn path_populated_in_report() {
    let path = fixture_path("simple_function.swift");
    let report = analyze_file(&path).unwrap();
    assert!(
        report.path.contains("simple_function.swift"),
        "report path should contain the fixture filename, got: {}",
        report.path
    );
}

#[test]
fn functions_sorted_by_start_line() {
    let report = analyze_file(fixture_path("boolean_operators.swift")).unwrap();
    assert_eq!(report.functions.len(), 2);
    assert!(
        report.functions[0].start_line < report.functions[1].start_line,
        "functions should be sorted by start_line"
    );
}
