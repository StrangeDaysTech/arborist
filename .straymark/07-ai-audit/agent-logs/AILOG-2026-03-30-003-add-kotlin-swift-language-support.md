---
id: AILOG-2026-03-30-003
title: Add Kotlin and Swift language support (Tier 2)
status: accepted
created: 2026-03-30
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
lines_changed: 515
files_modified:
  - Cargo.toml
  - Cargo.lock
  - src/types.rs
  - src/languages/mod.rs
  - src/languages/kotlin.rs
  - src/languages/swift.rs
  - tests/kotlin_analysis.rs
  - tests/swift_analysis.rs
  - tests/feature_flags.rs
  - tests/language_detection.rs
  - tests/fixtures/kotlin/simple_function.kt
  - tests/fixtures/kotlin/nested_control_flow.kt
  - tests/fixtures/kotlin/boolean_operators.kt
  - tests/fixtures/kotlin/else_if_chain.kt
  - tests/fixtures/kotlin/lambda_nested.kt
  - tests/fixtures/kotlin/match_switch.kt
  - tests/fixtures/kotlin/recursion.kt
  - tests/fixtures/swift/simple_function.swift
  - tests/fixtures/swift/nested_control_flow.swift
  - tests/fixtures/swift/boolean_operators.swift
  - tests/fixtures/swift/else_if_chain.swift
  - tests/fixtures/swift/lambda_nested.swift
  - tests/fixtures/swift/match_switch.swift
  - tests/fixtures/swift/recursion.swift
observability_scope: none
tags: [language-support, tier-2, kotlin, swift, tree-sitter]
related: [AILOG-2026-03-28-001, AILOG-2026-03-30-002]
---

# AILOG: Add Kotlin and Swift language support (Tier 2)

## Summary

Added Kotlin and Swift as the first two Tier 2 languages for the Arborist code metrics library, expanding coverage from 10 to 12 languages. Both languages implement the full `LanguageProfile` trait with verified AST node types from tree-sitter grammar dumps.

## Context

The project roadmap (research.md, Table R2) defines Tier 2 languages for v0.2.0: Swift, Kotlin, Ruby, Scala, Dart, and Lua. Kotlin and Swift were selected first as mobile-ecosystem languages with strong tree-sitter grammar support. The existing LanguageProfile pattern made this a mechanical, additive task.

## Actions Performed

1. Verified dependency compatibility: `tree-sitter-kotlin-ng 1.1` and `tree-sitter-swift 0.7` both compile successfully with `tree-sitter 0.25`
2. Created temporary AST dump examples to discover exact node type names for each grammar
3. Added `Kotlin` and `Swift` variants to the `Language` enum with `Display`, `FromStr`, and `Serialize/Deserialize` support
4. Implemented `KotlinProfile` following the Java profile pattern (closest language analogue)
5. Implemented `SwiftProfile` with overridden `boolean_expression_nodes()` for Swift's `conjunction_expression`/`disjunction_expression` grammar
6. Created 7 test fixtures per language covering all metric categories
7. Created integration tests (10 per language) with exact metric value assertions
8. Updated cross-cutting test files (feature_flags.rs, language_detection.rs)

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `Cargo.toml` | +8/-1 | Added tree-sitter-kotlin-ng and tree-sitter-swift dependencies, kotlin/swift features, updated `all` feature |
| `Cargo.lock` | +22/-0 | Auto-generated lockfile update |
| `src/types.rs` | +6/-0 | Added `Kotlin` and `Swift` variants to `Language` enum, `Display`, `FromStr` |
| `src/languages/mod.rs` | +10/-0 | Added module declarations, extension mappings, profile instantiation |
| `src/languages/kotlin.rs` | +82/-0 | New: `KotlinProfile` implementing `LanguageProfile` trait |
| `src/languages/swift.rs` | +88/-0 | New: `SwiftProfile` with `boolean_expression_nodes` override |
| `tests/kotlin_analysis.rs` | +120/-0 | New: 10 integration tests for Kotlin |
| `tests/swift_analysis.rs` | +119/-0 | New: 10 integration tests for Swift |
| `tests/feature_flags.rs` | +28/-0 | Added enabled/disabled tests for Kotlin and Swift |
| `tests/language_detection.rs` | +12/-0 | Added extension detection tests for .kt and .swift |
| `tests/fixtures/kotlin/*.kt` | +7 files | Test fixtures: simple, nested, boolean, else-if, lambda, when, recursion |
| `tests/fixtures/swift/*.swift` | +7 files | Test fixtures: simple, nested, boolean, else-if, lambda, switch, recursion |

## Decisions Made

1. **tree-sitter-kotlin-ng vs tree-sitter-kotlin**: Used the `-ng` (next-generation) fork as the original crate is pinned to tree-sitter 0.20 and incompatible with 0.25. Documented in research.md.
2. **Swift boolean_expression_nodes override**: Swift's grammar uses `conjunction_expression` and `disjunction_expression` instead of `binary_expression`. Required overriding `boolean_expression_nodes()` for correct boolean operator detection.
3. **Recursion detection not implemented**: Both Kotlin-ng and Swift grammars lack a named `"function"` field on `call_expression` nodes. The current `LanguageProfile::call_function_field()` mechanism requires this field. Recursion detection is silently skipped — a known, documented limitation.
4. **Guard statement as nesting node**: Swift's `guard_statement` is treated like `if_statement` for complexity purposes (increments cognitive +1 with nesting), consistent with SonarSource algorithm behavior for conditional branches.

## Impact

- **Functionality**: Adds code metrics analysis for `.kt`, `.kts`, and `.swift` files. All 6 metric dimensions (cognitive, cyclomatic, SLOC, per-function, file-level, threshold) work correctly.
- **Performance**: N/A — new grammars are only loaded when their feature flag is enabled; no impact on existing languages.
- **Security**: N/A — pure computation library, no I/O beyond file reading.
- **Privacy**: N/A — no PII processing.
- **Environmental**: N/A.

## Verification

- [x] Code compiles without errors (`cargo check --features all`)
- [x] Tests pass — 177 total (20 new + 4 updated + 153 existing)
- [x] Clippy clean (`cargo clippy --features all` — zero warnings)
- [ ] Manual review performed
- [x] Default features still work without pulling Kotlin/Swift dependencies

## Additional Notes

- Remaining Tier 2 languages: Ruby, Scala, Dart, Lua — all follow the same mechanical pattern established here.
- The recursion detection limitation could be addressed in a future enhancement by adding a more flexible `is_recursive_call()` method to the `LanguageProfile` trait, or by walking the AST children of `call_expression` instead of relying on field names.

---

<!-- Template: DevTrail | https://strangedays.tech -->
