---
id: ETH-YYYY-MM-DD-NNN
title: [Ethical review title]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium | low
review_required: true
risk_level: high | critical
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
gdpr_legal_basis: none          # consent | contract | legal_obligation | vital_interests | public_task | legitimate_interests | none
fria_required: false            # Fundamental Rights Impact Assessment
tags: []
related: []
approved_by: null
approved_date: null
---

# ETH: [Ethical Review Title]

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval before proceeding.

## Executive Summary

[Brief description of the ethical issue to consider]

## Context

[Describe the situation that generates ethical considerations]

## EU AI Act Risk Classification

> Reference: EU AI Act, Annex III.

| Risk Level | Obligations | Applicable |
|------------|-------------|-----------|
| **Unacceptable** | Prohibited — system cannot be deployed | [ ] |
| **High** | Full compliance required — conformity assessment, CE marking, registration, post-market monitoring | [ ] |
| **Limited** | Transparency obligations — users must be informed of AI interaction | [ ] |
| **Minimal** | No specific obligations — voluntary codes of conduct encouraged | [ ] |

- **Annex III Category** (if high-risk): [Specify: biometrics, critical infrastructure, education, employment, essential services, law enforcement, migration, justice, democratic processes]
- **Obligations checklist** (if high-risk):
  - [ ] Risk management system established (Art. 9)
  - [ ] Data governance requirements met (Art. 10)
  - [ ] Technical documentation prepared (Art. 11, Annex IV)
  - [ ] Record-keeping enabled (Art. 12)
  - [ ] Transparency information provided (Art. 13)
  - [ ] Human oversight measures in place (Art. 14)
  - [ ] Accuracy, robustness, and cybersecurity ensured (Art. 15)

## Areas of Concern

### 1. Data Privacy

- **Data involved**: [What data is processed]
- **Sensitivity**: [PII, health data, financial, etc.]
- **Jurisdictions**: [GDPR, CCPA, etc.]
- **Concerns**: [List specific concerns]

#### GDPR Legal Basis

> Per GDPR Art. 6. Complete when processing personal data.

| Processing Activity | Legal Basis (Art. 6) | Justification | Data Retention Period |
|--------------------|---------------------|---------------|----------------------|
| [Activity] | [consent/contract/legal_obligation/vital_interests/public_task/legitimate_interests] | [Why this basis applies] | [Period] |

#### Data Protection Impact Assessment Reference

- **DPIA exists**: [Yes/No]
- **DPIA document**: [DPIA-YYYY-MM-DD-NNN if applicable]

### 2. Bias and Fairness

- **Affected groups**: [Who might be affected]
- **Bias risks**: [Possible biases identified]
- **Proposed mitigations**: [How to address them]

#### Protected Characteristics

| Characteristic | Potentially Affected | Assessment | Mitigation |
|---------------|---------------------|------------|------------|
| Age | [Yes/No] | [Assessment] | [Mitigation] |
| Disability | [Yes/No] | [Assessment] | [Mitigation] |
| Gender | [Yes/No] | [Assessment] | [Mitigation] |
| Race / Ethnicity | [Yes/No] | [Assessment] | [Mitigation] |
| Religion / Belief | [Yes/No] | [Assessment] | [Mitigation] |
| Sexual Orientation | [Yes/No] | [Assessment] | [Mitigation] |
| Socioeconomic Status | [Yes/No] | [Assessment] | [Mitigation] |
| Other | [Yes/No] | [Assessment] | [Mitigation] |

### 3. Transparency

- **User communication**: [What is communicated]
- **Consent**: [How it is obtained]
- **Right to explanation**: [How it is guaranteed]

### 4. Security

- **Identified risks**: [List risks]
- **Potential impact**: [Consequences of a breach]
- **Proposed controls**: [Security measures]

### 5. Social Impact

- **Benefits**: [For whom and how]
- **Potential harms**: [To whom and how]
- **Balance**: [Cost-benefit analysis]

## Environmental Impact

> Complete when the system involves AI model training or significant compute resources.

| Metric | Value | Notes |
|--------|-------|-------|
| Training Energy Estimate (kWh) | [Value] | [Methodology] |
| CO2 Equivalent (tons) | [Value] | [Grid carbon intensity used] |
| Hardware Used | [GPUs/TPUs, count] | [Cloud provider/region] |
| Inference Cost per Request | [Value] | [Average/peak] |
| Mitigation Measures | [Description] | [Carbon offsets, efficient architectures, etc.] |

## Dual-Use Potential

> Assess whether the system could be repurposed for harmful applications.

- **Beneficial Uses**:
  - [Intended beneficial use 1]
  - [Intended beneficial use 2]
- **Potential Misuses**:
  - [Potential misuse 1]
  - [Potential misuse 2]
- **Safeguards Implemented**:
  - [Safeguard 1]
  - [Safeguard 2]
- **Residual Risk Assessment**: [Low/Medium/High — describe remaining risks after safeguards]

## Fundamental Rights Impact Assessment (FRIA)

> Required by Art. 27 EU AI Act for deployers of high-risk AI systems.
> Complete this section only if `eu_ai_act_risk` is `high` and the system is being deployed.

- **Categories of Affected Persons**: [Who is affected by the AI system's use]
- **Specific Risks to Fundamental Rights**: [Identify risks to dignity, non-discrimination, privacy, data protection, freedom of expression, etc.]
- **Period and Frequency of Use**: [How often and for how long the system will be used]
- **Governance Measures**: [Organizational measures to ensure fundamental rights]
- **Human Oversight Processes**: [How human oversight is implemented per Art. 14]

## Agent Recommendations

1. [Recommendation 1]
2. [Recommendation 2]
3. [Recommendation 3]

## Questions for the Human Reviewer

1. [Question the agent cannot resolve]
2. [Decision that requires human judgment]
3. [Aspect that needs validation]

## Review Checklist

### For the Human Reviewer

- [ ] I have read and understood the concerns raised
- [ ] I have evaluated the privacy risks
- [ ] I have considered the impact on vulnerable groups
- [ ] I have verified regulatory compliance
- [ ] I have evaluated the proposed mitigations
- [ ] I have reviewed the EU AI Act risk classification (if applicable)
- [ ] I have reviewed the FRIA (if high-risk deployment)
- [ ] I have reviewed the environmental impact assessment (if applicable)

### Decision

- [ ] **APPROVED** — Proceed as proposed
- [ ] **APPROVED WITH CONDITIONS** — Proceed with modifications
- [ ] **REJECTED** — Do not proceed
- [ ] **REQUIRES MORE ANALYSIS** — Escalate or investigate further

## Reviewer Notes

[Space for the human reviewer to document their analysis and decision]

---

## Approval

| Field | Value |
|-------|-------|
| Approved by | [Name] |
| Date | [YYYY-MM-DD] |
| Decision | [APPROVED/REJECTED/CONDITIONAL] |
| Conditions | [If applicable] |

<!-- Template: DevTrail | https://strangedays.tech -->
