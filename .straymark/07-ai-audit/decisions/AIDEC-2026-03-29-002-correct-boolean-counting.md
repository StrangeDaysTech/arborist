---
id: AIDEC-2026-03-29-002
title: Correct boolean operator counting per SonarSource specification
status: accepted
created: 2026-03-29
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: medium
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
tags: [cognitive-complexity, boolean-operators, sonarsource, metrics]
related: [src/metrics/cognitive.rs, src/languages/mod.rs, spec.md]
---

# AIDEC: Correct boolean operator counting per SonarSource specification

## Context

During `/speckit.analyze`, a critical inconsistency was found (issue C2): the cognitive complexity calculator counted each boolean operator token (`&&`, `||`, `and`, `or`) individually as +1, rather than grouping same-operator sequences as +1 per the SonarSource specification. This meant `a && b && c` produced cognitive +2 (one per `&&` token) instead of the correct +1 (one sequence of the same operator).

The root cause was that the algorithm matched on leaf token nodes (`&&`/`||`) rather than their containing expression nodes (`binary_expression`/`boolean_operator`). Since tokens have `binary_expression` as their parent (not another `&&`), the "continuation" detection always failed.

## Problem

Constitution Principle II mandates fidelity to the SonarSource cognitive complexity paper. The implementation deviated for same-operator chains:

| Expression | Expected (SonarSource) | Actual (before fix) |
|-----------|----------------------|-------------------|
| `a && b && c` | +1 (one sequence) | +2 (two tokens) |
| `a && b \|\| c` | +2 (sequence + switch) | +2 (two tokens) |

The mixed-operator case happened to produce the correct value by coincidence.

## Alternatives Considered

### Alternative 1: Document the deviation

**Description**: Keep the per-token counting and add a note to spec.md and research.md documenting it as a known simplification.

**Pros**:
- Zero code changes
- No risk of breaking anything
- Produces slightly higher (more conservative) complexity scores

**Cons**:
- Violates Constitution Principle II (fidelity to SonarSource)
- Makes the library's output incomparable with other SonarSource-compliant tools
- "Higher scores" is not always conservative — it can cause false positives in CI thresholds

### Alternative 2: Fix the engine to match SonarSource

**Description**: Rewrite the boolean handling in `cognitive.rs` to work at the expression level (`binary_expression`/`boolean_operator`) rather than the token level (`&&`/`||`). Add a new `boolean_expression_nodes()` trait method.

**Pros**:
- Correct per SonarSource specification
- Constitution Principle II compliance restored
- Output comparable with other tools (SonarQube, etc.)
- Clean design: expression-level detection with operator-switch counting

**Cons**:
- Changes cognitive values for all same-operator boolean fixtures (3 → 2)
- Requires updating 10 integration test files and 1 threshold test
- New trait method adds to the LanguageProfile interface

## Decision

**Chosen**: Alternative 2 - Fix the engine to match SonarSource

**Justification**: Constitution Principle II is clear: "la complejidad cognitiva debe implementarse fielmente según el paper de SonarSource." The deviation was not intentional — it was a consequence of working at the wrong AST level. The fix is clean and well-contained. All affected tests are predictable (only same-operator chains change, mixed-operator chains stay the same).

## Consequences

### Positive
- Cognitive complexity now matches the SonarSource specification exactly
- Output is directly comparable with SonarQube and other compliant tools
- Constitution Principle II compliance restored
- Clean trait method `boolean_expression_nodes()` with sensible default `["binary_expression"]`

### Negative
- Changed cognitive values: `check_same(a && b && c)` went from cognitive=3 to cognitive=2 across all 10 languages
- 11 test files updated (10 integration + 1 threshold)

### Risks
- Users who calibrated thresholds against the old (higher) values may need to adjust. Mitigation: this is a pre-release library (v0.1.0), no external users yet.

## Implementation

1. Added `boolean_expression_nodes()` to `LanguageProfile` trait with default `["binary_expression"]`
2. Python profile overrides to `["boolean_operator"]`
3. Rewrote boolean handling in `cognitive.rs`:
   - New `get_boolean_op()` helper extracts the operator from an expression node
   - Detection works at `binary_expression` level, not `&&`/`||` token level
   - `count_expression_switches()` replaces `count_operator_switches()` for subtree traversal
4. Updated test assertions in 11 files

## References

- G. Ann Campbell, "Cognitive Complexity: A new way of measuring understandability" (SonarSource, 2017)
- Constitution Principle II: "Correctness Over Coverage"
- `/speckit.analyze` report, issue C2

---

<!-- Template: DevTrail | https://strangedays.tech -->
