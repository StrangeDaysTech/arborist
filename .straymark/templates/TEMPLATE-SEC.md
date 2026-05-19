---
id: SEC-YYYY-MM-DD-NNN
title: "[System/Component] Security Assessment"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true
risk_level: high
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
iso_42001_clause: [6, 8]
threat_model_methodology: STRIDE  # STRIDE | PASTA | LINDDUN | custom
owasp_asvs_level: 1  # 1 | 2 | 3
tags: [security]
related: []
---

# SEC: [System/Component] Security Assessment

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval before proceeding.

## Scope and Objectives

| Field | Value |
|-------|-------|
| System Under Assessment | [Name and version of the system or component] |
| Assessment Type | [design review / code review / penetration test / threat model] |
| Assessment Date | [YYYY-MM-DD] |
| Assessor | [Agent name or human assessor] |

**Objectives**:

- [Primary objective of this security assessment]
- [Secondary objective, if applicable]

**In Scope**:

- [Component, service, or boundary included]
- [Component, service, or boundary included]

**Out of Scope**:

- [Component, service, or boundary excluded]
- [Component, service, or boundary excluded]

## Threat Model

> Methodology: **STRIDE** (Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege).
> Replace with PASTA, LINDDUN, or custom as indicated in frontmatter.

### Spoofing

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| S-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| S-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

### Tampering

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| T-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| T-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

### Repudiation

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| R-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| R-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

### Information Disclosure

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| I-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| I-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

### Denial of Service

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| D-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| D-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

### Elevation of Privilege

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| E-001 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |
| E-002 | [Threat description] | [1-5] | [1-5] | [L x I] | [Proposed mitigation] |

## OWASP ASVS Compliance

> Reference: OWASP Application Security Verification Standard (ASVS) 5.0.
> Verify controls at the level indicated in frontmatter (`owasp_asvs_level`).

| Control ID | Description | Level (L1/L2/L3) | Status | Evidence | Notes |
|------------|-------------|:-----------------:|:------:|----------|-------|
| V1.1.1 | [Architecture — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V2.1.1 | [Authentication — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V3.1.1 | [Session Management — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V4.1.1 | [Access Control — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V5.1.1 | [Validation — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V6.1.1 | [Stored Cryptography — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V7.1.1 | [Error Handling and Logging — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V8.1.1 | [Data Protection — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V9.1.1 | [Communication — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V10.1.1 | [Malicious Code — description of control] | L2 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V11.1.1 | [Business Logic — description of control] | L2 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V12.1.1 | [Files and Resources — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V13.1.1 | [API and Web Service — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V14.1.1 | [Configuration — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V50.1.1 | [OAuth and OIDC — description of control] | L1 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V51.1.1 | [Self-Contained Tokens — description of control] | L2 | [Pass / Fail / NA] | [Link or reference] | [Notes] |
| V52.1.1 | [SAML — description of control] | L2 | [Pass / Fail / NA] | [Link or reference] | [Notes] |

> Add or remove rows as needed. Focus on controls relevant to the system under assessment.

## Vulnerabilities Found

| Vuln ID | CWE | Severity (CVSS) | Description | Affected Component | Remediation | Status |
|---------|-----|:----------------:|-------------|-------------------|-------------|:------:|
| VULN-001 | [CWE-XXX] | [0.0-10.0] | [Vulnerability description] | [Component or module] | [Remediation steps] | [open / mitigated / accepted] |
| VULN-002 | [CWE-XXX] | [0.0-10.0] | [Vulnerability description] | [Component or module] | [Remediation steps] | [open / mitigated / accepted] |
| VULN-003 | [CWE-XXX] | [0.0-10.0] | [Vulnerability description] | [Component or module] | [Remediation steps] | [open / mitigated / accepted] |

## Security Controls

> Reference: OWASP Software Assurance Maturity Model (SAMM).

| Business Function | Practice | Maturity Level (1-3) | Current Status | Gaps |
|-------------------|----------|:--------------------:|----------------|------|
| Governance | Strategy and Metrics | [1-3] | [Description of current state] | [Identified gaps] |
| Governance | Policy and Compliance | [1-3] | [Description of current state] | [Identified gaps] |
| Governance | Education and Guidance | [1-3] | [Description of current state] | [Identified gaps] |
| Design | Threat Assessment | [1-3] | [Description of current state] | [Identified gaps] |
| Design | Security Requirements | [1-3] | [Description of current state] | [Identified gaps] |
| Design | Security Architecture | [1-3] | [Description of current state] | [Identified gaps] |
| Implementation | Secure Build | [1-3] | [Description of current state] | [Identified gaps] |
| Implementation | Secure Deployment | [1-3] | [Description of current state] | [Identified gaps] |
| Implementation | Defect Management | [1-3] | [Description of current state] | [Identified gaps] |
| Verification | Architecture Assessment | [1-3] | [Description of current state] | [Identified gaps] |
| Verification | Requirements-driven Testing | [1-3] | [Description of current state] | [Identified gaps] |
| Verification | Security Testing | [1-3] | [Description of current state] | [Identified gaps] |
| Operations | Incident Management | [1-3] | [Description of current state] | [Identified gaps] |
| Operations | Environment Management | [1-3] | [Description of current state] | [Identified gaps] |
| Operations | Operational Management | [1-3] | [Description of current state] | [Identified gaps] |

## Recommendations

| Priority | Description | Effort | Impact |
|:--------:|-------------|:------:|:------:|
| Critical | [Recommendation description] | [Low / Medium / High] | [Low / Medium / High] |
| High | [Recommendation description] | [Low / Medium / High] | [Low / Medium / High] |
| Medium | [Recommendation description] | [Low / Medium / High] | [Low / Medium / High] |
| Low | [Recommendation description] | [Low / Medium / High] | [Low / Medium / High] |

---

## Approval

| Field | Value |
|-------|-------|
| Approved by | [Name] |
| Date | [YYYY-MM-DD] |
| Decision | [APPROVED / REJECTED / CONDITIONAL] |
| Conditions | [If applicable] |

<!-- Template: DevTrail | https://strangedays.tech -->
