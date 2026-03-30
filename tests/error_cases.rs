use arborist::{analyze_file, ArboristError};

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

/// Edge case: file contains only comments and no executable code.
/// Spec says: "The report should have zero functions and SLOC of zero."
#[cfg(feature = "rust")]
#[test]
fn comments_only_file_zero_functions_zero_sloc() {
    let report = analyze_file("tests/fixtures/rust/comments_only.rs").unwrap();
    assert_eq!(report.functions.len(), 0, "comments-only file should have no functions");
    assert_eq!(report.file_sloc, 0, "comments-only file should have 0 SLOC");
    assert_eq!(report.file_cognitive, 0);
    assert_eq!(report.file_cyclomatic, 0);
}

/// Edge case: non-UTF-8 file should produce an I/O error.
/// Spec says: "other encodings should produce a clear error."
#[cfg(feature = "rust")]
#[test]
fn non_utf8_file_returns_io_error() {
    let result = analyze_file("tests/fixtures/rust/invalid_utf8.rs");
    assert!(result.is_err(), "non-UTF-8 file should return an error");
    let err = result.unwrap_err();
    assert!(
        matches!(err, ArboristError::Io(_)),
        "expected ArboristError::Io, got: {err}"
    );
}
