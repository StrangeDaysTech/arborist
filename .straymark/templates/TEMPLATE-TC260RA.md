---
id: TC260RA-YYYY-MM-DD-NNN
title: "[System] TC260 Risk Assessment"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true

# --- Approval workflow (optional, fill at review time) ---
# reviewed_by: <reviewer-id>           # email | github-handle | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
tc260_application_scenario: null  # e.g., public_service | healthcare | finance | content_generation | social | safety_critical
tc260_intelligence_level: null    # narrow | foundation | agentic | general
tc260_application_scale: null     # individual | organization | societal | cross_border
tc260_risk_level: not_applicable  # low | medium | high | very_high | extremely_severe | not_applicable
tc260_endogenous_risks: []
tc260_application_risks: []
tc260_derivative_risks: []
iso_42001_clause: [6]
tags: [china, tc260, risk-assessment]
related: []
---

# TC260RA: [System] TC260 Risk Assessment

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval.
>
> Aligned with the **AI Safety Governance Framework v2.0** published by TC260 (National Information Security Standardization Technical Committee, 全国信息安全标准化技术委员会) on 15 September 2025.

## 1. Four Pillars Mapping

Show how this system addresses each of the four pillars of the framework:

| Pillar | Coverage in this system |
|--------|-------------------------|
| **Governance principles** (people-centered, AI for good, safe & controllable) | [Statement] |
| **Risk taxonomy** (endogenous technical / application / derivative) | [Reference to taxonomy below] |
| **Technical countermeasures** | [Reference to section 4] |
| **Governance measures** (organizational implementation) | [Reference to section 5] |

## 2. Three-Criteria Risk Grading (Section 5.5 / Appendix 1)

Risk level is computed from the combination of **application scenario × intelligence level × application scale**.

### 2.1 Application Scenario

- **Selected**: [public_service / healthcare / finance / content_generation / social / safety_critical / industrial_control / other]
- **Justification**: [Why this scenario applies]

### 2.2 Level of Intelligence

| Level | Definition | This system |
|-------|-----------|------------|
| Narrow | Single-purpose model | [ ] |
| Foundation | General-purpose foundation model | [ ] |
| Agentic | Autonomous agent with tool use | [ ] |
| General | Approaching AGI | [ ] |

- **Selected**: [narrow / foundation / agentic / general]

### 2.3 Application Scale

| Scale | Definition | This system |
|-------|-----------|------------|
| Individual | Single-user or small team | [ ] |
| Organization | Single organization deployment | [ ] |
| Societal | Affects significant portion of public | [ ] |
| Cross-border | Operates across mainland China and other jurisdictions | [ ] |

- **Selected**: [individual / organization / societal / cross_border]

### 2.4 Resulting Risk Level

| Risk Level | Description |
|------------|-------------|
| Low | Minimal expected harm; standard controls suffice |
| Medium | Foreseeable, contained harm; review and basic countermeasures required |
| High | Significant risk to individuals or specific groups; comprehensive controls required |
| Very High | Risk to societal stability or large populations; sector-level oversight |
| Extremely Severe | Risk of catastrophic or systemic harm; loss of control / catastrophic risk concerns |

- **Computed Level**: [low / medium / high / very_high / extremely_severe]
- **Justification**: [Reasoning combining scenario × intelligence × scale]

## 3. Risk Taxonomy

### 3.1 Endogenous Technical Risks

> Risks arising from the AI model itself: vulnerabilities, bias, hallucination, robustness gaps.

| Risk | Description | Likelihood | Severity | Mitigation |
|------|-------------|-----------|----------|-----------|
| [Risk 1] | [Description] | [L/M/H] | [L/M/H] | [Measure] |
| [Risk 2] | [Description] | [L/M/H] | [L/M/H] | [Measure] |

### 3.2 Application Risks

> Risks arising from how the AI is technically applied: misuse, scope creep, dependency.

| Risk | Description | Likelihood | Severity | Mitigation |
|------|-------------|-----------|----------|-----------|
| [Risk 1] | [Description] | [L/M/H] | [L/M/H] | [Measure] |
| [Risk 2] | [Description] | [L/M/H] | [L/M/H] | [Measure] |

### 3.3 Derivative Risks

> Risks arising from second-order societal effects: labor displacement, opinion shaping, ecosystem disruption.

| Risk | Description | Likelihood | Severity | Mitigation |
|------|-------------|-----------|----------|-----------|
| [Risk 1] | [Description] | [L/M/H] | [L/M/H] | [Measure] |
| [Risk 2] | [Description] | [L/M/H] | [L/M/H] | [Measure] |

## 4. Technical Countermeasures

Map each prioritized risk to one or more technical controls (red-teaming, alignment, content filters, watermarking per GB 45438, model evaluation suites, sandboxing, etc.).

| Risk ID | Countermeasure | Owner | Verification |
|---------|---------------|-------|-------------|
| [E.1] | [Control] | [Role] | [Test plan / metric] |
| [A.1] | [Control] | [Role] | [Test plan / metric] |
| [D.1] | [Control] | [Role] | [Test plan / metric] |

## 5. Governance Measures

- **Designated Owner**: [Role / Person]
- **Internal Reporting Cadence**: [Monthly / quarterly]
- **Escalation Path**: [To whom and under what triggers]
- **Open-Source Components**: [If the system embeds open-source AI: governance per v2.0 OSS clauses]
- **Catastrophic-Risk Watch**: [Required for very_high / extremely_severe levels: how loss-of-control scenarios are monitored]

## 6. Monitoring & Review

- **Next Review Date**: [YYYY-MM-DD]
- **Review Triggers**: [Model version change / scenario expansion / scale jump / regulatory update]
- **Linked Documents**: [ETH-..., MCARD-..., AILABEL-..., CACFILE-...]

## Approval

| Approved by | Date | Decision | Conditions |
|-------------|------|----------|-----------|
| [Reviewer] | [YYYY-MM-DD] | [approved / conditional / rejected] | [Conditions if any] |

<!-- Template: StrayMark | https://strangedays.tech -->
