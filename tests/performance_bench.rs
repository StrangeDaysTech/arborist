use std::time::Instant;

/// SC-002: Analysis of files under 1000 lines must complete in under 100ms.
///
/// The large_file.rs fixture has 1000+ lines and 40+ functions with varying
/// complexity. We allow a generous margin and assert < 100ms per the spec.
#[test]
fn large_file_analysis_under_100ms() {
    let path = "tests/fixtures/rust/large_file.rs";

    // Warm up: run once to ensure any lazy initialization is done
    let _ = arborist::analyze_file(path).expect("fixture should parse");

    // Measure 10 iterations and take the median
    let mut durations = Vec::with_capacity(10);
    for _ in 0..10 {
        let start = Instant::now();
        let report = arborist::analyze_file(path).expect("fixture should parse");
        let elapsed = start.elapsed();
        durations.push(elapsed);

        // Sanity: the fixture has 20+ functions
        assert!(
            report.functions.len() >= 20,
            "expected 20+ functions, got {}",
            report.functions.len()
        );
    }

    durations.sort();
    let median = durations[durations.len() / 2];

    assert!(
        median.as_millis() < 100,
        "median analysis time was {}ms, expected < 100ms",
        median.as_millis()
    );

    eprintln!(
        "Performance: median={}us, min={}us, max={}us ({} functions, {} SLOC)",
        durations[durations.len() / 2].as_micros(),
        durations[0].as_micros(),
        durations[durations.len() - 1].as_micros(),
        arborist::analyze_file(path).unwrap().functions.len(),
        arborist::analyze_file(path).unwrap().file_sloc,
    );
}
