# NIST AI RMF --- MAP Function Implementation Guide

> **Framework**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Function**: MAP --- Context, categorization, and risk identification
>
> The MAP function establishes context for the AI system, identifies and categorizes risks, and assesses potential impacts on individuals, organizations, and society. It is the foundation for all subsequent risk management activities.

---

## MP-1: Context Establishment

Document the AI system's purpose, operating environment, stakeholders, and intended domain. Context is the basis for identifying relevant risks.

> Establishing thorough context ensures that risk assessments are grounded in the system's actual deployment conditions rather than generic assumptions.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| System purpose and scope | AILOG | `title`, Summary section |
| Operating environment | AILOG | Context section |
| Stakeholders and users | ETH | Stakeholder Analysis section |
| Regulatory context | MCARD | `regulatory_frameworks`, Regulatory Compliance section |
| Deployment constraints | ADR | Context and Constraints section |

### Implementation Checklist

- [ ] Create an AILOG entry documenting the AI system's purpose, scope, and operating context
- [ ] Identify all stakeholders (users, affected parties, operators) in the ETH document
- [ ] Record the regulatory environment and applicable frameworks in the MCARD
- [ ] Document deployment constraints and assumptions in an ADR

---

## MP-2: Categorization and Risk Classification

Classify AI systems by risk level based on their intended use, potential for harm, and the sensitivity of the domain in which they operate.

> Risk classification drives the depth of documentation and review requirements. High-risk systems demand more rigorous evidence.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Risk level classification | ETH | `risk_level` (low / medium / high / critical) |
| EU AI Act risk tier | ETH | `eu_ai_act_risk` (minimal / limited / high / unacceptable) |
| GenAI-specific risks | ETH | `nist_genai_risks` array |
| Domain sensitivity | MCARD | `intended_use`, Domain section |
| System categorization rationale | ADR | Decision and Rationale section |

### Implementation Checklist

- [ ] Assign a `risk_level` in the ETH frontmatter based on potential harm
- [ ] Map the system to an EU AI Act risk tier using the `eu_ai_act_risk` field
- [ ] Identify applicable GenAI risk categories in `nist_genai_risks` (see NIST-AI-600-1-GENAI-RISKS.md)
- [ ] Document the rationale for the chosen classification in an ADR if the level is contested or non-obvious

---

## MP-3: AI Capabilities and Limitations

Document the intended capabilities of the AI system alongside its known limitations, failure modes, and conditions under which performance degrades.

> Transparent documentation of limitations prevents overreliance and supports informed decision-making by operators and users.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Intended use and capabilities | MCARD | Intended Use section, `intended_use` |
| Known limitations | MCARD | Limitations section |
| Performance boundaries | MCARD | Performance Metrics section |
| Failure modes | TES | Edge Cases and Failure Tests section |
| Out-of-scope uses | MCARD | Out-of-Scope Uses section |

### Implementation Checklist

- [ ] Complete the MCARD Intended Use section with explicit statements of what the system is designed to do
- [ ] Document all known limitations and failure conditions in the MCARD Limitations section
- [ ] Define performance boundaries and degradation conditions in MCARD Performance Metrics
- [ ] Create TES documents that validate behavior at boundaries and under adversarial conditions

---

## MP-4: Risk Mapping and Registry

Maintain a centralized registry of identified risks, their severity, likelihood, current controls, and responsible owners.

> A living risk registry ensures risks are tracked over time, not just identified once and forgotten.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Risk catalog | AI-RISK-CATALOG.md | Full document |
| Individual risk assessments | ETH | Risk Assessment section |
| Risk severity and likelihood | AI-RISK-CATALOG.md | Severity, Likelihood columns |
| Risk owners | AI-RISK-CATALOG.md | Owner column |
| Mitigation status | AI-RISK-CATALOG.md | Status, Current Controls columns |

### Implementation Checklist

- [ ] Create or update AI-RISK-CATALOG.md with all identified risks
- [ ] Link each risk entry to the corresponding ETH document that provides detailed assessment
- [ ] Assign an owner and review date to every cataloged risk
- [ ] Schedule periodic reviews using `devtrail compliance` to verify risk catalog currency

---

## MP-5: Impact Assessment

Assess the potential impacts of the AI system on individuals (rights, safety, wellbeing), groups (equity, bias), communities, the environment, and the organization.

> Impact assessments must consider both intended effects and reasonably foreseeable unintended consequences across all affected parties.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Individual rights impact | DPIA | Rights Impact Assessment section |
| Bias and equity impact | ETH | Bias Evaluation section |
| Environmental impact | ETH | Environmental Impact section |
| Social impact | ETH | Social Impact section |
| Organizational impact | ETH | Risk Assessment section |
| Data protection impact | DPIA | Full document |

### Implementation Checklist

- [ ] Complete a DPIA for systems that process personal data or affect individual rights
- [ ] Evaluate bias and fairness impacts in the ETH Bias Evaluation section
- [ ] Assess environmental costs (compute, energy, carbon) in the ETH Environmental Impact section
- [ ] Document social and community impacts, especially for systems deployed at scale

---

## Summary: MAP Function to DevTrail Mapping

| Category | Description | Primary DevTrail Document | Key Fields / Sections |
|----------|-------------|---------------------------|----------------------|
| MP-1 | Context Establishment | AILOG, ETH | Context section, Stakeholder Analysis |
| MP-2 | Categorization | ETH | `risk_level`, `eu_ai_act_risk`, `nist_genai_risks` |
| MP-3 | AI Capabilities | MCARD | Intended Use, Limitations, Performance Metrics |
| MP-4 | Risk Mapping | AI-RISK-CATALOG.md | Severity, Likelihood, Owner, Status |
| MP-5 | Impact Assessment | DPIA, ETH | Rights Impact, Bias Evaluation, Environmental Impact |

---

*NIST AI RMF MAP Function Implementation Guide --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
