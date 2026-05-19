# NIST AI RMF --- MEASURE Function Implementation Guide

> **Framework**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Function**: MEASURE --- Quantitative and qualitative risk assessment
>
> The MEASURE function employs quantitative and qualitative methods to analyze, assess, benchmark, and monitor AI risks and their related impacts. It provides the evidence base for informed risk management decisions.

---

## MS-1: Metrics Identification

Define appropriate metrics for evaluating AI system performance, trustworthiness, and risk. Metrics should be measurable, relevant to the system's context, and aligned with organizational objectives.

> Good metrics are specific, consistently measurable, and directly tied to the risks and trustworthiness characteristics that matter for the system.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| KPI definitions | AI-KPIS.md | Full document |
| Performance metrics | TES | Performance Metrics section |
| Accuracy and reliability metrics | MCARD | Performance Metrics section |
| Documentation health metrics | `devtrail status` | Coverage, staleness, review rates |
| Compliance metrics | `devtrail compliance` | Validation results |

### Implementation Checklist

- [ ] Define key performance indicators in AI-KPIS.md, covering accuracy, fairness, reliability, and safety
- [ ] Establish baseline measurements for each metric in TES documents
- [ ] Set thresholds and alert conditions for each metric in the MCARD Performance Metrics section
- [ ] Run `devtrail status` regularly to track documentation health as a governance metric

---

## MS-2: Trustworthiness Evaluation

Assess the AI system against trustworthiness characteristics: validity, reliability, safety, security, accountability, transparency, explainability, privacy, and fairness.

> Trustworthiness is multidimensional. A system can score well on accuracy while failing on fairness or transparency. Evaluation must cover all relevant dimensions.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Bias and fairness evaluation | ETH | Bias Evaluation section |
| Transparency assessment | ETH | Transparency section |
| Explainability assessment | ETH | Explainability section |
| Privacy evaluation | ETH | Data Privacy section, DPIA |
| Performance validation | MCARD | Performance Metrics section |
| Security assessment | SEC | Full document |
| Safety evaluation | ETH | Risk Assessment section |

### Implementation Checklist

- [ ] Complete the ETH Bias Evaluation section with quantitative fairness metrics where possible
- [ ] Document the system's transparency characteristics, including what information is available to users and operators
- [ ] Assess explainability: can the system's outputs be understood and audited by humans?
- [ ] Validate performance claims through TES documents with reproducible test results

---

## MS-3: Risk Tracking

Track identified risks over time, monitoring changes in severity, likelihood, and the effectiveness of mitigation controls. Maintain an auditable record of risk evolution.

> Risk tracking transforms a static risk register into a living governance instrument that reflects actual system behavior and changing conditions.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Risk status over time | AI-RISK-CATALOG.md | Review Date, Status columns |
| Compliance validation | `devtrail compliance` | Validation output |
| Risk level changes | ETH | `risk_level` field (tracked across versions) |
| Control effectiveness | AI-RISK-CATALOG.md | Current Controls, Residual Risk columns |
| Trend analysis | AI-KPIS.md | Trend metrics section |

### Implementation Checklist

- [ ] Set review dates for every entry in AI-RISK-CATALOG.md and honor them
- [ ] Run `devtrail compliance` as part of CI to detect documentation gaps and stale risk assessments
- [ ] Update the ETH `risk_level` field when new evidence changes the risk profile
- [ ] Record control effectiveness observations during each risk review cycle

---

## MS-4: Feedback Integration

Incorporate feedback from users, operators, incident reports, and monitoring data into the risk assessment process. Use this evidence to refine metrics and update risk evaluations.

> Feedback loops close the gap between theoretical risk assessments and observed real-world behavior. Without them, risk management becomes disconnected from reality.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Incident post-mortems | INC | Post-Mortem Analysis section |
| User feedback integration | MANAGEMENT-REVIEW-TEMPLATE.md | Feedback Summary section |
| Monitoring data | AILOG | Monitoring and Observability entries |
| Lessons learned | INC | Lessons Learned section |
| Review actions | MANAGEMENT-REVIEW-TEMPLATE.md | Action Items section |

### Implementation Checklist

- [ ] Create INC documents for every significant incident, with root cause analysis and lessons learned
- [ ] Feed incident findings back into AI-RISK-CATALOG.md to update risk severity and controls
- [ ] Use MANAGEMENT-REVIEW-TEMPLATE.md to structure periodic reviews that incorporate all feedback sources
- [ ] Update ETH documents and MCARD limitations when monitoring reveals new failure modes

---

## Summary: MEASURE Function to DevTrail Mapping

| Category | Description | Primary DevTrail Document | Key Fields / Sections |
|----------|-------------|---------------------------|----------------------|
| MS-1 | Metrics Identification | AI-KPIS.md, TES | KPI definitions, Performance Metrics |
| MS-2 | Trustworthiness Evaluation | ETH, MCARD | Bias Evaluation, Performance Metrics |
| MS-3 | Risk Tracking | AI-RISK-CATALOG.md | Review Date, Status, `devtrail compliance` |
| MS-4 | Feedback Integration | INC, MANAGEMENT-REVIEW-TEMPLATE.md | Post-Mortem Analysis, Action Items |

---

*NIST AI RMF MEASURE Function Implementation Guide --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
