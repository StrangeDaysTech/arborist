---
id: AILOG-2026-03-29-001
title: Remediation of 4 speckit.analyze findings (C1, C2, C3, C4)
status: accepted
created: 2026-03-29
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: medium
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
lines_changed: 560
files_modified:
  - src/metrics/cognitive.rs
  - src/languages/mod.rs
  - src/languages/python.rs
  - src/languages/java.rs
  - src/languages/csharp.rs
  - src/languages/php.rs
  - specs/001-code-metrics-library/spec.md
  - specs/001-code-metrics-library/research.md
  - tests/rust_analysis.rs
  - tests/threshold_tests.rs
  - tests/fixtures/rust/boolean_operators.rs
observability_scope: none
tags: [cognitive-complexity, boolean-operators, sonarsource, constitution, refactor]
related:
  - AIDEC-2026-03-29-001-defer-cyclomatic-per-arm.md
  - AIDEC-2026-03-29-002-correct-boolean-counting.md
---

# AILOG: Remediation of 4 speckit.analyze findings (C1, C2, C3, C4)

## Summary

Addressed 4 issues identified by `/speckit.analyze` cross-artifact consistency review. Two were CRITICAL (C1: constitution violation, C2: SonarSource deviation), two were HIGH (C3: cyclomatic switch counting, C4: else-if documentation). All issues resolved: C1 and C2 via code changes, C3 and C4 via documentation.

## Context

A `/speckit.analyze` run identified inconsistencies between the specification, implementation, and project constitution. The most significant were: (1) hardcoded language-specific call node types in the core metric engine violating Constitution Principle IV, and (2) boolean operator counting deviating from the SonarSource cognitive complexity specification violating Constitution Principle II.

## Actions Performed

1. **C4 (Documentation)**: Updated spec.md and research.md to clarify that flat else-if only applies to languages with dedicated syntax nodes (Python `elif_clause`, PHP `else_if_clause`). C-family languages treat `else if` as nested.

2. **C3 (Documentation + AIDEC)**: Updated spec.md and research.md to document that cyclomatic complexity counts switch/match as a single decision point (not per-arm). Created AIDEC-2026-03-29-001 documenting the deferral to v0.2.0.

3. **C1 (Refactor)**: Added `call_nodes()` and `call_function_field()` methods to the `LanguageProfile` trait with sensible defaults. Moved the hardcoded `matches!()` in `is_recursive_call()` to use trait methods. Overrides in 4 profiles: Python (`["call"]`), Java (`["method_invocation"]`, `"name"`), C# (`["invocation_expression"]`), PHP (`["function_call_expression"]`).

4. **C2 (Behavioral fix)**: Added `boolean_expression_nodes()` method to trait. Rewrote boolean handling in cognitive.rs to work at expression level (`binary_expression`/`boolean_operator`) instead of token level (`&&`/`||`). Same-operator chains now count as +1 per SonarSource spec. Updated 11 test files. Created AIDEC-2026-03-29-002.

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `src/metrics/cognitive.rs` | +45/-30 | Rewrote boolean handling + refactored is_recursive_call |
| `src/languages/mod.rs` | +17/-0 | Added 3 trait methods with defaults |
| `src/languages/python.rs` | +8/-0 | Override boolean_expression_nodes + call_nodes |
| `src/languages/java.rs` | +8/-0 | Override call_nodes + call_function_field |
| `src/languages/csharp.rs` | +4/-0 | Override call_nodes |
| `src/languages/php.rs` | +4/-0 | Override call_nodes |
| `specs/.../spec.md` | +3/-2 | C3 + C4 edge case clarifications |
| `specs/.../research.md` | +5/-1 | C3 + C4 implementation notes |
| `tests/rust_analysis.rs` | +2/-2 | check_same cognitive 3 to 2 |
| `tests/threshold_tests.rs` | +3/-3 | Adjusted threshold from 2 to 1 |
| `tests/fixtures/rust/boolean_operators.rs` | +1/-1 | Updated comment |

Additionally, 9 other language test files updated by agents (cognitive 3 to 2 for check_same/checkAll).

## Decisions Made

- **C2**: Chose to fix the engine (Option A) over documenting the deviation (Option B). See AIDEC-2026-03-29-002.
- **C3**: Chose to defer per-arm counting to v0.2.0 over implementing now. See AIDEC-2026-03-29-001.

## Impact

- **Functionality**: Boolean operator same-sequence cognitive values decrease by 1 across all 10 languages. Constitution Principle IV restored.
- **Performance**: N/A
- **Security**: N/A
- **Privacy**: N/A
- **Environmental**: N/A

## Verification

- [x] Code compiles without errors
- [x] Tests pass (125 tests, 0 failures)
- [ ] Manual review performed
- [ ] Security scan passed (if risk_level: high/critical)
- [ ] Privacy review completed (if handling PII)

## Additional Notes

The 3 new trait methods all have sensible defaults, so only profiles that deviate need overrides:
- `boolean_expression_nodes()`: default `["binary_expression"]`, Python overrides to `["boolean_operator"]`
- `call_nodes()`: default `["call_expression"]`, 4 profiles override
- `call_function_field()`: default `"function"`, only Java overrides to `"name"`

---

<!-- Template: DevTrail | https://strangedays.tech -->
