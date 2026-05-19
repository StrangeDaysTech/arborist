---
id: AILOG-2026-03-30-002
title: "Implement cyclomatic per-arm counting for match/switch (I1)"
status: accepted
created: 2026-03-30
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: medium
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
lines_changed: ~350
files_modified:
  - src/languages/mod.rs
  - src/languages/rust.rs
  - src/languages/python.rs
  - src/languages/javascript.rs
  - src/languages/typescript.rs
  - src/languages/java.rs
  - src/languages/csharp.rs
  - src/languages/cpp.rs
  - src/languages/c.rs
  - src/languages/go.rs
  - src/languages/php.rs
  - src/metrics/cyclomatic.rs
  - specs/001-code-metrics-library/spec.md
  - specs/001-code-metrics-library/research.md
observability_scope: none
tags: [cyclomatic-complexity, switch, match, language-profile, metrics]
related: [AIDEC-2026-03-30-001, AIDEC-2026-03-29-001]
---

# AILOG: Implement cyclomatic per-arm counting for match/switch (I1)

## Summary

Resolved deferred issue I1 from v0.1.0: cyclomatic complexity now counts each match/switch arm individually instead of counting the entire construct as +1, aligning with SonarSource/McCabe specification.

## Changes

### Trait extension (non-breaking)
- Added `match_construct_nodes()` and `match_arm_nodes()` optional methods to `LanguageProfile` with `&[]` defaults
- Pattern follows precedent from AIDEC-2026-03-29-002 (`boolean_expression_nodes()`)

### Language profiles (10 files)
- Implemented both methods for all 10 supported languages with verified AST node types
- Gap fix: added Python `match_statement` to `control_flow_nodes` and `nesting_nodes`
- Gap fix: added PHP `match_expression` to `control_flow_nodes` and `nesting_nodes`

### Cyclomatic calculator
- Modified `walk_cyclomatic` to skip match constructs and count arms instead
- Cognitive calculator unchanged (still uses `control_flow_nodes` for construct-level counting)

### Tests
- Created 10 new `match_switch` fixture files (one per language)
- Added 10 new `match_switch_metrics` integration tests
- All 147 tests pass, including all existing tests (no regressions)

### Documentation
- Updated spec.md edge case and FR-002 to reflect per-arm counting
- Updated research.md R3 implementation note
- Created AIDEC-2026-03-30-001 (supersedes AIDEC-2026-03-29-001)

## Decision rationale

The key insight was that adding trait methods with default implementations is non-breaking in Rust, making the v0.2.0 deferral unnecessary. This approach maintains the declarative profile design while cleanly separating cognitive (construct-level) from cyclomatic (arm-level) counting.

---

<!-- Template: DevTrail | https://strangedays.tech -->
