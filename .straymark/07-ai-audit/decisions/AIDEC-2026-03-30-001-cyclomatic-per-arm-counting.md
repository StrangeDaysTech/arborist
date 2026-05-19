---
id: AIDEC-2026-03-30-001
title: Implement cyclomatic per-arm counting for match/switch via optional trait methods
status: accepted
created: 2026-03-30
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: medium
supersedes: AIDEC-2026-03-29-001
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
tags: [cyclomatic-complexity, switch, match, metrics, language-profile]
related: [spec.md, research.md, src/metrics/cyclomatic.rs, src/languages/mod.rs]
---

# AIDEC: Implement cyclomatic per-arm counting for match/switch

## Context

AIDEC-2026-03-29-001 deferred per-arm switch/match cyclomatic counting to v0.2.0, assuming it required a breaking change to `LanguageProfile`. Re-analysis revealed that adding optional trait methods with default empty implementations is non-breaking in Rust, enabling the fix without a semver-major bump.

## Problem

The SonarSource/McCabe specification counts each case/arm of a switch/match as an individual decision point. The implementation counted the entire construct as +1 because `control_flow_nodes()` is shared between cognitive and cyclomatic calculators. These metrics need different behavior:

- **Cognitive**: switch/match = +1 with nesting penalty (construct-level)
- **Cyclomatic**: each arm/case = +1 (arm-level)

## Decision

Add two optional trait methods to `LanguageProfile` with empty defaults:

```rust
fn match_construct_nodes(&self) -> &[&str] { &[] }
fn match_arm_nodes(&self) -> &[&str] { &[] }
```

The cyclomatic calculator skips nodes in `match_construct_nodes()` and counts nodes in `match_arm_nodes()` as +1 each. The cognitive calculator is unchanged — it continues using `control_flow_nodes()`.

**Why non-breaking**: Adding methods with default implementations to a trait does not break existing implementations. Profiles that don't override these methods retain the previous behavior (construct counted as +1 via `control_flow_nodes`).

## Alternatives Considered

### Alternative 1: Separate `cyclomatic_decision_nodes()` list

Add a complete, parallel control flow node list for cyclomatic only. Rejected: excessive boilerplate (duplicates most nodes), harder to maintain.

### Alternative 2: Defer to v0.2.0 (previous decision)

Wait for a semver-major release. Rejected: the non-breaking approach via optional methods makes deferral unnecessary.

## Implementation

1. Added `match_construct_nodes()` and `match_arm_nodes()` to `LanguageProfile` trait with `&[]` defaults
2. Implemented overrides for all 10 language profiles with verified AST node types
3. Modified `walk_cyclomatic` to skip match constructs and count arms instead
4. Created `match_switch` test fixtures for all 10 languages
5. Added `match_switch_metrics` integration tests for all 10 languages
6. Added Python `match_statement` and PHP `match_expression` to their profiles (gap fix)
7. Updated spec.md edge case and FR-002
8. Updated research.md R3 implementation note

### Per-language arm node types

| Language   | Construct node(s)                                      | Arm node(s)                                                       |
|------------|--------------------------------------------------------|-------------------------------------------------------------------|
| Rust       | `match_expression`                                     | `match_arm`                                                       |
| Python     | `match_statement`                                      | `case_clause`                                                     |
| JavaScript | `switch_statement`                                     | `switch_case`, `switch_default`                                   |
| TypeScript | `switch_statement`                                     | `switch_case`, `switch_default`                                   |
| Java       | `switch_expression`                                    | `switch_block_statement_group`                                    |
| C#         | `switch_statement`                                     | `switch_section`                                                  |
| C++        | `switch_statement`                                     | `case_statement`                                                  |
| C          | `switch_statement`                                     | `case_statement`                                                  |
| Go         | `expression_switch_statement`, `type_switch_statement`, `select_statement` | `expression_case`, `default_case`, `type_case`, `communication_case` |
| PHP        | `switch_statement`, `match_expression`                 | `case_statement`, `default_statement`, `match_conditional_expression`, `match_default_expression` |

## Consequences

### Positive
- Cyclomatic complexity now matches SonarSource/McCabe specification for switch/match
- Non-breaking change: no semver-major bump required
- Follows established pattern (same approach as `boolean_expression_nodes()` from AIDEC-2026-03-29-002)
- 10 new test fixtures and integration tests provide coverage for switch/match

### Negative
- Cyclomatic values for switch-heavy code will increase compared to v0.1.0 behavior
- Two additional trait methods add minor complexity to `LanguageProfile`

### Gap fixes included
- Python: `match_statement` added to `control_flow_nodes` and `nesting_nodes` (Python 3.10+)
- PHP: `match_expression` added to `control_flow_nodes` and `nesting_nodes` (PHP 8.0+)

## References

- SonarSource Cognitive Complexity paper (G. Ann Campbell, 2017)
- McCabe, T.J., "A Complexity Measure" (1976)
- AIDEC-2026-03-29-001 (superseded)
- AIDEC-2026-03-29-002 (pattern precedent: optional trait methods)

---

<!-- Template: DevTrail | https://strangedays.tech -->
