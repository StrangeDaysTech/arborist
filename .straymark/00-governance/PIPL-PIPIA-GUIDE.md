# PIPL / PIPIA Guide — StrayMark

> Practical guide for completing a Personal Information Protection Impact Assessment under China's **Personal Information Protection Law** (PIPL), Articles 55-56.

**Languages**: English | [Español](i18n/es/PIPL-PIPIA-GUIDE.md) | [简体中文](i18n/zh-CN/PIPL-PIPIA-GUIDE.md)

---

## When a PIPIA is Required

Per **PIPL Art. 55**, a PIPIA must be conducted prior to:

1. Processing **sensitive personal information** (Art. 28: biometrics, religious beliefs, specific identity, medical health, financial accounts, location tracking, personal information of minors under 14).
2. Using personal information for **automated decision-making**.
3. **Entrusting** processing to a third party / providing data to other personal information handlers / **publicly disclosing** personal information.
4. **Cross-border transfer** of personal information outside mainland China.
5. Other processing activities with **significant impact** on individuals.

If any apply, set `pipl_applicable: true` on the relevant ETH / MCARD / AILOG and create a PIPIA document. Validation rule **CROSS-005** enforces the link.

---

## PIPIA vs. DPIA

PIPIA (China) and DPIA (EU GDPR Art. 35) overlap conceptually but differ in detail:

| Aspect | DPIA (GDPR) | PIPIA (PIPL) |
|--------|------------|--------------|
| Statutory basis | GDPR Art. 35 | PIPL Art. 55-56 |
| Trigger threshold | "High risk" | Any of the five Art. 55 scenarios |
| Required elements | Necessity, risks to data subjects, mitigations, DPO consultation | Lawfulness/legitimacy/necessity, personal-rights impact, security risks, proportionality of safeguards |
| Retention | Not specified by GDPR | **Minimum 3 years** (Art. 56) |
| Authority consultation | Mandatory if residual risk is high | Provincial CAC consulted for cross-border transfer |

If your organization is subject to **both** GDPR and PIPL for the same processing, the simplest approach is:
- Maintain a **DPIA** as the primary document.
- Maintain a **PIPIA** that cross-references the DPIA via `related: [DPIA-...]` and adds the PIPL-specific elements.

The DPIA template now includes a *Cross-reference: PIPIA* section that points the other way when `pipl_applicable: true`.

---

## The Three Mandatory Elements of a PIPIA Report (Art. 56)

A PIPIA must address:

### 1. Lawfulness / Legitimacy / Necessity

- **Lawful** — Identify the legal basis under PIPL Art. 13 (consent, contract performance, legal duty, emergency, news reporting, public interest, disclosed-by-individual).
- **Legitimate** — The purpose is clear, reasonable, and disclosed.
- **Necessary** — The minimum data is processed to achieve the purpose.

### 2. Personal Rights Impact + Security Risks

- Map each PIPL right (Art. 44-47, plus Art. 24 opt-out for automated decision-making) to a likelihood/severity/mitigation row.
- Identify confidentiality, integrity, availability risks. Use the same scoring scale (low/medium/high) as StrayMark's `risk_level` field.

### 3. Proportionality of Protective Measures

- Demonstrate that the measures are **lawful, effective, and commensurate**.
- Document residual risk after each measure.

---

## Cross-Border Transfer (PIPL Art. 38-40)

Cross-border transfer requires **one** of these mechanisms:

| Mechanism | When | StrayMark field |
|-----------|------|----------------|
| **CAC Security Assessment** | Required for: Critical Information Infrastructure Operators (CIIO); processors handling personal info of ≥ 1M individuals; cumulative cross-border transfer of personal info of ≥ 100,000 individuals or sensitive personal info of ≥ 10,000 individuals (in any consecutive 12-month period) | `cac_security_assessment_reference` (free-form in PIPIA body) |
| **Personal Information Protection Certification** | Voluntary alternative; granted by accredited body | `pipia_certification_reference` (free-form) |
| **Standard Contract** | Filed with provincial CAC; suitable for smaller volumes | `pipia_standard_contract_filing` (free-form) |

StrayMark does not currently model these as structured fields beyond `pipl_cross_border_transfer: true` — document the chosen mechanism in the *Cross-Border Transfer Analysis* section of the PIPIA template.

---

## Retention

Per Art. 56, the PIPIA report and the related "handling-status records" must be **retained for at least three years**. Set `pipl_retention_until: YYYY-MM-DD` to a date at least 3 years from `created`. **TYPE-003** validation enforces this.

```yaml
created: 2026-04-25
pipl_retention_until: 2029-04-25  # exactly 3 years — minimum allowed
```

---

## Sensitive Personal Information Triggers

Special care is required for these categories (Art. 28):

- Biometric identification (fingerprint, face, voice, gait)
- Religious beliefs
- Specific identity (e.g., LGBTQ+ status, criminal record)
- Medical health
- Financial accounts
- Location tracking
- Personal information of minors under 14

Processing requires **separate consent** and a PIPIA. If your model uses any of these in training or inference:
- Set `pipl_sensitive_data: true`
- Document mitigation in the PIPIA *Protective Measures* section
- Cross-reference the relevant MCARD `## Training Data` section

---

## Personal Information Protection Officer (PIPL Art. 52)

A handler that processes personal information of **more than 1,000,000 individuals** must designate a Personal Information Protection Officer. If applicable:
- Set `dpo_consulted: true` on the PIPIA after PIPO consultation
- Document the PIPO's opinion in the *Consultation* section

---

## Worked Example: Customer Chatbot

A SaaS chatbot deployed in mainland China that:
- Stores conversation history (personal info)
- Uses an LLM for automated responses (automated decision-making)
- Sends some queries to an overseas inference endpoint (cross-border)

**Triggers**: 1, 2, 4 → PIPIA required.

**Frontmatter**:
```yaml
id: PIPIA-2026-04-25-001
pipl_applicable: true
pipl_article_55_trigger: cross_border  # primary trigger
pipl_sensitive_data: false
pipl_cross_border_transfer: true
pipl_retention_until: 2029-04-25
related:
  - MCARD-2026-04-25-001  # the LLM
  - TC260RA-2026-04-25-001
  - DPIA-2026-04-25-001  # if also GDPR-subject
```

**Sections to focus on**: cross-border analysis, automated decision-making (Art. 24 opt-out), legal basis (likely consent), recipient safeguards.

<!-- StrayMark | https://strangedays.tech -->
