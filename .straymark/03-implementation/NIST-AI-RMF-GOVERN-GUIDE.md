# NIST AI RMF --- GOVERN Function Implementation Guide

> **Framework**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Function**: GOVERN --- Organizational governance structures for AI
>
> The GOVERN function establishes and maintains the organizational structures, policies, processes, and culture necessary for responsible AI risk management. It is a cross-cutting function that informs and is informed by the MAP, MEASURE, and MANAGE functions.

---

## GV-1: Policies for AI Governance

Establish, document, and communicate organizational policies that define expectations for AI development, deployment, and use. Policies should be living documents, reviewed and updated regularly.

> Policies set the organizational baseline. Without explicit policies, teams default to inconsistent individual judgment.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| AI governance policy | AI-GOVERNANCE-POLICY.md | Full document |
| Documentation standards | DOCUMENTATION-POLICY.md | Full document |
| Agent autonomy limits | AGENT-RULES.md | Autonomy Table |
| Ethical principles | PRINCIPLES.md | Full document |
| Regulatory compliance policy | AI-GOVERNANCE-POLICY.md | Section 1.3 (Legal and Regulatory Requirements) |

### Implementation Checklist

- [ ] Customize AI-GOVERNANCE-POLICY.md for your organization's context, scope, and regulatory environment
- [ ] Adopt DOCUMENTATION-POLICY.md as the standard for all AI-related documentation
- [ ] Configure AGENT-RULES.md to reflect your organization's risk tolerance for AI agent autonomy
- [ ] Review and update all governance policies at least annually or when significant changes occur

---

## GV-2: Accountability and Roles

Define clear roles, responsibilities, and accountability structures for AI risk management. Ensure every governance function has an assigned owner.

> Accountability without assignment is an illusion. Every risk management activity needs a named responsible party.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Roles and responsibilities | AI-GOVERNANCE-POLICY.md | Section 2.2 (Roles and Responsibilities) |
| Agent autonomy boundaries | AGENT-RULES.md | Autonomy Table, Human Review Triggers |
| Decision authority | ADR | Decision Makers section |
| Review responsibilities | DOCUMENTATION-POLICY.md | Review Requirements section |
| Incident accountability | INC | Owner, Responders fields |

### Implementation Checklist

- [ ] Complete the Roles and Responsibilities table in AI-GOVERNANCE-POLICY.md Section 2.2
- [ ] Define human review triggers in AGENT-RULES.md for all high-risk activities
- [ ] Ensure every ADR identifies the decision makers and their authority
- [ ] Assign incident response roles and document them in the INC template

---

## GV-3: Workforce Diversity and Inclusion

Ensure that AI development and governance teams include diverse perspectives. Diverse teams are better at identifying risks, biases, and blind spots.

> Homogeneous teams produce homogeneous risk assessments. Diversity in perspective is a governance control, not just an HR objective.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Team composition guidance | AI-GOVERNANCE-POLICY.md | Section 4 (Support and Resources) |
| Competency requirements | AI-GOVERNANCE-POLICY.md | Section 4.2 (Competencies) |
| Inclusive review processes | DOCUMENTATION-POLICY.md | Review Requirements section |
| Stakeholder representation | ETH | Stakeholder Analysis section |

### Implementation Checklist

- [ ] Document team composition expectations in AI-GOVERNANCE-POLICY.md Section 4
- [ ] Define competency requirements that span technical, ethical, legal, and domain expertise
- [ ] Ensure ETH Stakeholder Analysis considers the perspectives of affected communities
- [ ] Include diverse reviewers in high-risk document review workflows

---

## GV-4: Organizational Culture

Foster an organizational culture that values responsible AI development, encourages raising concerns, and supports continuous learning about AI risks and ethics.

> Culture determines whether policies are followed in practice or exist only on paper. Governance is only as strong as the culture that sustains it.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Organizational principles | PRINCIPLES.md | Full document |
| Ethical guidelines | PRINCIPLES.md | Core Principles section |
| Transparency expectations | AGENT-RULES.md | Documentation requirements |
| Learning from incidents | INC | Lessons Learned section |

### Implementation Checklist

- [ ] Adopt and communicate PRINCIPLES.md as the team's shared ethical foundation
- [ ] Establish a no-blame culture for reporting AI concerns by treating INC documents as learning tools
- [ ] Integrate AI ethics awareness into onboarding using PRINCIPLES.md and AI-GOVERNANCE-POLICY.md
- [ ] Celebrate and share examples of responsible AI practices documented in AILOG and ETH records

---

## GV-5: Stakeholder Engagement

Engage internal and external stakeholders regularly in AI governance activities. Stakeholder input improves risk identification, builds trust, and ensures governance reflects diverse needs.

> Governance conducted in isolation lacks the external perspective needed to anticipate real-world impacts.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Management reviews | MANAGEMENT-REVIEW-TEMPLATE.md | Full document |
| Stakeholder feedback | MANAGEMENT-REVIEW-TEMPLATE.md | Feedback Summary section |
| External communication | AI-GOVERNANCE-POLICY.md | Section 4.4 (Communication) |
| Public transparency | MCARD | Full document (public-facing model documentation) |
| Review outcomes | MANAGEMENT-REVIEW-TEMPLATE.md | Action Items section |

### Implementation Checklist

- [ ] Schedule periodic management reviews using MANAGEMENT-REVIEW-TEMPLATE.md
- [ ] Include stakeholder feedback as a standing agenda item in management reviews
- [ ] Publish MCARD documents for externally deployed AI systems to support public transparency
- [ ] Document stakeholder engagement activities and outcomes for audit evidence

---

## GV-6: AI Supply Chain Governance

Govern the AI supply chain by maintaining transparency over third-party components, models, data sources, and services used in AI systems.

> You cannot manage what you cannot see. Supply chain governance starts with a complete and current inventory.

### DevTrail Mapping

| NIST Requirement | DevTrail Document | Section / Field |
|------------------|-------------------|-----------------|
| Component inventory | SBOM | Full document |
| Third-party services | SBOM | Third-Party Services section |
| License compliance | SBOM | License column |
| Supplier risk assessment | ETH | Third-Party Risk section |
| Supply chain policies | AI-GOVERNANCE-POLICY.md | Section 5.3 (Third-Party AI Components) |

### Implementation Checklist

- [ ] Maintain a current SBOM for every AI system, including models, libraries, APIs, and data sources
- [ ] Track license terms for all third-party components in the SBOM License column
- [ ] Conduct ETH assessments for critical suppliers whose components affect high-risk functions
- [ ] Define supply chain governance requirements in AI-GOVERNANCE-POLICY.md Section 5.3

---

## Summary: GOVERN Function to DevTrail Mapping

| Category | Description | Primary DevTrail Document | Key Fields / Sections |
|----------|-------------|---------------------------|----------------------|
| GV-1 | Policies | AI-GOVERNANCE-POLICY.md, DOCUMENTATION-POLICY.md | Full documents |
| GV-2 | Accountability | AGENT-RULES.md, AI-GOVERNANCE-POLICY.md | Autonomy Table, Section 2.2 |
| GV-3 | Workforce Diversity | AI-GOVERNANCE-POLICY.md | Section 4 |
| GV-4 | Organizational Culture | PRINCIPLES.md | Core Principles |
| GV-5 | Stakeholder Engagement | MANAGEMENT-REVIEW-TEMPLATE.md | Feedback Summary, Action Items |
| GV-6 | Supply Chain | SBOM | Full document |

---

*NIST AI RMF GOVERN Function Implementation Guide --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
