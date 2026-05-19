---
id: REQ-YYYY-MM-DD-NNN
title: [Requirement title]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium | low
review_required: true
risk_level: low | medium | high
type: functional | non-functional | constraint
priority: must | should | could | wont
stakeholder_type: end_user | operator | acquirer | regulator | maintainer | developer
observability_scope: none        # none | basic | full — set when OTel instrumentation is relevant
api_spec_path: ""               # Path to OpenAPI/AsyncAPI spec file if this requirement involves APIs
tags: []
related: []
validated_by: null
validated_date: null
---

# REQ: [Requirement Title]

> **PROPOSAL**: This requirement was proposed by an AI agent.
> It requires human validation.
>
> **Standard alignment**: ISO/IEC/IEEE 29148:2018 (Requirements Engineering)

## Description

[Clear and concise description of the requirement]

## Stakeholder Type

Identify who originates this requirement (per ISO/IEC/IEEE 29148:2018 §6.2):

- [ ] **End User** — Person who interacts with the system
- [ ] **Operator** — Person who operates the system
- [ ] **Acquirer** — Organization that acquires the system
- [ ] **Regulator** — Authority with regulatory oversight
- [ ] **Maintainer** — Person responsible for system maintenance
- [ ] **Developer** — Person who builds the system

## Requirement Type

- [ ] Functional
- [ ] Non-functional (see quality characteristics below)
- [ ] Constraint

## Priority (MoSCoW)

- [ ] **Must have** — Mandatory for MVP
- [ ] **Should have** — Important but not critical
- [ ] **Could have** — Desirable if there is time
- [ ] **Won't have** — Out of current scope

## Justification

[Why this requirement is necessary. What problem it solves or what value it provides]

## Acceptance Criteria

1. **Given** [initial context]
   **When** [user action]
   **Then** [expected result]

2. **Given** [initial context]
   **When** [user action]
   **Then** [expected result]

## Non-Functional Requirements

> Categories aligned with ISO/IEC 25010:2023. See `00-governance/ISO-25010-2023-REFERENCE.md` for full definitions.
> Complete only the sections relevant to this requirement.

### Functional Suitability

- Completeness: [Degree to which functions cover all specified tasks]
- Correctness: [Required precision of results]
- Appropriateness: [How well functions facilitate task accomplishment]

### Performance Efficiency

- Time Behaviour: [Response time, throughput requirements]
- Resource Utilization: [Memory, CPU, storage constraints]
- Capacity: [Maximum limits and load requirements]

### Compatibility

- Co-existence: [Shared environment constraints]
- Interoperability: [Systems to integrate with, data exchange formats]

### Interaction Capability

> *Renamed from "Usability" in ISO 25010:2023*

- Appropriateness Recognizability: [How easily users recognize the product fits their needs]
- Learnability: [Learning curve expectations]
- Operability: [Ease of operation requirements]
- User Error Protection: [Error prevention mechanisms]
- User Engagement: [Engagement and motivation requirements]
- Inclusivity: [Range of user characteristics to support]
- User Assistance: [Help and guidance requirements]
- Self-descriptiveness: [Self-evident capability requirements]

### Reliability

- Faultlessness: [Expected fault-free operation under normal conditions]
- Availability: [Uptime requirements, e.g., 99.9%]
- Fault Tolerance: [Behaviour under hardware/software faults]
- Recoverability: [Recovery time objective, data recovery requirements]

### Security

- Confidentiality: [Data access control requirements]
- Integrity: [Data protection requirements]
- Non-repudiation: [Audit trail requirements]
- Accountability: [Action tracing requirements]
- Authenticity: [Identity verification requirements]
- Resistance: [Attack resistance requirements]

### Maintainability

- Modularity: [Component isolation requirements]
- Reusability: [Reuse expectations]
- Analysability: [Impact assessment requirements]
- Modifiability: [Modification ease requirements]
- Testability: [Testing requirements]

### Flexibility

> *Renamed from "Portability" in ISO 25010:2023*

- Adaptability: [Environment adaptation requirements]
- Installability: [Installation requirements]
- Replaceability: [Replacement requirements]
- Scalability: [Growth/shrink handling requirements]

### Safety

> *New in ISO 25010:2023 — especially relevant for AI systems*

- Operational Constraint: [Safe operating parameters]
- Risk Identification: [Risk detection requirements]
- Fail Safe: [Safe mode/fallback requirements]
- Hazard Warning: [Warning mechanism requirements]
- Safe Integration: [Safety during integration requirements]

## Observability Requirements

> Complete this section when the project uses OpenTelemetry or has observability requirements.
> Activate with tag `observabilidad`.

| Requirement | Value | Notes |
|-------------|-------|-------|
| Coverage | [Endpoints/services that must generate traces] | [e.g., "All public API endpoints"] |
| Trace Quality | [% minimum of spans with required attributes] | [e.g., "95% of spans must have service.name, service.version"] |
| Max Trace Latency | [Maximum acceptable time for trace availability] | [e.g., "< 30s from emission to backend"] |
| Retention Policy | [Retention period by environment] | [e.g., "prod: 30 days, dev: 7 days"] |
| SLOs Linked to Observable Metrics | [SLOs that depend on OTel metrics] | [e.g., "p99 latency < 500ms measured via OTel histogram"] |

## External Interfaces

> Per ISO/IEC/IEEE 29148:2018 §9.4.2. Document interfaces with external systems.

### User Interfaces

| Interface ID | Description | Source | Protocol/Format | Data Items | Constraints |
|-------------|-------------|--------|-----------------|------------|-------------|
| UI-001 | [Description] | [Source] | [Protocol] | [Data items] | [Constraints] |

### Hardware Interfaces

| Interface ID | Description | Source | Protocol/Format | Data Items | Constraints |
|-------------|-------------|--------|-----------------|------------|-------------|
| HW-001 | [Description] | [Source] | [Protocol] | [Data items] | [Constraints] |

### Software Interfaces

| Interface ID | Description | Source | Protocol/Format | Data Items | Constraints |
|-------------|-------------|--------|-----------------|------------|-------------|
| SW-001 | [Description] | [Source] | [Protocol] | [Data items] | [Constraints] |

### Communications Interfaces

| Interface ID | Description | Source | Protocol/Format | Data Items | Constraints |
|-------------|-------------|--------|-----------------|------------|-------------|
| COM-001 | [Description] | [Source] | [Protocol] | [Data items] | [Constraints] |

## Dependencies

| Type | ID | Description |
|------|-----|-------------|
| Requires | [REQ-XXX] | [Description] |
| Blocks | [REQ-XXX] | [Description] |
| Related | [ADR-XXX] | [Description] |

## Constraints

- [Technical constraint 1]
- [Business constraint 1]
- [Regulatory constraint if applicable]

## Assumptions

- [Assumption 1]
- [Assumption 2]

## Traceability

> Per ISO/IEC/IEEE 29148:2018 §6.3. Establish traceability from stakeholder needs to system requirements to acceptance tests.

| Stakeholder Need | System Requirement | Acceptance Test |
|-----------------|-------------------|-----------------|
| [Need description] | [REQ-YYYY-MM-DD-NNN] | [TES-YYYY-MM-DD-NNN / TC-NNN] |
| [Need description] | [REQ-YYYY-MM-DD-NNN] | [TES-YYYY-MM-DD-NNN / TC-NNN] |

## Verification and Validation

> Per ISO/IEC/IEEE 29148:2018 §6.6. Define how each requirement will be verified.

| Requirement ID | Verification Method | Acceptance Criteria | Responsible |
|---------------|--------------------|--------------------|-------------|
| [REQ-XXX] | inspection \| analysis \| demonstration \| test | [Measurable criteria] | [Role/Name] |
| [REQ-XXX] | inspection \| analysis \| demonstration \| test | [Measurable criteria] | [Role/Name] |

## Agent Notes

[Additional context, questions, or suggestions from the proposing agent]

---

## Validation

| Field | Value |
|-------|-------|
| Validated by | [Name] |
| Date | [YYYY-MM-DD] |
| Status | [Validated/Rejected/Modified] |
| Comments | [Validator notes] |

<!-- Template: DevTrail | https://strangedays.tech -->
