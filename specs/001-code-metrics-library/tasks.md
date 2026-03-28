# Tasks: Code Metrics Analysis Library (Arborist)

**Input**: Design documents from `/specs/001-code-metrics-library/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/public-api.md

**Tests**: Required by project constitution (Principle VI: Test-First, Fixture-Driven).

**Organization**: Tasks grouped by user story. Each language profile includes both fixtures (tests) and implementation per TDD mandate.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization, dependency configuration, and directory structure.

- [x] T001 Create Cargo.toml with tree-sitter 0.25 dependency, serde dependency, all 10 grammar crates as optional dependencies, and feature flags (default = rust/python/javascript/typescript/java/go, all = default + csharp/cpp/c/php) per research.md R6 in Cargo.toml
- [x] T002 Create project directory structure: src/ (lib.rs, types.rs, error.rs, walker.rs, metrics/, languages/), tests/fixtures/ (rust/, python/, javascript/, typescript/, java/, csharp/, cpp/, c/, go/, php/), tests/integration/ per plan.md
- [x] T003 [P] Add LICENSE-MIT and LICENSE-APACHE dual license files at project root

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types, traits, and generic metric calculators. MUST complete before any user story.

**CRITICAL**: No user story work can begin until this phase is complete.

- [x] T004 Define Language enum (Rust, Python, JavaScript, TypeScript, Java, CSharp, Cpp, C, Go, Php) with Display and FromStr implementations in src/types.rs per data-model.md
- [x] T005 [P] Define FunctionMetrics struct with fields: name (String), start_line (usize), end_line (usize), cognitive (u64), cyclomatic (u64), sloc (u64), exceeds_threshold (Option\<bool\>) with Serialize/Deserialize derives in src/types.rs
- [x] T006 [P] Define FileReport struct with fields: path (String), language (Language), functions (Vec\<FunctionMetrics\>), file_cognitive (u64), file_cyclomatic (u64), file_sloc (u64) with Serialize/Deserialize derives in src/types.rs
- [x] T007 [P] Define AnalysisConfig struct with fields: cognitive_threshold (Option\<u64\>), include_methods (bool, default true) with Default impl in src/types.rs
- [x] T008 Define ArboristError enum with variants FileNotFound, UnsupportedLanguage, UnrecognizedExtension, LanguageNotEnabled, ParseError, Io per research.md R5, implement Display and std::error::Error in src/error.rs. Document in the Io variant's doc comment that non-UTF-8 files surface as Io(std::io::Error) with ErrorKind::InvalidData since std::fs::read_to_string enforces UTF-8
- [x] T009 Define LanguageProfile trait with methods: function_nodes, control_flow_nodes, nesting_nodes, boolean_operators, else_if_nodes, lambda_nodes, comment_nodes, extract_function_name, language, extensions per research.md R7 in src/languages/mod.rs
- [x] T010 Implement cognitive complexity calculator: traverse AST nodes, apply SonarSource rules (increment per control flow, nesting penalty, boolean operator sequences, flat else-if, lambda nesting) per research.md R2 in src/metrics/cognitive.rs
- [x] T011 [P] Implement cyclomatic complexity calculator: count decision points (if, for, while, match arms, boolean operators) with base of 1 per research.md R3 in src/metrics/cyclomatic.rs
- [x] T012 [P] Implement SLOC counter: count source lines excluding blank and comment-only lines using comment_nodes from LanguageProfile per research.md R4 in src/metrics/loc.rs
- [x] T013 Create metrics module orchestration: expose compute_metrics function that runs all three calculators on a function node in src/metrics/mod.rs
- [x] T014 Implement generic AST walker: parse source with tree-sitter, find function nodes via LanguageProfile, extract function names, compute metrics per function, aggregate file-level totals (complexity = sum of functions, SLOC = all source lines) in src/walker.rs

**Checkpoint**: Foundation ready — all generic infrastructure complete. User story implementation can now begin.

---

## Phase 3: User Story 1 — Analyze a Single Source File (Priority: P1) MVP

**Goal**: Analyze a source file by path, auto-detect language from extension, return FileReport with per-function metrics.

**Independent Test**: Pass a Rust source file with known complexity values and verify the report matches expected metrics.

### TDD: Fixtures and Tests for US1

> **Constitution Principle VI**: Write fixtures FIRST, ensure tests FAIL before implementation.

- [x] T015 [P] [US1] Create 6 Rust test fixture files with pre-calculated complexity values in tests/fixtures/rust/: simple_function.rs (1 function, cognitive=0), nested_control_flow.rs (nested if/for/while, cognitive=6+), boolean_operators.rs (mixed && and ||), else_if_chain.rs (flat else-if), closures_lambdas.rs (closures incrementing nesting), recursion.rs (direct recursive call, +1 per SonarSource spec)
- [x] T016 [US1] Write integration tests that analyze each Rust fixture file and assert exact cognitive, cyclomatic, and SLOC values match pre-calculated expectations in tests/integration/rust_analysis.rs

### Implementation for US1

- [x] T017 [US1] Implement RustProfile (LanguageProfile for Rust): function_nodes = [function_item, impl_item method], control_flow_nodes = [if_expression, for_expression, while_expression, match_expression, ...], nesting/boolean/else-if/lambda/comment nodes, extract_function_name, extensions = [".rs"] in src/languages/rust.rs
- [x] T018 [US1] Implement language detection: from_extension function that maps file extensions to Language enum and returns the corresponding LanguageProfile, with cfg-gated language availability in src/languages/mod.rs
- [x] T019 [US1] Implement analyze_file function: read file from path, detect language from extension, parse with tree-sitter, walk AST, return FileReport. Handle errors per FR-013 in src/lib.rs
- [x] T020 [US1] Implement analyze_file_with_config function: same as analyze_file but accept and pass AnalysisConfig to walker. Note: threshold logic (exceeds_threshold population) is a no-op until T049 (US4) implements it in the walker; this task only wires the config through in src/lib.rs
- [x] T021 [US1] Write integration tests for error cases: file not found, unknown extension, empty file in tests/integration/error_cases.rs

**Checkpoint**: US1 complete. `analyze_file("tests/fixtures/rust/nested_control_flow.rs")` returns correct FileReport with all metrics.

---

## Phase 4: User Story 2 — Analyze Source Code from Memory (Priority: P1)

**Goal**: Analyze source code provided as a string with explicit language, without reading from disk.

**Independent Test**: Pass a Python source string with known complexity and verify the report matches expected values.

### TDD: Fixtures and Tests for US2

- [ ] T022 [US2] Write unit tests that call analyze_source with Rust source strings (reusing fixture content) and assert metrics match the same expected values as file-based analysis in tests/integration/source_analysis.rs. Note: spec US2:AS1 references Python, but Rust is used here because Python profile is not yet available; Python is validated in Phase 5 (T046) when its profile and fixtures are integrated

### Implementation for US2

- [ ] T023 [US2] Implement analyze_source function: accept source string + Language, get LanguageProfile, parse with tree-sitter, walk AST, return FileReport with empty path in src/lib.rs
- [ ] T024 [US2] Implement analyze_source_with_config function: same as analyze_source but apply AnalysisConfig in src/lib.rs
- [ ] T025 [US2] Write tests for error cases: unsupported language, syntax errors producing best-effort results in tests/integration/source_analysis.rs

**Checkpoint**: US2 complete. `analyze_source(rust_code, Language::Rust)` produces identical metrics to `analyze_file` for the same code.

---

## Phase 5: User Story 3 + User Story 5 — Multi-Language Support & Feature Flags (Priority: P2)

**Goal**: Support all 10 Tier 1 languages with correct language detection and compile-time feature flags.

**Independent Test**: For each language, analyze a fixture file and verify metrics match expected values. Compile with a subset of features and verify unsupported languages return LanguageNotEnabled error.

> US3 (language detection) and US5 (feature flags) are combined because each language profile simultaneously provides extension mapping (US3) and is gated by its feature flag (US5).

### TDD: Fixtures for Remaining 9 Languages

> **Create 5 fixtures per language** covering: simple function, nested control flow, mixed boolean operators, else-if chains, closures/lambdas.

- [ ] T026 [P] [US3] Create 6 Python test fixtures with pre-calculated complexity values in tests/fixtures/python/: simple_function.py, nested_control_flow.py, boolean_operators.py, elif_chain.py, lambda_nested.py, recursion.py
- [ ] T027 [P] [US3] Create 6 JavaScript test fixtures with pre-calculated complexity values in tests/fixtures/javascript/: simple_function.js, nested_control_flow.js, boolean_operators.js, else_if_chain.js, arrow_functions.js, recursion.js
- [ ] T028 [P] [US3] Create 6 TypeScript test fixtures with pre-calculated complexity values in tests/fixtures/typescript/: simple_function.ts, nested_control_flow.ts, boolean_operators.ts, else_if_chain.ts, arrow_closures.tsx, recursion.ts
- [ ] T029 [P] [US3] Create 6 Java test fixtures with pre-calculated complexity values in tests/fixtures/java/: SimpleFunction.java, NestedControlFlow.java, BooleanOperators.java, ElseIfChain.java, LambdaStreams.java, Recursion.java
- [ ] T030 [P] [US3] Create 6 C# test fixtures with pre-calculated complexity values in tests/fixtures/csharp/: simple_function.cs, nested_control_flow.cs, boolean_operators.cs, else_if_chain.cs, lambda_delegates.cs, recursion.cs
- [ ] T031 [P] [US3] Create 6 C++ test fixtures with pre-calculated complexity values in tests/fixtures/cpp/: simple_function.cpp, nested_control_flow.cpp, boolean_operators.cpp, else_if_chain.cpp, lambda_captures.cpp, recursion.cpp
- [ ] T032 [P] [US3] Create 6 C test fixtures with pre-calculated complexity values in tests/fixtures/c/: simple_function.c, nested_control_flow.c, boolean_operators.c, else_if_chain.c, goto_example.c, recursion.c
- [ ] T033 [P] [US3] Create 6 Go test fixtures with pre-calculated complexity values in tests/fixtures/go/: simple_function.go, nested_control_flow.go, boolean_operators.go, else_if_chain.go, closure_goroutine.go, recursion.go
- [ ] T034 [P] [US3] Create 6 PHP test fixtures with pre-calculated complexity values in tests/fixtures/php/: simple_function.php, nested_control_flow.php, boolean_operators.php, else_if_chain.php, closure_anonymous.php, recursion.php

### Implementation: Language Profiles

- [ ] T035 [P] [US3] Implement PythonProfile: function_nodes (function_definition, decorated_definition), control flow nodes, nesting/boolean/elif/lambda/comment mappings, extensions = [".py", ".pyi"] in src/languages/python.rs
- [ ] T036 [P] [US3] Implement JavaScriptProfile: function_nodes (function_declaration, function_expression, arrow_function, method_definition), control flow nodes, JSX support, extensions = [".js", ".jsx", ".mjs", ".cjs"] in src/languages/javascript.rs
- [ ] T037 [P] [US3] Implement TypeScriptProfile: extend JavaScript node mappings for TS-specific syntax, extensions = [".ts", ".tsx", ".mts", ".cts"] in src/languages/typescript.rs
- [ ] T038 [P] [US3] Implement JavaProfile: function_nodes (method_declaration, constructor_declaration), control flow nodes, lambda_expression, extensions = [".java"] in src/languages/java.rs
- [ ] T039 [P] [US3] Implement CSharpProfile: function_nodes (method_declaration, local_function_statement), control flow nodes, lambda_expression, extensions = [".cs"] in src/languages/csharp.rs
- [ ] T040 [P] [US3] Implement CppProfile: function_nodes (function_definition), control flow nodes, lambda_expression, extensions = [".cpp", ".cc", ".cxx", ".hpp", ".hxx", ".hh"] in src/languages/cpp.rs
- [ ] T041 [P] [US3] Implement CProfile: function_nodes (function_definition), control flow nodes (no lambdas), goto node, extensions = [".c", ".h"] in src/languages/c.rs
- [ ] T042 [P] [US3] Implement GoProfile: function_nodes (function_declaration, method_declaration), control flow nodes, func_literal for closures, extensions = [".go"] in src/languages/go.rs
- [ ] T043 [P] [US3] Implement PhpProfile: function_nodes (function_definition, method_declaration), control flow nodes, anonymous_function, arrow_function, extensions = [".php"] in src/languages/php.rs

### Implementation: Feature Flags & Detection

- [ ] T044 [US5] Add cfg-gate attributes to all language profile modules and language detection match arms, ensuring LanguageNotEnabled error when feature is off in src/languages/mod.rs
- [ ] T045 [US5] Write tests: compile with subset of features, verify enabled languages work and disabled languages return LanguageNotEnabled in tests/integration/feature_flags.rs

### Integration Tests for All Languages

- [ ] T046 [US3] Write integration tests for all 9 new languages: analyze each fixture file and assert metrics match expected values in tests/integration/ (one test file per language: python_analysis.rs, javascript_analysis.rs, etc.)
- [ ] T047 [US3] Write language detection tests: verify all extensions map to correct Language variant, unknown extensions return UnrecognizedExtension, .h defaults to C in tests/integration/language_detection.rs

**Checkpoint**: All 10 Tier 1 languages analyzed correctly. Feature flags gate compilation. Extension detection complete.

---

## Phase 6: User Story 4 — Configure Analysis Thresholds (Priority: P2)

**Goal**: Allow configuring analysis behavior: cognitive complexity threshold and method inclusion filtering.

**Independent Test**: Analyze a file with threshold=8 and verify functions above/below threshold have correct exceeds_threshold values. Analyze a file with include_methods=false and verify only top-level functions appear.

### TDD: Tests for US4

- [ ] T048 [US4] Write tests: analyze Rust fixtures with cognitive_threshold=8, verify exceeds_threshold is Some(true) for complex functions and Some(false) for simple ones. Verify exceeds_threshold is None when no threshold configured in tests/integration/threshold_tests.rs
- [ ] T048b [US4] Write tests: analyze a Java fixture containing a class with methods using include_methods=false, verify only top-level functions appear in report. Verify include_methods=true (default) includes methods in tests/integration/config_tests.rs

### Implementation for US4

- [ ] T049 [US4] Implement threshold logic: after computing metrics, if AnalysisConfig.cognitive_threshold is Some(n), set exceeds_threshold = Some(func.cognitive > n) on each FunctionMetrics in src/walker.rs
- [ ] T049b [US4] Implement include_methods filtering: in the walker, when AnalysisConfig.include_methods is false, skip function nodes that are methods (determined by LanguageProfile context — e.g., nodes inside impl blocks, class bodies) per FR-018 in src/walker.rs

**Checkpoint**: US4 complete. Threshold configuration correctly flags functions. Method filtering works.

---

## Phase 7: User Story 6 — Serialize Results (Priority: P3)

**Goal**: All public result structures serializable/deserializable via serde.

**Independent Test**: Analyze a file, serialize to JSON, deserialize back, verify all fields preserved.

### TDD: Tests for US6

- [ ] T050 [US6] Write serialization round-trip tests: analyze a fixture, serialize FileReport to JSON, deserialize back, assert equality. Verify all fields (name, lines, metrics, exceeds_threshold, language) are preserved in tests/integration/serialization_tests.rs

### Implementation for US6

- [ ] T051 [US6] Add PartialEq and Eq derives on all public types (FunctionMetrics, FileReport, Language, AnalysisConfig, ArboristError) needed for round-trip test assertions. Verify existing Serialize/Deserialize derives (added in Phase 2) are complete and correct in src/types.rs and src/error.rs

**Checkpoint**: US6 complete. All results round-trip through JSON without data loss.

---

## Phase 8: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, quality enforcement, and final validation across all stories.

- [ ] T052 [P] Add doc comments with executable examples (cargo test --doc) on all public functions: analyze_file, analyze_file_with_config, analyze_source, analyze_source_with_config in src/lib.rs
- [ ] T053 [P] Add doc comments on all public types: FunctionMetrics, FileReport, Language, AnalysisConfig, ArboristError in src/types.rs and src/error.rs
- [ ] T054 [P] Add doc comment on LanguageProfile trait explaining how to implement a new language profile in src/languages/mod.rs
- [ ] T055 Run cargo clippy -- -D warnings and fix all warnings across all source files
- [ ] T056 Run cargo test --all-features and verify all tests pass
- [ ] T057 Validate quickstart.md examples compile and run correctly against the implemented library
- [ ] T058 Create README.md with: project description, installation instructions, usage examples, feature flags table, supported languages, contributing guidelines, license info
- [ ] T059 Add performance benchmark: create a large fixture file (500+ lines, 20+ functions) and a benchmark test that asserts analysis completes in under 100ms per SC-002 in tests/integration/performance_bench.rs
- [x] T060 Add #![forbid(unsafe_code)] to src/lib.rs to enforce constitution "no unsafe" rule at compile time

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 (Cargo.toml + structure)  — BLOCKS all user stories
- **US1 + US2 (Phases 3-4)**: Depend on Phase 2. US2 depends on US1 (shares analyze pipeline)
- **US3 + US5 (Phase 5)**: Depend on Phase 2 (trait + walker). Can run in parallel with US1/US2 BUT benefit from US1 being done first (proven pipeline)
- **US4 (Phase 6)**: Depends on Phase 2 (walker). Can run after Phase 3
- **US6 (Phase 7)**: Depends on Phase 2 (types). Can run anytime after types exist
- **Polish (Phase 8)**: Depends on all user stories complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no other story dependencies. **MVP target.**
- **US2 (P1)**: After Phase 2 — shares walker with US1, minor dependency on US1 lib.rs functions
- **US3 + US5 (P2)**: After Phase 2 — each language profile is independent of other profiles
- **US4 (P2)**: After Phase 2 — independent of US3/US5
- **US6 (P3)**: After Phase 2 — independent (just serde derives + tests)

### Within Each User Story

1. Fixture files (tests) FIRST — Constitution Principle VI
2. Tests written that FAIL
3. Implementation
4. Tests PASS
5. Checkpoint validation

### Parallel Opportunities

- T003 (licenses) parallel with T001-T002
- T005, T006, T007 (struct definitions) parallel with each other
- T008 (ArboristError) parallel with type definitions
- T010, T011, T012 (cognitive, cyclomatic, SLOC) parallel after trait defined
- T026-T034 (all 9 language fixtures) fully parallel
- T035-T043 (all 9 language profiles) fully parallel
- T052-T054 (doc comments) parallel with each other
- US4 and US6 can run in parallel once Phase 2 is complete

---

## Parallel Example: Phase 5 (9 Language Profiles)

```text
# All fixture creation tasks can run in parallel:
T026: Python fixtures
T027: JavaScript fixtures
T028: TypeScript fixtures
T029: Java fixtures
T030: C# fixtures
T031: C++ fixtures
T032: C fixtures
T033: Go fixtures
T034: PHP fixtures

# Then all profile implementations can run in parallel:
T035: PythonProfile
T036: JavaScriptProfile
T037: TypeScriptProfile
T038: JavaProfile
T039: CSharpProfile
T040: CppProfile
T041: CProfile
T042: GoProfile
T043: PhpProfile
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001-T003)
2. Complete Phase 2: Foundational (T004-T014)
3. Complete Phase 3: US1 — Analyze Single File with Rust (T015-T021)
4. **STOP and VALIDATE**: `cargo test` passes, Rust fixtures produce correct metrics
5. Library is usable for Rust files at this point

### Incremental Delivery

1. Setup + Foundational → core infrastructure ready
2. US1 (Rust file analysis) → MVP! Library works for one language
3. US2 (in-memory analysis) → embeddable in pipelines
4. US3 + US5 (9 more languages + feature flags) → full Tier 1 coverage
5. US4 (thresholds) → CI integration ready
6. US6 (serialization) → JSON output for tooling
7. Polish → ready for crates.io publish

### Parallel Team Strategy

With multiple developers after Phase 2 completes:

- Developer A: US1 (Rust profile) → US2 (source analysis)
- Developer B: US3 fixtures + profiles (Python, JS, TS, Java, Go)
- Developer C: US3 fixtures + profiles (C#, C++, C, PHP) + US5 (feature flags)
- Developer D: US4 (thresholds) + US6 (serialization)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Constitution mandates TDD: fixtures and failing tests BEFORE implementation
- Each fixture file must have pre-calculated expected values documented (comments in fixture or in test assertions)
- 6 fixture categories per language: simple function, nested control flow, mixed boolean operators, else-if chains, closures/lambdas, recursion
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
