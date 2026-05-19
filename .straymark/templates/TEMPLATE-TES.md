---
id: TES-YYYY-MM-DD-NNN
title: [Test plan title]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true
risk_level: low | medium
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
observability_scope: none        # none | basic | full — set when OTel instrumentation is relevant
tags: []
related: []
---

# TES: [Test Plan Title]

> **PROPOSAL**: This plan was created by an AI agent and requires validation.
>
> **Standard alignment**: ISO/IEC/IEEE 29119-3:2021 (Software Testing — Test Documentation)
>
> This template corresponds to the **Test Plan** level in the ISO 29119-3 hierarchy:
> - **Organizational Test Policy** — Organization-wide testing principles (out of DevTrail scope)
> - **Test Strategy** — Project-level testing strategy (may be referenced in project governance)
> - **Test Plan** — This document: specific test planning for a feature, component, or change

## Scope

### In Scope
- [Functionality/component to test 1]
- [Functionality/component to test 2]

### Out of Scope
- [What will not be tested and why]

## Test Approach

> *Per ISO/IEC/IEEE 29119-3:2021. Describes the overall approach to testing for this plan.*

### Test Design Techniques

| Technique | Application | Rationale |
|-----------|-------------|-----------|
| Equivalence Partitioning | [Where applied] | [Why chosen] |
| Boundary Value Analysis | [Where applied] | [Why chosen] |
| Decision Table | [Where applied] | [Why chosen] |
| State Transition | [Where applied] | [Why chosen] |
| Exploratory | [Where applied] | [Why chosen] |

### Test Types and Coverage

| Type | Coverage | Tool | Rationale |
|------|----------|------|-----------|
| Unit | [%] | [Jest/Vitest/etc.] | [Why this level] |
| Integration | [%] | [Tool] | [Why this level] |
| E2E | [Critical cases] | [Cypress/Playwright/etc.] | [Why these cases] |
| Performance | [If applicable] | [Tool] | [Why needed] |

### Test Completion Criteria

- [ ] [Criterion 1, e.g., "All critical test cases pass"]
- [ ] [Criterion 2, e.g., "Code coverage >= X%"]
- [ ] [Criterion 3, e.g., "No open severity-1 defects"]
- [ ] [Criterion 4, e.g., "Performance within SLA thresholds"]

### Test Suspension and Resumption Criteria

**Suspension criteria** (conditions to halt testing):
- [e.g., "Build fails to deploy to test environment"]
- [e.g., "Blocking defect found in critical path"]

**Resumption criteria** (conditions to resume testing):
- [e.g., "Blocking defect resolved and verified"]
- [e.g., "Test environment restored and stable"]

## Test Cases

### Functionality: [Name]

| ID | Case | Preconditions | Steps | Expected Result | Priority |
|----|------|---------------|-------|-----------------|----------|
| TC-001 | [Name] | [Setup] | 1. [Step] | [Expected] | High |
| TC-002 | [Name] | [Setup] | 1. [Step] | [Expected] | Medium |

### Negative Cases

| ID | Case | Invalid Input | Expected Result |
|----|------|---------------|-----------------|
| TC-N01 | [Name] | [Input] | [Expected error] |

### Edge Cases

| ID | Case | Condition | Expected Result |
|----|------|-----------|-----------------|
| TC-E01 | [Name] | [Boundary condition] | [Expected] |

## Test Data Requirements

> Per ISO/IEC/IEEE 29119-3:2021.

| Data Set ID | Source | Preparation Steps | Sensitivity Classification | Retention Policy |
|-------------|--------|-------------------|---------------------------|-----------------|
| TD-001 | [Source] | [How to prepare] | [Public/Internal/Confidential/Restricted] | [Retain/Delete after test] |
| TD-002 | [Source] | [How to prepare] | [Classification] | [Policy] |

### Required Fixtures
- [Fixture 1]: [Description]
- [Fixture 2]: [Description]

### Required Mocks
- [Mock 1]: [What it simulates]
- [Mock 2]: [What it simulates]

## Test Environment Requirements

> Per ISO/IEC/IEEE 29119-3:2021.

| Component | Version | Configuration | Dependencies |
|-----------|---------|---------------|-------------|
| [OS/Runtime] | [Version] | [Specific config] | [Required services] |
| [Database] | [Version] | [Schema/seed] | [Connectivity] |
| [External service] | [Version/API] | [Stub/live] | [Auth/network] |

- **Environment**: [Local/CI/Staging]
- **Special configuration**: [If applicable]
- **External dependencies**: [List]

## Observability Tests

> Complete this section when the project uses OpenTelemetry or has observability requirements.
> Activate with tag `observabilidad`.

| Test ID | Test Description | Expected Result | Status |
|---------|-----------------|-----------------|--------|
| OBS-01 | Verify W3C Trace Context propagation across internal service calls | `traceparent` header present in all downstream requests | [ ] |
| OBS-02 | Verify W3C Trace Context propagation across external/async calls | `traceparent` propagated through message queues and async processes | [ ] |
| OBS-03 | Validate log-trace correlation | `trace_id` and `span_id` present in structured log entries | [ ] |
| OBS-04 | Test head sampling under load | Sampling rate matches configured percentage under sustained load | [ ] |
| OBS-05 | Test tail sampling for error cases (if applicable) | Error traces are captured regardless of head sampling rate | [ ] |
| OBS-06 | Verify sensitive data redaction in Collector | PII, tokens, and secrets are redacted before reaching the backend | [ ] |

## Acceptance Criteria

- [ ] Minimum coverage of [X]%
- [ ] All critical cases pass
- [ ] No regressions in existing functionality
- [ ] Performance within acceptable thresholds

## Risks and Mitigations

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| [Risk] | [High/Medium/Low] | [High/Medium/Low] | [Action] |

## Results

> Use ISO/IEC/IEEE 29119-3:2021 terminology for test documentation artifacts.

### Test Execution Log

| TC ID | Date | Tester | Result | Actual Output | Notes |
|-------|------|--------|--------|---------------|-------|
| TC-001 | [Date] | [Agent/Human] | [Pass/Fail/Blocked] | [Actual] | [Notes] |

### Test Incident Reports

> Document any defects or unexpected behaviors found during execution.

| Incident ID | TC ID | Severity | Description | Status |
|-------------|-------|----------|-------------|--------|
| TI-001 | [TC-XXX] | [Critical/Major/Minor] | [Description] | [Open/Resolved] |

### Test Status Report

| Metric | Value |
|--------|-------|
| Total test cases | [N] |
| Passed | [N] |
| Failed | [N] |
| Blocked | [N] |
| Not executed | [N] |
| Pass rate | [%] |

### Test Completion Report

- **Completion date**: [YYYY-MM-DD]
- **Completion criteria met**: [Yes/No — list any unmet criteria]
- **Outstanding risks**: [Any residual risks from unresolved incidents]
- **Recommendation**: [Proceed to release / Hold for fixes / Additional testing needed]

---

## Validation

| Field | Value |
|-------|-------|
| Validated by | [Name] |
| Date | [YYYY-MM-DD] |
| Comments | [Notes] |

<!-- Template: DevTrail | https://strangedays.tech -->
