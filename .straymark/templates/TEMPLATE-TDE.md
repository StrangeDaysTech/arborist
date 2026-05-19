---
id: TDE-YYYY-MM-DD-NNN
title: [Technical debt title]
status: identified                # `identified` → `resolved` when the debt is paid (TDE-only terminal)
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high
type: code | architecture | infrastructure | documentation | testing
impact: low | medium | high
effort: low | medium | high
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
tags: []
related: []
priority: null
assigned_to: null
promoted_from_followup: null    # FU-NNN if promoted from .straymark/follow-ups-backlog.md
---

# TDE: [Technical Debt Title]

> **IDENTIFIED BY AGENT**: Prioritization and assignment require human decision.
>
> **Activation triggers** (any one suffices — file as `R<N> (new, not in Charter)` in an AILOG instead if none apply): heritage from a prior Charter, applies to multiple modules/Charters, requires a dedicated Charter outside the current scope envelope, or requires human prioritization/assignment the agent cannot decide alone. See `.straymark/00-governance/AGENT-RULES.md` §3 for the full disambiguation.

## Summary

[Brief description of the identified technical debt]

## Debt Type

- [ ] **Code**: Hard to maintain, duplicated, or poorly structured code
- [ ] **Architecture**: Suboptimal architectural decisions
- [ ] **Infrastructure**: Problematic configurations or dependencies
- [ ] **Documentation**: Missing or outdated documentation
- [ ] **Testing**: Insufficient coverage or fragile tests

## Location

| File/Component | Description |
|----------------|-------------|
| `path/to/file.ts` | [What the problem is] |
| `path/to/component/` | [What the problem is] |

## Problem Description

[Detailed description of why this is technical debt]

### Observed Symptoms
- [Symptom 1: e.g., "The file has more than 1000 lines"]
- [Symptom 2: e.g., "There are 5 functions that do almost the same thing"]

### Original Cause
[Why this debt was generated - if known]

## Impact

### On Development
- [How it affects the development team]

### On Maintenance
- [How it hinders maintenance]

### On Performance (if applicable)
- [Performance impact]

### On Security (if applicable)
- [Security risks]

## Proposed Solution

[Description of how it could be resolved]

### Recommended Approach
1. [Step 1]
2. [Step 2]
3. [Step 3]

### Alternatives
- [Alternative 1]: [Brief description]
- [Alternative 2]: [Brief description]

## Estimation

| Aspect | Value | Justification |
|--------|-------|---------------|
| Effort | [Low/Medium/High] | [Why] |
| Impact of resolving | [Low/Medium/High] | [Why] |
| Risk of not resolving | [Low/Medium/High] | [Why] |
| Urgency | [Low/Medium/High] | [Why] |

## Prioritization Matrix (for human reference)

```
         │ Low Effort  │ High Effort │
─────────┼─────────────┼─────────────┤
High     │   DO NOW    │    PLAN     │
Impact   │             │             │
─────────┼─────────────┼─────────────┤
Low      │  QUICK WIN  │  CONSIDER   │
Impact   │             │             │
```

## Dependencies

- [Other debts that should be resolved first]
- [Features that might be affected]

## Agent Notes

[Additional context, observations, or recommendations]

---

## Prioritization Decision

| Field | Value |
|-------|-------|
| Prioritized by | [Name] |
| Date | [YYYY-MM-DD] |
| Assigned priority | [P1/P2/P3/Backlog/Will not resolve] |
| Sprint/Milestone | [If applicable] |
| Assigned to | [Team/Person] |
| Comments | [Notes] |

---

## Resolution

> Fill this section AND flip `status: identified → resolved` in the frontmatter when
> the debt described here has been addressed. Keep the document on disk — `resolved`
> is the canonical TDE terminal state; the file becomes audit history rather than
> being deleted. See DOCUMENTATION-POLICY.md §3 for the lifecycle semantics.
>
> Omit this section entirely while the debt is still `identified` / `accepted` /
> superseded — it is meaningful only at the terminal transition.

| Field | Value |
|-------|-------|
| Resolved by | [Charter ID / PR / commit that paid the debt] |
| Date | [YYYY-MM-DD] |
| Verification | [How was the resolution verified — tests, drift check, audit, etc.] |
| Notes | [Anything future readers should know, e.g. partial-resolution scope] |

<!-- Template: StrayMark | https://strangedays.tech -->
