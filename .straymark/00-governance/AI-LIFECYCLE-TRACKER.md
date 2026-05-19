# AI System Lifecycle Tracker

> **Aligned with**: ISO/IEC 42001:2023 Annex A.6 — AI System Lifecycle
>
> This document tracks all AI systems under governance through their lifecycle phases. It provides visibility into each system's current state, ownership, and compliance status, enabling teams to manage transitions and ensure appropriate controls at every phase.
>
> **This is a template** — register your organization's AI systems and update as they progress through lifecycle phases.

---

## 1. AI Systems Inventory

| System Name | Type | Current Phase | Version | Owner | Last Review Date | Risk Level |
|------------|------|--------------|---------|-------|-----------------|-----------|
| Acme Sentiment Classifier | Classifier (NLP) | Monitoring | 2.1.0 | [ML Team Lead] | [YYYY-MM-DD] | Medium |
| [System name] | [LLM / Classifier / Recommender / Agent / Other] | [Phase] | [Version] | [Owner] | [Date] | [Low / Medium / High / Critical] |

> **DevTrail mapping**: Each system should have corresponding ETH, MCARD, and AILOG documents. Use `devtrail status` to verify documentation coverage.

---

## 2. Lifecycle Phase Definitions

> Each phase maps to ISO 42001 Annex A.6.2 controls. The checklists below define minimum requirements before a system can transition to the next phase.

### Phase 1: Design (ISO 42001 A.6.2.2)

> Define requirements, architecture, and initial risk assessment before development begins.

- [ ] System purpose and intended use documented (REQ)
- [ ] Architecture decision recorded (ADR or AIDEC)
- [ ] Initial risk assessment completed (ETH)
- [ ] Data requirements identified
- [ ] Success criteria and metrics defined
- [ ] Stakeholders and affected parties identified
- [ ] Regulatory requirements mapped (REQ)

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Requirements specification | REQ |
| Architecture decision | ADR, AIDEC |
| Initial risk assessment | ETH |

---

### Phase 2: Training and Testing (ISO 42001 A.6.2.3)

> Build, train, and test the AI model with quality-assured data and bias evaluation.

- [ ] Training data quality verified and documented
- [ ] Data provenance recorded (SBOM)
- [ ] Bias assessment completed (ETH — Bias section)
- [ ] Model card created (MCARD)
- [ ] Training configuration documented
- [ ] Initial performance benchmarks recorded
- [ ] Data privacy assessment completed (DPIA if applicable)

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Data quality assessment | MCARD |
| Bias evaluation | ETH |
| Model documentation | MCARD |
| Data provenance | SBOM |

---

### Phase 3: Verification and Validation (ISO 42001 A.6.2.4)

> Execute test plans, validate against requirements, and confirm the system meets defined criteria.

- [ ] Test plan executed and results documented (TES)
- [ ] Performance metrics meet defined targets
- [ ] Fairness metrics evaluated across demographic groups
- [ ] Security testing completed (adversarial inputs, prompt injection)
- [ ] Edge case and failure mode analysis completed
- [ ] Explainability assessment passed
- [ ] Sign-off from designated reviewer

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Test results | TES |
| Security assessment | ETH (Security section) |
| Reviewer approval | ETH (Approval) |

---

### Phase 4: Deployment (ISO 42001 A.6.2.5)

> Release the system to production with monitoring, rollback plans, and operational documentation.

- [ ] Deployment plan documented
- [ ] Monitoring and alerting configured
- [ ] Rollback plan defined and tested
- [ ] User documentation or disclosure prepared
- [ ] Human oversight mechanisms in place
- [ ] Incident response procedures defined
- [ ] Deployment recorded (AILOG)

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Deployment record | AILOG |
| Monitoring setup | AILOG |
| Rollback plan | ADR |

---

### Phase 5: Operation and Monitoring (ISO 42001 A.6.2.6)

> Continuously monitor system performance, data drift, and compliance in production.

- [ ] SLOs defined and tracked
- [ ] Data drift monitoring active
- [ ] Model performance tracked against baseline
- [ ] Incident response procedures active
- [ ] Regular compliance checks scheduled
- [ ] User feedback collection mechanism in place
- [ ] Periodic risk reassessment scheduled

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Performance logs | AILOG |
| Incidents | INC |
| Risk reassessment | ETH |
| Compliance checks | `devtrail compliance` |

---

### Phase 6: Retirement (ISO 42001 A.6.2.7)

> Decommission the system safely with proper data handling and stakeholder notification.

- [ ] Retirement decision documented (ADR)
- [ ] Stakeholders and affected parties notified
- [ ] Data archived or disposed per retention policy
- [ ] Model artifacts archived or deleted
- [ ] API endpoints deprecated with appropriate notice period
- [ ] Replacement system documented (if applicable)
- [ ] Final status recorded in this tracker

| DevTrail Evidence | Document Type |
|-------------------|---------------|
| Retirement decision | ADR |
| Final status update | AILOG |
| Data disposition record | AILOG |

---

## 3. Example: Acme Sentiment Classifier

> Pre-filled example showing a system in the monitoring phase.

### System Details

| Field | Value |
|-------|-------|
| **System Name** | Acme Sentiment Classifier |
| **Type** | Classifier (NLP) |
| **Current Phase** | Operation and Monitoring |
| **Version** | 2.1.0 |
| **Owner** | ML Team Lead |
| **Risk Level** | Medium |
| **Deployment Date** | 2025-09-15 |
| **Last Review** | [YYYY-MM-DD] |
| **Next Review** | [YYYY-MM-DD] |

### Phase Completion History

| Phase | Completed | Date | Key Documents |
|-------|-----------|------|--------------|
| Design | Yes | 2025-03-10 | REQ-001, ADR-012, ETH-008 |
| Training/Testing | Yes | 2025-06-20 | MCARD-003, ETH-008 (updated) |
| Verification/Validation | Yes | 2025-08-05 | TES-015, TES-016 |
| Deployment | Yes | 2025-09-15 | AILOG-042 |
| Operation/Monitoring | Active | — | AILOG-050, INC-003 |
| Retirement | — | — | — |

### Current Monitoring Status

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Accuracy | > 92% | 94.1% | On target |
| Fairness (demographic parity) | < 5% gap | 3.2% gap | On target |
| Latency (p95) | < 200ms | 145ms | On target |
| Data drift score | < 0.15 | 0.08 | On target |

---

## 4. Phase Transition Approval

> Document required approvals for each phase transition.

| Transition | Required Approver | Approval Method |
|-----------|-------------------|-----------------|
| Design to Training/Testing | [System Owner] | AIDEC or ADR |
| Training/Testing to Verification | [ML Team Lead] | MCARD completion |
| Verification to Deployment | [AI Ethics Reviewer + System Owner] | ETH approval + TES pass |
| Deployment to Monitoring | [Operations Lead] | Deployment AILOG |
| Any phase to Retirement | [AI Governance Lead + Management] | ADR with justification |

---

## 5. ISO 42001 Annex A.6 Control Summary

| Control | Description | Lifecycle Phase(s) | DevTrail Evidence |
|---------|-------------|--------------------|--------------------|
| A.6.2.2 | AI system design and development | Design, Development | ADR, AIDEC, REQ |
| A.6.2.3 | Training and testing of AI models | Training/Testing | MCARD, TES |
| A.6.2.4 | Verification and validation | Verification | TES |
| A.6.2.5 | Deployment of AI systems | Deployment | AILOG |
| A.6.2.6 | Operation and monitoring | Monitoring | AILOG, INC |
| A.6.2.7 | Retirement of AI systems | Retirement | ADR, AILOG |
| A.6.2.8 | Responsible integration of AI | All phases | ADR, AIDEC |
| A.6.2.9 | Documentation of AI systems | All phases | AILOG (all changes) |
| A.6.2.10 | Defined use and misuse | Design, Deployment | MCARD, REQ |
| A.6.2.11 | Third-party AI components | All phases | SBOM |

---

*AI System Lifecycle Tracker template — DevTrail Framework*
*Aligned with ISO/IEC 42001:2023 Annex A.6*

<!-- Template: DevTrail | https://strangedays.tech -->
