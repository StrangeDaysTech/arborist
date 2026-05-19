# AI Risk Catalog

> **Aligned with**: NIST AI 600-1 and ISO/IEC 42001:2023 Annex C
>
> This document provides a centralized risk registry for AI systems managed under DevTrail. It maps risk entries to the 12 NIST AI 600-1 risk categories and aligns with ISO 42001 Annex A.5 (Assessing Impacts of AI Systems) and ISO/IEC 23894 (AI Risk Management).
>
> **This is a template** — populate with risks specific to your organization's AI systems.

---

## 1. Purpose

This risk catalog serves as the single source of truth for AI-related risks across the organization. It enables:

- **Centralized tracking** of all identified AI risks
- **Standardized assessment** using consistent likelihood and impact scales
- **Regulatory mapping** to NIST AI 600-1 categories and ISO 42001 controls
- **Continuous monitoring** through regular review cycles

> **DevTrail mapping**: ETH documents assess individual risks at the change level. This catalog consolidates organizational risks per ISO 42001 Annex A.5.

---

## 2. Risk Assessment Methodology

### 2.1 Scales

| Score | Likelihood | Impact |
|-------|-----------|--------|
| 1 | Rare — unlikely to occur | Negligible — minimal effect on operations |
| 2 | Unlikely — could occur in exceptional circumstances | Minor — limited effect, easily managed |
| 3 | Possible — might occur at some point | Moderate — noticeable effect, requires response |
| 4 | Likely — will probably occur in most circumstances | Major — significant effect on operations or individuals |
| 5 | Almost Certain — expected to occur regularly | Severe — critical effect, potential regulatory action |

### 2.2 Risk Score Calculation

**Risk Score = Likelihood x Impact**

| Risk Score | Risk Level | Action Required |
|-----------|-----------|-----------------|
| 1–4 | Low | Accept or monitor; document in next review cycle |
| 5–9 | Medium | Implement additional controls; review quarterly |
| 10–15 | High | Implement controls urgently; review monthly |
| 16–25 | Critical | Immediate action required; escalate to management |

### 2.3 Risk Management Phases (ISO/IEC 23894)

| Phase | Description | ISO 23894 Reference | DevTrail Evidence |
|-------|-------------|--------------------|--------------------|
| Identification | Discover and describe AI risks | Clause 6.1 | ETH documents, this catalog |
| Assessment | Evaluate likelihood, impact, and risk score | Clause 6.2 | Risk Score in this catalog |
| Treatment | Select and implement controls to mitigate risks | Clause 6.3 | Current Controls column, ADR for decisions |
| Monitoring | Track residual risk and control effectiveness | Clause 6.4 | Review Date, `devtrail metrics` |

---

## 3. Risk Register

### RISK-001 — Bias in Classification Model

| Field | Value |
|-------|-------|
| **Risk ID** | RISK-001 |
| **Category** | Bias (NIST: `bias`) |
| **Description** | Classification model produces systematically unfair outcomes for protected demographic groups due to imbalanced training data or biased feature selection. |
| **Likelihood** | 4 |
| **Impact** | 4 |
| **Risk Score** | 16 (Critical) |
| **Current Controls** | Fairness metrics evaluated during training; bias audit in ETH documents; demographic parity checks in test suite. |
| **Residual Risk** | Medium — controls reduce but do not eliminate bias in edge cases. |
| **Owner** | [AI Ethics Reviewer] |
| **Review Date** | [YYYY-MM-DD] |

> **ISO 42001 mapping**: A.5.2 (Risk Assessment), A.6.2.3 (Training and Testing), A.7.3 (Data Quality for ML)

---

### RISK-002 — Training Data Leak

| Field | Value |
|-------|-------|
| **Risk ID** | RISK-002 |
| **Category** | Privacy (NIST: `privacy`) |
| **Description** | Personally identifiable information (PII) or proprietary data included in training datasets is exposed through model outputs, memorization, or extraction attacks. |
| **Likelihood** | 3 |
| **Impact** | 5 |
| **Risk Score** | 15 (High) |
| **Current Controls** | Data anonymization pipeline; PII detection scan before training; access controls on training data; DPIA completed. |
| **Residual Risk** | Medium — extraction attacks on large models remain an evolving threat. |
| **Owner** | [Data Protection Officer] |
| **Review Date** | [YYYY-MM-DD] |

> **ISO 42001 mapping**: A.5.3 (Impact Assessment), A.7.2 (Data for Development), A.7.5 (Data Acquisition)

---

### RISK-003 — Hallucination in Text Generator

| Field | Value |
|-------|-------|
| **Risk ID** | RISK-003 |
| **Category** | Confabulation (NIST: `confabulation`) |
| **Description** | LLM-based text generator produces plausible but factually incorrect content that is presented to users or downstream systems without adequate verification. |
| **Likelihood** | 4 |
| **Impact** | 3 |
| **Risk Score** | 12 (High) |
| **Current Controls** | Human review required for all generated content in production; confidence thresholds configured; retrieval-augmented generation (RAG) with source citations. |
| **Residual Risk** | Medium — hallucinations can still pass human review in specialized domains. |
| **Owner** | [Development Team Lead] |
| **Review Date** | [YYYY-MM-DD] |

> **ISO 42001 mapping**: A.6.2.4 (Verification and Validation), A.8.3 (AI Outcomes Information), A.9.5 (Human Oversight)

---

### RISK-004 — AI Supply Chain Dependency

| Field | Value |
|-------|-------|
| **Risk ID** | RISK-004 |
| **Category** | Value Chain (NIST: `value_chain`) |
| **Description** | Critical dependency on third-party AI model provider (API, weights, or infrastructure) creates risk of service disruption, unannounced model changes, or inherited vulnerabilities. |
| **Likelihood** | 3 |
| **Impact** | 4 |
| **Risk Score** | 12 (High) |
| **Current Controls** | SBOM maintained for AI components; vendor SLA monitoring; fallback provider configured; model version pinning. |
| **Residual Risk** | Medium — vendor lock-in limits fallback options for some capabilities. |
| **Owner** | [AI Governance Lead] |
| **Review Date** | [YYYY-MM-DD] |

> **ISO 42001 mapping**: A.6.2.11 (Third-Party Components), A.10.2 (Suppliers of AI Components), A.10.3 (Shared ML Models)

---

### [RISK-NNN] — [Risk Title]

| Field | Value |
|-------|-------|
| **Risk ID** | RISK-NNN |
| **Category** | [Category] (NIST: `[nist_category]`) |
| **Description** | [Description of the risk] |
| **Likelihood** | [1-5] |
| **Impact** | [1-5] |
| **Risk Score** | [Calculated] |
| **Current Controls** | [Controls in place] |
| **Residual Risk** | [Residual risk level and justification] |
| **Owner** | [Owner] |
| **Review Date** | [YYYY-MM-DD] |

---

## 4. NIST AI 600-1 Risk Categories

> Reference table of all 12 NIST AI 600-1 risk categories. Use the **Category ID** when classifying risks in the register above.

| Category ID | Category Name | Description |
|------------|--------------|-------------|
| `cbrn` | CBRN Information | Risks related to AI enabling access to chemical, biological, radiological, or nuclear weapons information. |
| `confabulation` | Confabulation | Risks from AI generating plausible but factually incorrect or fabricated information. |
| `dangerous_content` | Dangerous Content | Risks from AI generating content that could cause physical harm or enable dangerous activities. |
| `privacy` | Data Privacy | Risks to individual privacy through data collection, inference, memorization, or unauthorized disclosure. |
| `environmental` | Environmental Impact | Risks from the environmental costs of AI training and operation, including energy consumption and carbon emissions. |
| `bias` | Harmful Bias and Homogenization | Risks from systematic discrimination or reduction of diversity in AI outputs and decisions. |
| `human_ai_config` | Human-AI Configuration | Risks arising from inappropriate levels of human oversight or over-reliance on AI system outputs. |
| `information_integrity` | Information Integrity | Risks to the integrity of information ecosystems through AI-generated misinformation or manipulation. |
| `information_security` | Information Security | Risks from AI systems being exploited through adversarial attacks, prompt injection, or model theft. |
| `intellectual_property` | Intellectual Property | Risks related to AI systems infringing on copyrights, patents, or trade secrets in training or output. |
| `obscene_content` | Obscene, Degrading, or Abusive Content | Risks from AI generating content that is sexually explicit, degrading, or abusive. |
| `value_chain` | Value Chain and Component Integration | Risks from dependencies on third-party AI components, models, or services in the AI supply chain. |

---

## 5. ISO 42001 Annex A.5 Mapping

> This catalog fulfills the following ISO 42001 Annex A.5 requirements.

| Control | Requirement | How This Catalog Addresses It |
|---------|------------|-------------------------------|
| A.5.2 | AI risk assessment | Risk register with standardized scoring methodology |
| A.5.3 | Assessing impacts of AI systems on individuals | Impact column captures effects on individuals and groups |
| A.5.4 | Documenting and reporting AI system impact assessments | Each risk entry documents assessment results with review dates |

> **Cross-references**: Individual ETH documents provide change-level risk assessments. DPIA documents address data protection impacts. This catalog provides the organizational-level consolidated view.

---

## 6. Review Schedule

| Review Type | Frequency | Next Review | Responsible |
|------------|-----------|-------------|-------------|
| Full catalog review | Quarterly | [YYYY-MM-DD] | [AI Governance Lead] |
| High/Critical risk review | Monthly | [YYYY-MM-DD] | [Risk Manager] |
| Post-incident risk update | After each INC | As needed | [Incident Owner] |
| New system onboarding | Per AI-LIFECYCLE-TRACKER | As needed | [System Owner] |

---

*AI Risk Catalog template — DevTrail Framework*
*Aligned with NIST AI 600-1 and ISO/IEC 42001:2023 Annex C*

<!-- Template: DevTrail | https://strangedays.tech -->
