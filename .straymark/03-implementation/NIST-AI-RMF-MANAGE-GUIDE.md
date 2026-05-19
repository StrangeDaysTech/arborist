# NIST AI RMF --- MANAGE Function Implementation Guide

> **Framework**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Function**: MANAGE --- Risk response, mitigation, and monitoring
>
> The MANAGE function allocates resources and implements plans to respond to, mitigate, and monitor AI risks on an ongoing basis. It translates risk assessments into concrete actions that reduce harm and maintain trustworthiness throughout the AI system lifecycle.

---

## MG-1: Risk Response

Plan and execute responses to identified risks. Risk responses include acceptance, avoidance, transfer, or mitigation, each documented with clear rationale and accountability.

> Every identified risk must have an explicit response strategy. Undocumented risk acceptance is implicit negligence.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Risk response decisions | ADR | Decision and Rationale section |
| Recommended mitigations | ETH | Recommendations section |
| Architectural mitigations | ADR | Consequences section |
| Response prioritization | AI-RISK-CATALOG.md | Priority, Status columns |
| Decision rationale | AIDEC | Alternatives Considered, Decision section |

### Implementation Checklist

- [ ] For each risk in AI-RISK-CATALOG.md, document the chosen response strategy (accept, avoid, transfer, mitigate)
- [ ] Create an ADR for architectural decisions made to mitigate high-severity risks
- [ ] Record mitigation recommendations in the ETH Recommendations section
- [ ] Assign owners and deadlines to every risk response action

---

## MG-2: Risk Mitigation

Implement specific technical and organizational controls to reduce risk severity or likelihood. Verify that mitigations are effective and do not introduce new risks.

> Mitigations must be verified, not assumed. Implement controls, test them, and document evidence of their effectiveness.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Security controls | SEC | Controls Implemented section |
| Privacy mitigations | DPIA | Mitigation Measures section |
| Bias mitigations | ETH | Bias Evaluation, Recommendations sections |
| Technical safeguards | ADR | Implementation Details section |
| Mitigation testing | TES | Mitigation Validation tests |

### Implementation Checklist

- [ ] Document implemented security controls in SEC documents, referencing the risks they address
- [ ] Complete the DPIA Mitigation Measures section for every identified privacy risk
- [ ] Create TES documents that specifically validate the effectiveness of key mitigations
- [ ] Update AI-RISK-CATALOG.md residual risk levels after mitigations are deployed and verified

---

## MG-3: Third-Party Risk Management

Manage risks arising from third-party AI components, models, data sources, APIs, and services. Ensure supply chain transparency and establish contractual protections.

> Third-party components inherit and propagate risks. Organizations remain accountable for risks introduced by their suppliers.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Third-party inventory | SBOM | Third-Party Services, Components sections |
| Component risk assessment | SBOM | Risk Assessment section |
| Supply chain transparency | SBOM | License, Version, Source columns |
| Vendor evaluation | ETH | Third-Party Risk section |
| Dependency tracking | SBOM | Dependencies section |

### Implementation Checklist

- [ ] Maintain a current SBOM listing all third-party AI components, models, and data sources
- [ ] Assess risks for each third-party component and document them in the SBOM Risk Assessment section
- [ ] Evaluate critical vendors through dedicated ETH documents when their components affect high-risk functions
- [ ] Establish review cadences for third-party components and record review dates in the SBOM

---

## MG-4: Post-Deployment Monitoring

Monitor deployed AI systems for performance degradation, emerging risks, incidents, and changes in the operating environment. Maintain operational awareness through the full system lifecycle.

> Deployment is not the end of risk management --- it is where real-world risks begin to manifest. Continuous monitoring is essential.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Incident tracking | INC | Full document |
| Lifecycle status | AI-LIFECYCLE-TRACKER.md | Monitoring Phase section |
| Operational monitoring | AILOG | Monitoring and observability entries |
| Performance drift detection | TES | Regression and drift tests |
| System health | `devtrail status` | Documentation health metrics |

### Implementation Checklist

- [ ] Create INC documents for every operational incident, with severity classification and timeline
- [ ] Track the system's lifecycle phase in AI-LIFECYCLE-TRACKER.md and update as the system evolves
- [ ] Log monitoring-related changes and observations in AILOG entries
- [ ] Run periodic regression tests (TES) to detect performance drift in deployed systems

---

## Summary: MANAGE Function to DevTrail Mapping

| Category | Description | Primary DevTrail Document | Key Fields / Sections |
|----------|-------------|---------------------------|----------------------|
| MG-1 | Risk Response | ETH, ADR | Recommendations, Decision and Rationale |
| MG-2 | Risk Mitigation | SEC, DPIA | Controls Implemented, Mitigation Measures |
| MG-3 | Third-Party Risk | SBOM | Third-Party Services, Risk Assessment |
| MG-4 | Post-Deployment Monitoring | INC, AI-LIFECYCLE-TRACKER.md | Full document, Monitoring Phase |

---

*NIST AI RMF MANAGE Function Implementation Guide --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
