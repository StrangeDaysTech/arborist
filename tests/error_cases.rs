use arborist::analyze_file;

#[test]
fn file_not_found_error() {
    let result = analyze_file("tests/fixtures/rust/nonexistent.rs");
    assert!(result.is_err(), "should return an error for missing file");
    let err = result.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("file not found"),
        "error should mention 'file not found', got: {msg}"
    );
}

#[test]
fn unknown_extension_error() {
    // Create a path with an unrecognized extension. The file must exist
    // so that we get past the file-not-found check.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest_dir}/Cargo.toml");

    // .toml is not a recognized source language extension
    let result = analyze_file(&path);
    assert!(result.is_err(), "should return an error for unknown extension");
    let err = result.unwrap_err();
    let msg = format!("{err}");
    assert!(
        msg.contains("unrecognized file extension"),
        "error should mention 'unrecognized file extension', got: {msg}"
    );
}

#[cfg(feature = "rust")]
#[test]
fn empty_file_no_error() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = format!("{manifest_dir}/tests/fixtures/rust/empty.rs");
    let report = analyze_file(&path).unwrap();
    assert_eq!(report.functions.len(), 0, "empty file should produce no functions");
    assert_eq!(report.file_sloc, 0, "empty file should have 0 sloc");
}
