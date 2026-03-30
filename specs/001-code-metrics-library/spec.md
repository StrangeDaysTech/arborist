# Feature Specification: Code Metrics Analysis Library (Arborist)

**Feature Branch**: `001-code-metrics-library`
**Created**: 2026-03-27
**Status**: Implemented
**Input**: User description: "Librería independiente de análisis de métricas de código (complejidad cognitiva, ciclomática, SLOC) usando tree-sitter para múltiples lenguajes"

## Clarifications

### Session 2026-03-27

- Q: Should closures/lambdas produce their own FunctionMetrics entries? → A: No. They only affect the complexity metrics of their containing function.
- Q: Do file-level aggregates include top-level code outside functions? → A: Complexity aggregates sum only reported functions. File SLOC counts all source lines in the file including top-level code.
- Q: How are threshold-exceeding functions flagged in the output? → A: An optional boolean field `exceeds_threshold` on FunctionMetrics, present only when a threshold is configured.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Analyze a Single Source File (Priority: P1)

As a developer or automated tool, I want to analyze a source code file and get its complexity metrics (cognitive complexity, cyclomatic complexity, and SLOC) broken down by function, so I can identify which functions are hardest to understand and maintain.

**Why this priority**: This is the core value proposition of the library. Without per-function metric extraction from a file, nothing else works. It enables both human developers and tooling (like DevTrail) to make informed decisions about code quality.

**Independent Test**: Can be fully tested by passing a source file path and verifying that the returned report contains the expected function names with their metrics. Delivers immediate value: a developer can point arborist at any supported file and get actionable complexity data.

**Acceptance Scenarios**:

1. **Given** a valid Rust source file with 3 functions of known complexity, **When** the user calls the file analysis function, **Then** the report contains exactly 3 function entries with correct cognitive, cyclomatic, and SLOC values for each.
2. **Given** a source file with a supported extension (e.g., `.py`, `.js`, `.rs`), **When** the user calls the file analysis function without specifying a language, **Then** the library automatically detects the language from the file extension and produces a valid report.
3. **Given** a file path that does not exist, **When** the user calls the file analysis function, **Then** the library returns a clear error indicating the file was not found.
4. **Given** an empty source file, **When** the user calls the file analysis function, **Then** the report contains zero functions and all file-level metrics are zero.

---

### User Story 2 - Analyze Source Code from Memory (Priority: P1)

As a tool integrator, I want to analyze source code provided as a string (not from a file on disk), specifying the language explicitly, so I can use arborist in pipelines where code is fetched from version control, APIs, or generated dynamically.

**Why this priority**: Equal to file analysis because many integration scenarios (CI pipelines, editor plugins, code review tools) work with in-memory code rather than files on disk. This is essential for embeddability.

**Independent Test**: Can be fully tested by passing a source string and a language identifier, then verifying the returned report matches expected metrics. Delivers value independently: a CI tool can analyze code from a diff without writing temp files.

**Acceptance Scenarios**:

1. **Given** a Python source string containing 2 functions with known complexity, **When** the user calls the source analysis function specifying Python as the language, **Then** the report contains exactly 2 function entries with correct metrics.
2. **Given** a source string in an unsupported language, **When** the user calls the source analysis function, **Then** the library returns a clear error indicating the language is not supported.
3. **Given** a source string with syntax errors, **When** the user calls the source analysis function, **Then** the library produces a best-effort report (tree-sitter is error-tolerant) and does not crash.

---

### User Story 3 - Detect Language Automatically (Priority: P2)

As a developer, I want the library to automatically detect the programming language of a file based on its extension, so I don't need to manually specify the language for each file I analyze.

**Why this priority**: Reduces friction for the most common use case (analyzing files on disk). Language detection by extension is well-defined and removes a step from the user's workflow.

**Independent Test**: Can be tested by calling file analysis on files with various extensions and verifying each resolves to the correct language.

**Acceptance Scenarios**:

1. **Given** a file with `.ts` extension, **When** the library detects the language, **Then** it identifies the file as TypeScript.
2. **Given** a file with `.tsx` extension, **When** the library detects the language, **Then** it identifies the file as TypeScript (TSX variant).
3. **Given** a file with `.h` extension, **When** the library detects the language, **Then** it identifies the file as C (default for ambiguous headers).
4. **Given** a file with an unknown extension (e.g., `.xyz`), **When** the library attempts detection, **Then** it returns an error indicating the language could not be determined.

---

### User Story 4 - Configure Analysis Thresholds (Priority: P2)

As a developer integrating arborist into a CI pipeline, I want to configure a cognitive complexity threshold so I can identify functions that exceed an acceptable complexity level.

**Why this priority**: Thresholds enable actionable output (pass/fail decisions in CI). Without configuration, the library produces data but the consumer must implement their own threshold logic.

**Independent Test**: Can be tested by analyzing a file with a threshold configured and verifying that the report correctly flags functions above the threshold.

**Acceptance Scenarios**:

1. **Given** a configuration with cognitive threshold set to 8, **When** analyzing a file with functions of complexity 5, 10, and 15, **Then** the report identifies the two functions exceeding the threshold.
2. **Given** no custom configuration (defaults), **When** analyzing a file, **Then** all metrics are computed without any threshold filtering.
3. **Given** a configuration with `include_methods` set to false, **When** analyzing a file containing a class or struct with methods, **Then** the report includes only top-level functions, not methods inside class or impl blocks.

---

### User Story 5 - Select Languages via Feature Flags (Priority: P2)

As a library consumer, I want to include only the language support I need via compile-time feature flags, so I can minimize binary size and compilation time.

**Why this priority**: Each language grammar adds compilation time and binary size. For a crate published on crates.io, granular feature flags are essential for adoption, as users should not be forced to compile 10 grammars when they only need 2.

**Independent Test**: Can be tested by compiling the library with a subset of features enabled and verifying that only those languages are available, while others return an "unsupported language" error.

**Acceptance Scenarios**:

1. **Given** the library compiled with only the `rust` and `python` features, **When** analyzing a Go file, **Then** the library returns an error indicating Go support is not enabled.
2. **Given** the library compiled with the `default` features, **When** analyzing files in the 6 default languages, **Then** all produce valid reports.
3. **Given** the library compiled with the `all` feature, **When** analyzing files in any of the 10 Tier 1 languages, **Then** all produce valid reports.

---

### User Story 6 - Serialize Results (Priority: P3)

As a tool integrator, I want metric results to be serializable (e.g., to JSON), so I can pass analysis results to other tools, store them, or display them in dashboards.

**Why this priority**: Serialization is a common integration requirement but not critical for core functionality. The data structures are useful even without serialization; this story adds convenience.

**Independent Test**: Can be tested by analyzing a file, serializing the report, and verifying the serialized output is valid and contains all metric fields.

**Acceptance Scenarios**:

1. **Given** a file analysis report, **When** the consumer serializes it to JSON via serde, **Then** the output contains all function names, metrics, and file-level aggregates.
2. **Given** a report with multiple functions, **When** serialized to JSON and deserialized back, **Then** all fields (including line numbers, language, and exceeds_threshold) are preserved without data loss.

---

### Edge Cases

- What happens when a file contains only comments and no executable code? The report should have zero functions and SLOC of zero.
- What happens when a function contains deeply nested closures/lambdas? Cognitive complexity should correctly compound nesting penalties. Closures/lambdas do not produce their own FunctionMetrics entries; they only affect the containing function's metrics.
- What happens when a file has mixed-encoding content? The library should handle UTF-8 source; other encodings should produce a clear error.
- What happens when a match/switch has many arms? Each arm/case (including `default`/wildcard) counts as +1 for cyclomatic complexity, following the SonarSource/McCabe specification. The match/switch construct itself does not add +1 — only its arms do. Cognitive complexity is unaffected: the construct counts as +1 with nesting penalty, and arms are not counted individually.
- What happens when boolean expressions chain the same operator (e.g., `a && b && c && d`)? Cognitive complexity counts this as +1 for the whole sequence, not per operator.
- What happens when boolean expressions mix operators (e.g., `a && b || c`)? Cognitive complexity counts +1 for the first sequence and +1 for each operator change.
- What happens when `else if` chains are used? In languages with dedicated else-if syntax (Python `elif`, PHP `elseif`), they do not increment the nesting level (treated as flat continuations). In languages where `else if` is syntactically `else { if {} }` (JavaScript, TypeScript, Java, C#, C++, C, Go, Rust), each nested `if` applies standard nesting rules per the AST structure.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST compute cognitive complexity for each function in a source file, following the SonarSource cognitive complexity specification (G. Ann Campbell, 2017).
- **FR-002**: The library MUST compute cyclomatic complexity for each function, counting decision points (if, for, while, each match/switch arm, boolean operators) per the SonarSource/McCabe specification.
- **FR-003**: The library MUST compute SLOC (Source Lines of Code) for each function, excluding blank lines and comment-only lines.
- **FR-004**: The library MUST accept a file path as input and automatically detect the programming language from the file extension.
- **FR-005**: The library MUST accept source code as a string with an explicit language identifier and produce the same metrics as file-based analysis. Both plain and config-accepting variants (`analyze_source`, `analyze_source_with_config`) MUST be provided for parity with file-based analysis.
- **FR-006**: The library MUST support 10 programming languages at initial release: Rust, Python, JavaScript, TypeScript/TSX, Java, C#, C++, C, Go, and PHP.
- **FR-007**: The library MUST allow each language to be enabled or disabled independently via compile-time feature flags.
- **FR-008**: The library MUST provide a sensible set of default features (a subset of supported languages) to minimize compilation time for common use cases.
- **FR-009**: The library MUST return structured results containing per-function metrics (name, start line, end line, cognitive complexity, cyclomatic complexity, SLOC) and file-level aggregates. File-level complexity aggregates MUST be the sum of all function-level values. File-level SLOC MUST count all source lines in the file, including code outside functions.
- **FR-010**: The library MUST support serialization of result structures via serde, with JSON as the primary validated format. Serde's design allows consumers to use any compatible format (TOML, MessagePack, etc.) without library changes.
- **FR-011**: The library MUST allow configuring an optional cognitive complexity threshold. When configured, each FunctionMetrics entry MUST include an `exceeds_threshold` boolean field indicating whether the function's cognitive complexity exceeds the configured value.
- **FR-012**: The library MUST handle syntax errors gracefully (leveraging tree-sitter's error tolerance) and produce best-effort results without crashing. Best-effort means all parseable function nodes produce metrics; unparseable regions are skipped.
- **FR-013**: The library MUST return clear, descriptive errors for: file not found, unsupported language, unrecognized file extension, and language feature not enabled.
- **FR-014**: The library MUST correctly implement cognitive complexity nesting rules: control flow structures increment nesting, `else if` is flat in languages with dedicated syntax (see Edge Cases), and lambdas/closures increment nesting.
- **FR-015**: The library MUST correctly implement cognitive complexity boolean operator rules: same-operator sequences count as +1, operator changes add additional increments.
- **FR-016**: The library MUST be usable as an independent crate published on crates.io, with no dependency on DevTrail or any specific framework.
- **FR-017**: The library MUST use a modular architecture where adding support for a new language requires implementing only a language-specific profile, without modifying core metric computation logic.
- **FR-018**: The library MUST allow configuring whether class/struct methods are included in the analysis via an `include_methods` option (default: true). When disabled, only top-level functions produce FunctionMetrics entries.

### Key Entities

- **FunctionMetrics**: Represents the analysis results for a single function or method (not closures/lambdas). Closures and lambdas contribute to the complexity metrics of their containing function but do not produce independent FunctionMetrics entries. Contains the function name, line range, all three computed metrics (cognitive, cyclomatic, SLOC), and an optional boolean `exceeds_threshold` field (present only when a cognitive complexity threshold is configured).
- **FileReport**: Represents the analysis results for a complete source file. Contains the file path, detected language, list of FunctionMetrics, and file-level aggregate metrics. File-level complexity aggregates (cognitive, cyclomatic) are the sum of all reported functions. File-level SLOC counts all source lines in the file, including top-level code outside functions.
- **Language**: Represents a supported programming language. Used for automatic detection from file extensions and explicit specification by users.
- **AnalysisConfig**: Represents user-configurable analysis parameters, such as complexity thresholds and analysis scope options.
- **LanguageProfile**: Represents the mapping between a language's AST node types and control-flow concepts. This is the extension point for adding new language support.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: The library correctly computes cognitive complexity for known test cases across all 10 Tier 1 languages, matching reference values from the SonarSource cognitive complexity specification.
- **SC-002**: Analysis of a typical source file (under 1000 lines) completes in under 100 milliseconds (measured via `cargo bench` or equivalent; hardware baseline: GitHub Actions runner or comparable 2+ vCPU environment).
- **SC-003**: The library can analyze all 10 Tier 1 languages with consistent accuracy, validated against at least 5 test fixtures per language with pre-calculated expected values.
- **SC-004**: Adding support for a new language requires implementing only a single language profile component, without changes to core metric calculation logic.
- **SC-005**: Compiling the library with a single language feature enabled takes under 30 seconds (measured via `cargo build --no-default-features --features rust` on a GitHub Actions runner or comparable 2+ vCPU environment).
- **SC-006**: The library handles files with syntax errors without crashing, producing partial results for 100% of parseable functions in the file.
- **SC-007**: All public result structures can be serialized to JSON (primary validated format) and deserialized back without data loss. Serde compatibility enables other formats without library changes.

## Assumptions

- The library targets the Rust ecosystem and will be published as a crate on crates.io. Consumers are Rust developers or tools written in Rust.
- tree-sitter v0.25 is the parsing foundation. Grammar crates have been verified to compile together and are stable.
- The SonarSource cognitive complexity paper (G. Ann Campbell, 2017) is the authoritative reference for cognitive complexity calculation rules.
- The initial release (v0.1.0) covers only Tier 1 languages (10 languages). Tier 2 and Tier 3 languages are planned for subsequent releases and are out of scope for this specification.
- Dual licensing under MIT and Apache-2.0 (Rust ecosystem convention) is assumed for the open-source release.
- The library is a pure computation library with no I/O beyond reading source files. It does not interact with networks, databases, or external services.
- File encoding is assumed to be UTF-8. Non-UTF-8 files are out of scope for the initial release.
- The `.h` file extension will default to C. Users needing C++ analysis for `.h` files can use the explicit language API.
