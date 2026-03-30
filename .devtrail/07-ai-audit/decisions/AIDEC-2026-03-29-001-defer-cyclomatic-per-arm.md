---
id: AIDEC-2026-03-29-001
title: Defer cyclomatic per-arm counting for switch/match to v0.2.0
status: superseded
superseded_by: AIDEC-2026-03-30-001
created: 2026-03-29
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: medium
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: []
tags: [cyclomatic-complexity, switch, match, metrics]
related: [spec.md, research.md, src/metrics/cyclomatic.rs]
---

# AIDEC: Defer cyclomatic per-arm counting for switch/match to v0.2.0

## Context

During a `/speckit.analyze` cross-artifact consistency review, a discrepancy was found between the specification and the implementation regarding how switch/match constructs are counted in cyclomatic complexity. The spec (edge cases) and research.md (R3) stated "each arm counts as a decision point," but the implementation counts the entire switch/match construct as a single decision point.

## Problem

The `control_flow_nodes()` slice from `LanguageProfile` is shared between the cognitive complexity calculator and the cyclomatic complexity calculator. These two metrics need different behavior for switch/match:

- **Cognitive**: switch/match = +1 with nesting penalty (needs to stay in `control_flow_nodes`)
- **Cyclomatic (per-arm)**: individual case arms = +1 each, switch itself should not count

Implementing per-arm counting requires breaking the shared `control_flow_nodes` design or adding complex conditional logic.

## Alternatives Considered

### Alternative 1: Add `case_nodes()` trait method

**Description**: Add a new `case_nodes()` method to `LanguageProfile` returning per-case AST node types. In `cyclomatic.rs`, count cases and exclude the parent switch from the control flow count.

**Pros**:
- Correct per-arm counting
- Clean trait extension

**Cons**:
- Requires conditional logic in cyclomatic.rs to skip switch when case_nodes are present
- Needs a way to identify "switch-like" nodes (another trait method or hardcoded check)
- Removing switch from `control_flow_nodes` breaks cognitive complexity
- No existing fixtures exercise switch/match, so no validation data exists

### Alternative 2: Add separate `cyclomatic_decision_nodes()`

**Description**: Create a cyclomatic-specific control flow node list that excludes switch and includes individual case nodes.

**Pros**:
- Clean separation of concerns
- Each metric gets its own node list

**Cons**:
- Breaks the single-list simplicity of the trait design (Constitution Principle IV: profiles should be declarative)
- Significant boilerplate increase across all 10 language profiles
- Increases maintenance burden: two lists to keep in sync for most node types

### Alternative 3: Document deviation and defer to v0.2.0

**Description**: Update the spec and research documentation to accurately reflect the current behavior (switch = single decision point). Plan per-arm counting for v0.2.0 when the trait can be redesigned more carefully.

**Pros**:
- Zero code changes, zero test impact
- Honest documentation of current behavior
- Defers complexity to a point where more design thought can be applied
- Current behavior is defensible (switch-as-one-decision is a valid McCabe interpretation)

**Cons**:
- Cyclomatic values for switch-heavy code will be lower than expected by SonarSource users
- Spec deviation persists until v0.2.0

## Decision

**Chosen**: Alternative 3 - Document deviation and defer to v0.2.0

**Justification**: The shared `control_flow_nodes` design serves both cognitive and cyclomatic well for all constructs except switch/match. Breaking this shared design for one edge case introduces complexity disproportionate to the benefit, especially when no current fixtures exercise switch. The v0.2.0 release can introduce a more considered redesign (potentially Alternative 1 or 2 with proper fixtures and validation).

## Consequences

### Positive
- No code changes, no test breakage
- Documentation now accurately reflects behavior
- Maintains trait simplicity for v0.1.0

### Negative
- Cyclomatic complexity values for switch-heavy code are lower than per-arm counting would produce
- Users comparing against SonarSource cyclomatic may notice discrepancy

### Risks
- Users relying on per-arm cyclomatic for test coverage estimation may undercount required tests. Mitigation: document the deviation clearly.

## Implementation

- Updated `specs/001-code-metrics-library/spec.md` edge case for switch/match
- Updated `specs/001-code-metrics-library/research.md` R3 with implementation note
- v0.2.0 roadmap item: redesign `control_flow_nodes` sharing or add `case_nodes()` trait method

## References

- SonarSource Cognitive Complexity paper (G. Ann Campbell, 2017)
- McCabe, T.J., "A Complexity Measure" (1976)
- `/speckit.analyze` report identifying issue C3

---

<!-- Template: DevTrail | https://strangedays.tech -->
