# AI Governance KPIs

> **Aligned with**: ISO/IEC 42001:2023 Clause 9 — Performance Evaluation
>
> This document defines key performance indicators (KPIs) for measuring the effectiveness of the AI management system. These metrics provide objective evidence for management reviews (Clause 9.3) and internal audits (Clause 9.2), and support continual improvement (Clause 10).
>
> **This is a template** — adjust targets and measurement methods to your organization's maturity and context.

---

## 1. Purpose

KPIs serve three functions within the AI management system:

- **Monitoring**: Track governance health and detect degradation early
- **Accountability**: Provide objective evidence of compliance for audits and reviews
- **Improvement**: Identify trends and areas requiring corrective action

> **DevTrail mapping**: Use `devtrail metrics` for automated KPI collection. Results feed into MANAGEMENT-REVIEW-TEMPLATE.md for periodic reviews.

---

## 2. KPI Definitions

### KPI-01: Documentation Coverage

| Field | Value |
|-------|-------|
| **KPI Name** | Documentation Coverage |
| **Description** | Percentage of AI-related changes that have associated DevTrail documentation (AILOG, AIDEC, ETH, etc.). |
| **Target** | > 80% |
| **Current Value** | [Measured value] |
| **Measurement Method** | `devtrail metrics` — ratio of documented changes to total AI-related commits |
| **Frequency** | Monthly |
| **Owner** | [Development Team Lead] |
| **ISO 42001 Reference** | Clause 9.1 (Monitoring, measurement, analysis, and evaluation) |

> **Guidance**: Start with a realistic baseline. Teams new to DevTrail may target 50% initially and increase by 10% per quarter. Changes requiring documentation are defined in AGENT-RULES.md.

---

### KPI-02: Review Compliance Rate

| Field | Value |
|-------|-------|
| **KPI Name** | Review Compliance Rate |
| **Description** | Percentage of documents requiring human review that were reviewed within the defined timeframe. |
| **Target** | 100% |
| **Current Value** | [Measured value] |
| **Measurement Method** | `devtrail metrics` — ratio of reviewed documents to documents flagged for review |
| **Frequency** | Monthly |
| **Owner** | [AI Ethics Reviewer] |
| **ISO 42001 Reference** | Clause 9.1 (Monitoring, measurement, analysis, and evaluation) |

> **Guidance**: Documents requiring review include: ETH documents, AILOG entries with `risk_level: high` or `risk_level: critical`, and changes affecting auth/PII per AGENT-RULES.md.

---

### KPI-03: Mean Time to Document

| Field | Value |
|-------|-------|
| **KPI Name** | Mean Time to Document |
| **Description** | Average number of days between a code change and the creation of its corresponding AILOG entry. |
| **Target** | < 1 day |
| **Current Value** | [Measured value] |
| **Measurement Method** | Compare git commit timestamps with AILOG creation dates in document headers |
| **Frequency** | Monthly |
| **Owner** | [Development Team Lead] |
| **ISO 42001 Reference** | Clause 9.1 (Monitoring, measurement, analysis, and evaluation) |

> **Guidance**: AI agents should create documentation in the same session as the change, targeting 0 days. Human-authored documentation should not exceed 2 business days.

---

### KPI-04: Risk Distribution

| Field | Value |
|-------|-------|
| **KPI Name** | Risk Distribution |
| **Description** | Ratio of high and critical risk entries to total entries in the AI-RISK-CATALOG. |
| **Target** | < 20% high/critical |
| **Current Value** | [Measured value] |
| **Measurement Method** | `devtrail metrics` — count of risks with score >= 10 divided by total risks |
| **Frequency** | Quarterly |
| **Owner** | [Risk Manager] |
| **ISO 42001 Reference** | Clause 6 (Planning — Actions to address risks and opportunities) |

> **Guidance**: A high ratio of critical risks indicates either immature controls or aggressive risk identification. Either way, it requires management attention. Track trend over time rather than absolute value.

---

### KPI-05: Incident Response Time

| Field | Value |
|-------|-------|
| **KPI Name** | Incident Response Time |
| **Description** | Time elapsed from incident detection to creation of a corresponding INC document in DevTrail. |
| **Target** | < 24 hours |
| **Current Value** | [Measured value] |
| **Measurement Method** | Compare incident detection timestamp (from monitoring/alerting) with INC document creation date |
| **Frequency** | Per incident (aggregated quarterly) |
| **Owner** | [Operations Lead] |
| **ISO 42001 Reference** | Clause 10.1 (Nonconformity and corrective action) |

> **Guidance**: This measures documentation response, not resolution. Faster documentation enables faster root cause analysis and prevents knowledge loss. Target should be stricter for high-severity incidents.

---

### KPI-06: Compliance Score

| Field | Value |
|-------|-------|
| **KPI Name** | Compliance Score |
| **Description** | Overall regulatory compliance percentage as measured by DevTrail's compliance validation engine. |
| **Target** | > 75% |
| **Current Value** | [Measured value] |
| **Measurement Method** | `devtrail compliance --all` — percentage of passing rules across all applicable regulations |
| **Frequency** | Quarterly |
| **Owner** | [AI Governance Lead] |
| **ISO 42001 Reference** | Clause 9.2 (Internal audit) |

> **Guidance**: The compliance engine checks documentation completeness, required fields, review status, and regulatory mappings. A score below 50% indicates significant gaps requiring immediate remediation planning.

---

## 3. KPI Summary Dashboard

| KPI | Target | Current | Trend | Status |
|-----|--------|---------|-------|--------|
| Documentation Coverage | > 80% | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Review Compliance Rate | 100% | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Mean Time to Document | < 1 day | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Risk Distribution | < 20% high/critical | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Incident Response Time | < 24h | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |
| Compliance Score | > 75% | [Value] | [Up/Down/Stable] | [On target / At risk / Below target] |

---

## 4. Measurement Schedule

| Activity | Frequency | Responsible | Output |
|----------|-----------|-------------|--------|
| Run `devtrail metrics` | Monthly | [Development Team Lead] | Updated KPI values |
| Run `devtrail compliance --all` | Quarterly | [AI Governance Lead] | Compliance score |
| Update KPI Summary Dashboard | Monthly | [AI Governance Lead] | This document (Section 3) |
| Review KPIs in management review | Quarterly | [Management] | MANAGEMENT-REVIEW-TEMPLATE |
| Reassess targets | Annually | [AI Governance Lead] | Updated targets in this document |

---

## 5. Adding Custom KPIs

Organizations may define additional KPIs. Use the following template:

### KPI-NN: [KPI Name]

| Field | Value |
|-------|-------|
| **KPI Name** | [Name] |
| **Description** | [What this KPI measures] |
| **Target** | [Target value] |
| **Current Value** | [Measured value] |
| **Measurement Method** | [How to measure] |
| **Frequency** | [How often] |
| **Owner** | [Responsible person] |
| **ISO 42001 Reference** | [Applicable clause] |

---

## 6. ISO 42001 Clause 9 Mapping

| Clause | Requirement | KPI Coverage |
|--------|-------------|-------------|
| 9.1 | Monitoring, measurement, analysis, and evaluation | KPI-01, KPI-02, KPI-03 |
| 9.2 | Internal audit | KPI-06 (Compliance Score) |
| 9.3 | Management review | All KPIs feed into MANAGEMENT-REVIEW-TEMPLATE |
| 6.1 | Actions to address risks and opportunities | KPI-04 (Risk Distribution) |
| 10.1 | Nonconformity and corrective action | KPI-05 (Incident Response Time) |

---

*AI Governance KPIs template — DevTrail Framework*
*Aligned with ISO/IEC 42001:2023 Clause 9*

<!-- Template: DevTrail | https://strangedays.tech -->
