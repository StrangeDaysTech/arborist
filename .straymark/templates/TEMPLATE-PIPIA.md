---
id: PIPIA-YYYY-MM-DD-NNN
title: "[System/Process] Personal Information Protection Impact Assessment"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: low  # PIPIA requires extensive human judgment
review_required: true  # Always mandatory under PIPL Art. 55

# --- Approval workflow (optional, fill at review time) ---
# reviewed_by: <reviewer-id>           # email | github-handle | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
pipl_applicable: true
pipl_article_55_trigger: sensitive_data  # sensitive_data | automated_decision | third_party_disclosure | cross_border | public_disclosure | other
pipl_sensitive_data: true
pipl_cross_border_transfer: false
pipl_retention_until: YYYY-MM-DD  # Minimum 3 years from `created` per PIPL retention requirements
iso_42001_clause: [6, 8]
dpo_consulted: false
tags: [privacy, pipl, china, pipia]
related: []
---

# PIPIA: [System/Process] Personal Information Protection Impact Assessment

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval before any processing of personal information begins.
> Per PIPL Art. 56, retain this report and the related handling-status records for **at least 3 years**.

## Article 55 Trigger

Mark the trigger(s) that required this PIPIA per PIPL Art. 55:

- [ ] Processing of **sensitive personal information** (Art. 28: biometrics, religious beliefs, specific identity, medical health, financial accounts, location tracking, personal information of minors under 14, etc.)
- [ ] Use of personal information for **automated decision-making**
- [ ] **Entrusting** processing to a third party / providing data to other personal information handlers / publicly disclosing personal information
- [ ] **Cross-border transfer** of personal information
- [ ] Other processing activities with **significant impact** on individuals

## Processing Description

- **Nature of Processing**: [How data is collected, stored, used, and deleted]
- **Scope**: [Number of data subjects, volume, geographic scope inside/outside mainland China]
- **Context**: [Relationship with data subjects, reasonable expectations]
- **Purpose**: [Specific stated purpose]
- **Legal Basis (PIPL Art. 13)**: [consent / contract performance / legal duty / emergency / news reporting / public interest / disclosed-by-individual / other]
- **Categories of Data Subjects**: [Employees, customers, minors, patients, etc.]
- **Categories of Personal Data**: [Identifiers, contact, health, biometric, financial, location, etc.]
- **Sensitive Personal Information involved**: [Yes/No — list categories per Art. 28]
- **Recipients / Entrusted Processors**: [Who receives the data, contractual safeguards]
- **Retention Period**: [How long, deletion criteria, lawful basis for retention]

## Necessity and Proportionality (PIPL Art. 56 §1)

- **Lawfulness**: [Why processing is lawful under PIPL Art. 13]
- **Legitimacy**: [Why processing serves a clear, reasonable purpose]
- **Necessity**: [Why this is the minimum processing required for the purpose]
- **Purpose Limitation**: [How processing is bounded to the stated purpose]
- **Data Minimization**: [How only the minimum necessary data is collected]

## Personal Rights Impact (PIPL Art. 56 §2)

| Right Impacted | Likelihood | Severity | Risk Level | Mitigation |
|----------------|-----------|----------|------------|-----------|
| Right to know / decide (Art. 44) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |
| Right to access / copy (Art. 45) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |
| Right to correct / supplement (Art. 46) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |
| Right to delete (Art. 47) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |
| Right to portability (Art. 45) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |
| Right to opt-out of automated decision-making (Art. 24) | [Low/Med/High] | [Low/Med/High] | [Low/Med/High] | [Measures] |

## Security Risks (PIPL Art. 56 §2)

| Risk | Likelihood | Severity | Risk Level | Source | Nature of Impact |
|------|-----------|----------|------------|--------|------------------|
| [Risk 1] | [L/M/H] | [L/M/H] | [L/M/H] | [Source] | [physical/material/reputational] |
| [Risk 2] | [L/M/H] | [L/M/H] | [L/M/H] | [Source] | [physical/material/reputational] |

## Protective Measures (PIPL Art. 56 §3)

Demonstrate that protective measures are **lawful, effective, and commensurate** with the level of risk.

| Risk | Measure | Type | Residual Risk | Responsible |
|------|---------|------|---------------|-------------|
| [Risk 1] | [Mitigation measure] | [technical / organizational / contractual] | [L/M/H] | [Role/Person] |
| [Risk 2] | [Mitigation measure] | [technical / organizational / contractual] | [L/M/H] | [Role/Person] |

## Cross-Border Transfer Analysis

> Complete this section only if `pipl_cross_border_transfer: true`.
>
> Per PIPL Art. 38-40, cross-border transfer additionally requires **one** of: (a) CAC security assessment, (b) personal information protection certification, or (c) standard contract filed with provincial CAC.

- **Destination(s)**: [Countries/regions]
- **Transfer Mechanism**: [security_assessment / certification / standard_contract / other]
- **CAC Security Review Reference**: [Filing number or N/A]
- **Necessity Justification**: [Why the data must leave mainland China]
- **Recipient Safeguards**: [Contractual obligations imposed on overseas recipient]
- **Data Subject Notification**: [How affected individuals are informed and consent obtained]

## Automated Decision-Making (PIPL Art. 24)

> Complete this section if processing involves automated decision-making.

- **Decision Logic Disclosure**: [How the logic is explained to data subjects]
- **Fairness and Transparency**: [How the system is audited for unfair differentiated treatment]
- **Right to Object / Human Review**: [How individuals can request human intervention]
- **Marketing / Push Notifications**: [If used for personalized marketing, how the opt-out is provided]

## Consultation

- **Personal Information Protection Officer Opinion**: [Required when handler processes personal information of more than 1 million individuals — PIPL Art. 52]
- **Data Subjects Consulted**: [Yes/No] — [Methodology]
- **Provincial CAC Consulted**: [Yes/No / Not applicable] — [Reference]

## Retention and Review

- **PIPIA Report Retention Until**: [YYYY-MM-DD — at least 3 years from creation]
- **Next Review Date**: [YYYY-MM-DD]
- **Review Trigger Events**:
  - Change in processing scope or purpose
  - New categories of personal information
  - Regulatory changes (PIPL amendments, CAC guidance)
  - Security incidents
- **Responsible Reviewer**: [Role/Person]

## Approval

| Approved by | Date | Decision | Conditions |
|-------------|------|----------|-----------|
| [Reviewer] | [YYYY-MM-DD] | [approved / conditional / rejected] | [Conditions if any] |

<!-- Template: StrayMark | https://strangedays.tech -->
