# AI Governance Policy

> **Aligned with**: ISO/IEC 42001:2023 — Artificial Intelligence Management System (AIMS)
>
> This document provides a governance template for organizations using DevTrail. It maps ISO 42001 clauses to DevTrail document types, enabling teams to build compliance-ready documentation as part of their development workflow.
>
> **This is a template** — adapt each section to your organization's context.

---

## 1. Scope and Context (ISO 42001 Clause 4)

> Define the boundaries and context of your AI management system.

### 1.1 Organizational Context

- **Organization**: [Organization name]
- **Industry / Sector**: [Sector]
- **AI Systems in Scope**: [List AI systems covered by this policy]
- **Exclusions**: [Systems or activities explicitly excluded]

### 1.2 Interested Parties

| Party | Needs and Expectations | Relevant Requirements |
|-------|----------------------|----------------------|
| End Users | [Expectations] | [Requirements] |
| Regulators | [Compliance expectations] | [EU AI Act, GDPR, etc.] |
| Development Team | [Expectations] | [Requirements] |
| Management | [Expectations] | [Requirements] |
| Data Subjects | [Privacy expectations] | [GDPR rights] |

### 1.3 Legal and Regulatory Requirements

| Regulation | Applicable | Status | DevTrail Evidence |
|-----------|-----------|--------|-------------------|
| EU AI Act | [Yes/No] | [Compliant/In progress/Gap] | ETH, MCARD |
| GDPR | [Yes/No] | [Status] | ETH (Data Privacy), DPIA |
| NIST AI RMF | [Yes/No] | [Status] | AI-RISK-CATALOG |
| ISO/IEC 42001 | [Yes/No] | [Status] | This document |
| [Other] | [Yes/No] | [Status] | [Documents] |

> **DevTrail mapping**: REQ documents capture regulatory requirements. ETH documents assess compliance gaps.

---

## 2. Leadership and Commitment (ISO 42001 Clause 5)

> Define the AI policy, roles, and leadership commitment.

### 2.1 AI Policy Statement

[Organization name] commits to:

- Developing and deploying AI systems responsibly and ethically
- Ensuring transparency and explainability in AI-assisted decisions
- Protecting privacy and fundamental rights of affected persons
- Maintaining human oversight of AI systems
- Continuously improving AI governance practices

### 2.2 Roles and Responsibilities

| Role | Responsibilities | DevTrail Mapping |
|------|-----------------|------------------|
| AI Governance Lead | Overall AIMS management, policy maintenance | This document, MANAGEMENT-REVIEW-TEMPLATE |
| Development Team | Documentation, implementation, testing | AILOG, AIDEC, TES |
| AI Ethics Reviewer | Ethical review and approval | ETH approval |
| Risk Manager | Risk identification and assessment | AI-RISK-CATALOG, ETH |
| Data Protection Officer | Privacy compliance, DPIA oversight | DPIA, ETH (Data Privacy) |
| AI Agents | Autonomous documentation within defined limits | Per AGENT-RULES.md autonomy table |

### 2.3 Management Commitment

- [ ] AI policy approved and communicated
- [ ] Roles and responsibilities assigned
- [ ] Resources allocated for AI governance
- [ ] Regular management reviews scheduled

> **DevTrail mapping**: ADR documents record governance decisions. AGENT-RULES.md defines agent autonomy limits.

---

## 3. Risk Planning (ISO 42001 Clause 6)

> Identify risks, set objectives, and plan for changes.

### 3.1 Risk Identification

| Risk Category | Description | Likelihood | Impact | Current Controls | DevTrail Evidence |
|--------------|-------------|-----------|--------|-----------------|-------------------|
| Bias / Fairness | [Description] | [H/M/L] | [H/M/L] | [Controls] | ETH (Bias section) |
| Privacy | [Description] | [H/M/L] | [H/M/L] | [Controls] | ETH (Privacy), DPIA |
| Security | [Description] | [H/M/L] | [H/M/L] | [Controls] | SEC |
| Safety | [Description] | [H/M/L] | [H/M/L] | [Controls] | ETH, REQ (Safety) |
| Transparency | [Description] | [H/M/L] | [H/M/L] | [Controls] | ETH (Transparency) |
| Environmental | [Description] | [H/M/L] | [H/M/L] | [Controls] | ETH (Environmental Impact) |

> **Reference**: See AI-RISK-CATALOG.md for the full risk catalog mapped to NIST AI 600-1 categories.
>
> **DevTrail mapping**: ETH documents assess individual risks. AI-RISK-CATALOG.md consolidates the organizational risk register per ISO 42001 Annex A.5.

### 3.2 AI Objectives

| Objective | Target | Measurement | Timeline | Owner |
|-----------|--------|-------------|----------|-------|
| Documentation coverage | [e.g., 100% of significant changes documented] | `devtrail metrics` | [Date] | [Owner] |
| Review compliance | [e.g., All high-risk docs reviewed within 5 days] | `devtrail metrics` | [Date] | [Owner] |
| Risk assessment coverage | [e.g., ETH for all high-risk changes] | `devtrail compliance` | [Date] | [Owner] |

### 3.3 Planning for Changes

When significant changes affect the AI management system:

1. Document the change decision in an **ADR**
2. Assess risks in an **ETH** document
3. Update this policy if governance scope changes
4. Communicate changes to all interested parties

---

## 4. Support and Resources (ISO 42001 Clause 7)

> Define resources, competencies, and communication.

### 4.1 Resources

| Resource | Description | Status |
|----------|-------------|--------|
| DevTrail Framework | Documentation governance system | [Installed/Version] |
| DevTrail CLI | Automation and validation tools | [Version] |
| AI Agent Platforms | [Claude, Gemini, Copilot, Cursor] | [Configured] |
| Training | AI governance training for team | [Status] |

### 4.2 Competencies

| Role | Required Competencies | Training Plan |
|------|----------------------|---------------|
| Developers | DevTrail usage, AI ethics basics, regulatory awareness | [Plan] |
| AI Agents | AGENT-RULES.md compliance, template usage | [Directive configuration] |
| Reviewers | Risk assessment, EU AI Act requirements, ISO 42001 basics | [Plan] |

### 4.3 Awareness

All team members must be aware of:
- This AI Governance Policy
- AGENT-RULES.md and documentation requirements
- PRINCIPLES.md and ethical guidelines
- Their role in the AI management system

### 4.4 Communication

| What | To Whom | When | Method | DevTrail Record |
|------|---------|------|--------|----------------|
| Policy updates | All team | On change | [Method] | ADR |
| Risk assessments | Reviewers | Per ETH creation | DevTrail notification | ETH |
| Incident reports | Management | Within 24h | [Method] | INC |
| Governance metrics | Management | [Monthly/Quarterly] | `devtrail metrics` | Metrics report |

### 4.5 Documented Information

DevTrail serves as the documented information system for the AIMS. Key documents:

| ISO 42001 Requirement | DevTrail Document |
|----------------------|-------------------|
| AI Policy | This document (§2) |
| Risk Assessment | ETH, AI-RISK-CATALOG.md |
| Operational procedures | AGENT-RULES.md, DOCUMENTATION-POLICY.md |
| Change records | AILOG (all changes) |
| Decision records | AIDEC, ADR |

---

## 5. AI Lifecycle Operations (ISO 42001 Clause 8)

> Define how AI systems are managed throughout their lifecycle.

### 5.1 Lifecycle Phases

| Phase | Activities | DevTrail Documents | ISO 42001 Control |
|-------|-----------|-------------------|-------------------|
| Design | Requirements, architecture decisions | REQ, ADR, AIDEC | A.6.2.2 |
| Development | Implementation, code changes | AILOG, AIDEC | A.6.2.2, A.6.2.9 |
| Testing | Validation, verification | TES | A.6.2.3, A.6.2.4 |
| Deployment | Release, configuration | AILOG | A.6.2.5 |
| Monitoring | Operations, observability | AILOG, INC | A.6.2.6 |
| Retirement | Decommission | ADR, AILOG | A.6.2.7 |

> **Reference**: See AI-LIFECYCLE-TRACKER.md for tracking system lifecycle status.

### 5.2 Documentation Requirements

Per AGENT-RULES.md, documentation is required when:

- Changes affect auth/authorization/PII → AILOG + ETH draft
- Changes in public API or DB schema → AILOG
- Changes in ML models or AI prompts → AILOG + human review
- Code above cognitive complexity threshold (run `devtrail analyze`; fallback: >20 lines) → AILOG
- Decision between 2+ alternatives → AIDEC
- Security-critical dependency changes → AILOG + human review

### 5.3 Third-Party AI Components

| Component | Provider | Purpose | Risk Level | Last Review |
|-----------|----------|---------|-----------|------------|
| [Component] | [Provider] | [Purpose] | [H/M/L] | [Date] |

> **DevTrail mapping**: SBOM documents AI supply chain. ETH assesses third-party risks.

---

## 6. Performance Evaluation (ISO 42001 Clause 9)

> Define how governance performance is monitored and reviewed.

### 6.1 Monitoring and Measurement

| KPI | Target | Measurement Method | Frequency |
|-----|--------|-------------------|-----------|
| Documentation coverage | [Target %] | `devtrail metrics` | [Frequency] |
| Review completion rate | [Target %] | `devtrail metrics` | [Frequency] |
| Mean time to document | [Target days] | `devtrail metrics` | [Frequency] |
| High-risk doc review time | [Target days] | Manual tracking | [Frequency] |
| Incident documentation | [Target %] | INC count vs incidents | [Frequency] |

> **Reference**: See AI-KPIS.md for detailed KPI definitions.

### 6.2 Internal Audit

- **Frequency**: [e.g., Quarterly]
- **Scope**: Compliance with this policy, AGENT-RULES.md, and regulatory requirements
- **Method**: `devtrail compliance --all` + manual review
- **Auditor**: [Role/Name]

### 6.3 Management Review

- **Frequency**: [e.g., Quarterly]
- **Inputs**: Governance metrics, audit results, incident reports, risk assessments
- **Outputs**: Policy updates, resource decisions, improvement actions

> **Reference**: See MANAGEMENT-REVIEW-TEMPLATE.md for the review agenda template.

---

## 7. Continual Improvement (ISO 42001 Clause 10)

> Define how nonconformities are handled and improvements driven.

### 7.1 Nonconformity and Corrective Action

When a nonconformity is identified:

1. **Document** the nonconformity (INC or AILOG with `risk_level: high`)
2. **Assess** root cause and impact
3. **Implement** corrective action
4. **Verify** effectiveness
5. **Update** governance documents if needed (ADR for policy changes)

### 7.2 Improvement Sources

| Source | DevTrail Input | Action |
|--------|---------------|--------|
| Validation failures | `devtrail validate` errors | Fix and document |
| Compliance gaps | `devtrail compliance` report | Plan remediation |
| Incident post-mortems | INC documents | Implement corrective actions |
| Management reviews | Review meeting outputs | Update policy/objectives |
| Agent feedback | AILOG with suggestions | Evaluate and prioritize |
| Regulatory changes | External monitoring | Update templates and policy |

---

## Annex: ISO 42001 Annex A Controls → DevTrail Mapping

> This mapping enables teams to demonstrate Annex A control coverage through DevTrail documentation.

| Topic Area | Control | ID | DevTrail Document(s) |
|-----------|---------|-----|---------------------|
| **A.2 Policies for AI** | AI Policy | A.2.2 | This document §2 |
| | Responsible AI Topics | A.2.3 | This document §2, PRINCIPLES.md |
| **A.3 Internal Organization** | Roles and Responsibilities | A.3.2 | This document §2, AGENT-RULES.md |
| | Reporting of AI Concerns | A.3.3 | INC, ETH |
| | Impact of Organizational Changes | A.3.4 | ADR |
| **A.4 Resources** | Resources | A.4.2 | This document §4 |
| | Competencies | A.4.3 | This document §4 |
| | Awareness of Responsible Use | A.4.4 | PRINCIPLES.md, agent directives |
| | Consultation | A.4.5 | MANAGEMENT-REVIEW-TEMPLATE |
| | Communication About AI System | A.4.6 | ADR, REQ |
| **A.5 Assessing Impacts** | Risk Assessment | A.5.2 | ETH, AI-RISK-CATALOG |
| | Impact Assessment | A.5.3 | ETH, DPIA |
| | Impact Documentation | A.5.4 | ETH, DPIA |
| **A.6 AI System Lifecycle** | Design and Development | A.6.2.2 | ADR, AIDEC |
| | Training and Testing AI Model | A.6.2.3 | MCARD, TES |
| | Verification and Validation | A.6.2.4 | TES |
| | Deployment | A.6.2.5 | AILOG, AI-LIFECYCLE-TRACKER |
| | Operation and Monitoring | A.6.2.6 | AILOG, AI-LIFECYCLE-TRACKER, OBSERVABILITY-GUIDE |
| | Retirement | A.6.2.7 | AI-LIFECYCLE-TRACKER, ADR |
| | Responsible Integration | A.6.2.8 | ADR, AIDEC |
| | Documentation | A.6.2.9 | AILOG (all changes documented) |
| | Defined Use and Misuse | A.6.2.10 | MCARD |
| | Third-Party Components | A.6.2.11 | SBOM |
| **A.7 Data for AI** | Data for Development | A.7.2 | MCARD |
| | Data Quality for ML | A.7.3 | MCARD |
| | Data Preparation | A.7.4 | MCARD |
| | Data Acquisition | A.7.5 | SBOM, DPIA |
| | Data Provenance | A.7.6 | SBOM |
| **A.8 Information for Parties** | AI Interaction Transparency | A.8.2 | ETH (Transparency) |
| | AI Outcomes Information | A.8.3 | ETH (Explainability) |
| | Access to Information | A.8.4 | REQ, ADR |
| | Enabling Human Actions | A.8.5 | AGENT-RULES.md (human review triggers) |
| **A.9 Use of AI Systems** | Objectives for Responsible Use | A.9.2 | This document §5, PRINCIPLES.md |
| | Intended Use | A.9.3 | MCARD, REQ |
| | Processes for Responsible Use | A.9.4 | DOCUMENTATION-POLICY.md, AGENT-RULES.md |
| | Human Oversight | A.9.5 | AGENT-RULES.md (autonomy table) |
| **A.10 Third-Party** | Suppliers of AI Components | A.10.2 | SBOM |
| | Shared ML Models | A.10.3 | SBOM |
| | Provision to Third Parties | A.10.4 | ETH, MCARD |

---

*AI Governance Policy template — DevTrail Framework*
*Aligned with ISO/IEC 42001:2023*

<!-- Template: DevTrail | https://strangedays.tech -->
