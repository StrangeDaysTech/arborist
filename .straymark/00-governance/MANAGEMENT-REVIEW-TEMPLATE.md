# Management Review Template

> **Aligned with**: ISO/IEC 42001:2023 Clause 9.3 — Management Review
>
> This template structures periodic management reviews of the AI management system. It ensures all required inputs per Clause 9.3 are considered and that decisions and actions are recorded for follow-up.
>
> **This is a template** — complete one instance per review period and archive in your project records.

---

## Review Metadata

| Field | Value |
|-------|-------|
| **Review Date** | [YYYY-MM-DD] |
| **Attendees** | [Names and roles] |
| **Review Period** | [Start date] to [End date] |
| **Next Review Date** | [YYYY-MM-DD] |
| **Facilitator** | [Name] |
| **Minutes Taken By** | [Name] |

---

## Agenda

### 1. Status of Actions from Previous Reviews

> ISO 42001 Clause 9.3 input (a): status of actions from previous management reviews.

| Action | Owner | Deadline | Status | Notes |
|--------|-------|----------|--------|-------|
| [Action from previous review] | [Owner] | [Date] | [Open / In progress / Closed] | [Notes] |
| [Action from previous review] | [Owner] | [Date] | [Open / In progress / Closed] | [Notes] |

**Discussion notes**: [Record any discussion about overdue or incomplete actions]

---

### 2. Changes in External and Internal Issues

> ISO 42001 Clause 9.3 input (b): changes in external and internal issues relevant to the AI management system.

#### 2.1 Regulatory Changes

| Change | Impact | Action Required | Owner |
|--------|--------|-----------------|-------|
| [New regulation or amendment] | [Impact on AI systems] | [Action needed] | [Owner] |

#### 2.2 Organizational Changes

| Change | Impact | Action Required | Owner |
|--------|--------|-----------------|-------|
| [Team restructuring, new systems, strategy changes] | [Impact on governance] | [Action needed] | [Owner] |

#### 2.3 Technology Changes

| Change | Impact | Action Required | Owner |
|--------|--------|-----------------|-------|
| [New AI capabilities, vendor changes, infrastructure updates] | [Impact on risk profile] | [Action needed] | [Owner] |

**Discussion notes**: [Record discussion about significant changes]

---

### 3. Information on AI System Performance

> ISO 42001 Clause 9.3 input (c): information on the performance of the AI management system.

#### 3.1 Governance Metrics (from AI-KPIS.md)

| KPI | Target | Current | Previous | Trend | Status |
|-----|--------|---------|----------|-------|--------|
| Documentation Coverage | > 80% | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Review Compliance Rate | 100% | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Mean Time to Document | < 1 day | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Risk Distribution | < 20% high/critical | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Incident Response Time | < 24h | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Compliance Score | > 75% | [Value] | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |

> **Source**: Run `devtrail metrics` to collect current values before the review.

#### 3.2 AI System Status (from AI-LIFECYCLE-TRACKER.md)

| System | Phase | Risk Level | Issues This Period |
|--------|-------|-----------|-------------------|
| [System name] | [Current phase] | [Risk level] | [Summary of issues] |

**Discussion notes**: [Record discussion about performance trends and system health]

---

### 4. Audit Results

> ISO 42001 Clause 9.3 input (d): results of audits.

#### 4.1 Internal Audit Findings

| Finding ID | Description | Severity | Corrective Action | Status |
|-----------|-------------|----------|-------------------|--------|
| [ID] | [Finding description] | [Major / Minor / Observation] | [Action taken or planned] | [Open / Closed] |

#### 4.2 External Audit Findings (if applicable)

| Finding ID | Description | Severity | Corrective Action | Status |
|-----------|-------------|----------|-------------------|--------|
| [ID] | [Finding description] | [Severity] | [Action taken or planned] | [Open / Closed] |

> **Source**: Run `devtrail compliance --all` to generate compliance report before the review.

**Discussion notes**: [Record discussion about audit findings and remediation progress]

---

### 5. Achievement of AI Objectives

> ISO 42001 Clause 9.3 input (e): information about the achievement of AI objectives.

| Objective | Target | Current Status | On Track | Notes |
|-----------|--------|----------------|----------|-------|
| [Objective from governance policy §3.2] | [Target] | [Status] | [Yes / At risk / No] | [Notes] |

> **Reference**: Objectives are defined in AI-GOVERNANCE-POLICY.md Section 3.2 (AI Objectives).

**Discussion notes**: [Record discussion about objective achievement and any needed adjustments]

---

### 6. Nonconformities and Corrective Actions

> ISO 42001 Clause 9.3 input (f): nonconformities and corrective actions.

| NC ID | Description | Root Cause | Corrective Action | Status |
|-------|-------------|-----------|-------------------|--------|
| NC-[NNN] | [Description of the nonconformity] | [Root cause analysis] | [Corrective action taken or planned] | [Open / In progress / Closed / Verified] |

> **Source**: INC documents in DevTrail and `devtrail compliance` failures.

**Discussion notes**: [Record discussion about recurring issues and systemic problems]

---

### 7. Opportunities for Improvement

> ISO 42001 Clause 9.3 input (g): opportunities for continual improvement.

| Improvement | Priority | Owner | Deadline | Expected Benefit |
|------------|----------|-------|----------|-----------------|
| [Improvement description] | [High / Medium / Low] | [Owner] | [Date] | [Expected benefit] |

**Discussion notes**: [Record discussion about improvement priorities and resource allocation]

---

## Decisions and Actions

> ISO 42001 Clause 9.3 output: decisions and actions related to continual improvement opportunities and any need for changes to the AI management system.

| Action | Owner | Deadline | Status | Priority |
|--------|-------|----------|--------|----------|
| [Action description] | [Owner] | [YYYY-MM-DD] | [Not started / In progress / Complete] | [High / Medium / Low] |

---

## Notes

[Additional notes, observations, or context from the review session]

---

## Approval

| Role | Name | Signature | Date |
|------|------|-----------|------|
| Review Facilitator | [Name] | | [Date] |
| AI Governance Lead | [Name] | | [Date] |
| Management Representative | [Name] | | [Date] |

---

*Management Review template — DevTrail Framework*
*Aligned with ISO/IEC 42001:2023 Clause 9.3*

<!-- Template: DevTrail | https://strangedays.tech -->
