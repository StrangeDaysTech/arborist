---
id: CACFILE-YYYY-MM-DD-NNN
title: "[Service Name] CAC Algorithm Filing"
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
cac_filing_required: true
cac_filing_type: algorithm  # algorithm | generative_ai | dual
cac_filing_status: pending  # pending | provincial_submitted | provincial_approved | national_submitted | national_approved | rejected | not_required
cac_filing_number: null  # populated upon successful filing — format: e.g., "网信算备XXXXXXXXXXXX"
cac_provincial_authority: null  # provincial CAC office handling the filing
cac_national_decision_date: null  # YYYY-MM-DD when national CAC published the decision
iso_42001_clause: [8]
tags: [china, cac, algorithm-filing]
related: []
---

# CACFILE: [Service Name] CAC Algorithm Filing

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review by counsel and the responsible compliance officer before any submission to the Cyberspace Administration of China.
>
> CAC algorithm filing is required for any externally facing service with **public-opinion attributes** or **social-mobilization capabilities** under the *Provisions on the Administration of Algorithmic Recommendations of Internet Information Services* and the *Interim Measures for the Management of Generative AI Services*. Generative AI large models additionally require a **dual filing**: provincial security assessment + national algorithm filing.

## Service Overview

- **Service Name**: [Customer-facing name]
- **Provider Legal Entity**: [Registered company / branch in mainland China]
- **Provider Form**: [internet information service / app / mini-program / API / other]
- **Application Domain**: [content recommendation / search ranking / personalization / generative AI / synthesis / other]
- **Algorithm Type**: [generation_synthesis / personalized_push / sequence_scheduling / search_filter / dispatch_decision]
- **Target Audience**: [General public / specific industry / minors / other]
- **Applicable Scenarios**: [Concrete use cases]

## Public-Opinion / Social-Mobilization Attribute Assessment

Per CAC guidance, services with these attributes must file. Document the assessment:

- [ ] Generates / synthesizes / disseminates content viewable by the public
- [ ] Influences user opinion through ranking, recommendation, or content selection
- [ ] Capable of mobilizing users for collective action
- [ ] Operates in news, social media, audio/video, or live streaming domain

**Conclusion**: [Required / Not required] — [Justification]

## Training Data Description

- **Data Sources**: [Public corpora / proprietary / licensed third-party / user-generated]
- **Data Volume**: [Approximate token / image / sample count]
- **Geographic Origin**: [Domestic / overseas / mixed — note PIPL cross-border implications]
- **Lawfulness of Sources**: [Demonstrate per Art. 7 of the Interim Measures for GenAI Services: lawfulness, accuracy, objectivity, diversity]
- **Personal Information in Training Data**: [Yes/No — if yes, link to PIPIA-XXXX]
- **Sensitive Categories Filtered**: [Methods used to remove sensitive data per PIPL Art. 28]

## Blocked Keywords Strategy

> Per the Interim Measures, providers must prevent generation of content that subverts state power, incites separatism, undermines national unity, promotes terrorism / extremism / ethnic hatred / discrimination, violence, or obscene content.

- **Keyword List Source**: [Reference: internal list, version, last update]
- **Update Cadence**: [How often the list is reviewed]
- **Layered Controls**: [Pre-prompt filtering / post-generation filtering / response refusal / safe completion]
- **Audit Sample**: [Path to representative sample retained for filing]

## Testing Question Set

- **Test Set Size**: [Number of red-team prompts]
- **Coverage**: [Categories tested: political sensitivity, violence, self-harm, hate, discrimination, privacy, minors, etc.]
- **Pass Threshold**: [Required pass rate]
- **Latest Test Result**: [Date / Pass rate / Document reference]

## Internal Algorithm Management Policy

- **Designated Responsible Person**: [Name / role with full Chinese-language credentials]
- **Internal Review Workflow**: [Pre-deployment review steps]
- **Complaint Handling**: [User complaint channel + SLA]
- **Logging & Traceability**: [How model inputs/outputs are logged for accountability]

## Algorithm Security Responsibility Report

Per Art. 24 of the Internet Information Service Algorithm Recommendation Provisions, prepare a *Report on Implementation of Algorithm Security Responsibilities*. Outline:

- **Algorithm Mechanism Disclosure**: [Plain-language description of how the algorithm operates]
- **User Notification**: [How users are informed of algorithmic decision-making]
- **Opt-Out Mechanism**: [How users can disable personalization]
- **Manual Intervention Capability**: [How operators override algorithmic outputs]

## Self-Assessment

| Risk Area | Finding | Severity | Mitigation |
|-----------|---------|----------|-----------|
| Content compliance (illegal content generation) | [Finding] | [L/M/H] | [Measure] |
| Personal information protection | [Finding] | [L/M/H] | [Measure] |
| Bias / unfair differentiated treatment | [Finding] | [L/M/H] | [Measure] |
| Intellectual property | [Finding] | [L/M/H] | [Measure] |
| Minor protection | [Finding] | [L/M/H] | [Measure] |

## Filing Trail

| Stage | Date | Outcome / Reference |
|-------|------|---------------------|
| Internal sign-off | [YYYY-MM-DD] | [Document reference] |
| Provincial submission | [YYYY-MM-DD] | [Receipt number] |
| Provincial decision | [YYYY-MM-DD] | [Approved / Rejected — reasons] |
| National submission | [YYYY-MM-DD] | [Receipt number] |
| National decision | [YYYY-MM-DD] | [Filing number issued] |
| Public disclosure | [YYYY-MM-DD] | [URL where the filing number is shown to end-users] |

## Approval

| Approved by | Date | Decision | Conditions |
|-------------|------|----------|-----------|
| [Compliance officer] | [YYYY-MM-DD] | [approved / conditional / rejected] | [Conditions if any] |

<!-- Template: StrayMark | https://strangedays.tech -->
