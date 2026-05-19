---
id: INC-YYYY-MM-DD-NNN
title: [Incident title]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true
risk_level: high | critical
severity: SEV1 | SEV2 | SEV3 | SEV4
eu_ai_act_applicable: false
incident_report_deadline: null  # YYYY-MM-DD — regulatory deadline if applicable
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
observability_scope: none        # none | basic | full — set when OTel instrumentation is relevant
tags: []
related: []
incident_date: YYYY-MM-DD
resolved_date: null
---

# INC: [Incident Title]

> **PARTIAL ANALYSIS**: This document contains analysis from an AI agent.
> Final conclusions and corrective actions require human review.

## Incident Summary

| Field | Value |
|-------|-------|
| Severity | [SEV1/SEV2/SEV3/SEV4] |
| Start date/time | [YYYY-MM-DD HH:MM UTC] |
| Resolution date/time | [YYYY-MM-DD HH:MM UTC] |
| Duration | [X hours Y minutes] |
| Affected services | [List of services] |
| Affected users | [Estimate] |
| Business impact | [Description] |

## Severity Definitions

| Severity | Definition |
|----------|------------|
| SEV1 | Total service outage, critical business impact |
| SEV2 | Severe degradation, main functionality affected |
| SEV3 | Partial degradation, workarounds available |
| SEV4 | Minor impact, few users affected |

## Timeline

> If your system uses OpenTelemetry, include trace-id for correlated evidence.

| Time (UTC) | Event | Trace ID | Span ID | Dashboard Link |
|------------|-------|----------|---------|----------------|
| HH:MM | [First symptom detected] | [trace-id if available] | [span-id] | [link] |
| HH:MM | [Alert triggered] | | | |
| HH:MM | [Team notified] | | | |
| HH:MM | [Initial diagnosis] | | | |
| HH:MM | [Mitigation applied] | | | |
| HH:MM | [Service restored] | | | |
| HH:MM | [Incident closed] | | | |

## Root Cause Analysis

### Immediate Cause
[What directly failed]

### Contributing Causes
1. [Contributing factor 1]
2. [Contributing factor 2]

### Root Cause (Agent Analysis)
[Agent's analysis of the fundamental cause]

> **Note**: This analysis requires validation from the technical team.

## Impact

### Technical
- [Technical impact 1]
- [Technical impact 2]

### Business
- [Business impact 1]
- [Business impact 2]

### Users
- [User impact 1]
- [User impact 2]

## Mitigation Actions Taken

1. [Action taken to resolve the incident]
2. [Action taken to resolve the incident]

## Proposed Corrective Actions

> These proposals require human review and prioritization.

| # | Action | Type | Priority | Owner | Deadline |
|---|--------|------|----------|-------|----------|
| 1 | [Action] | Prevention | [High/Medium/Low] | [TBD] | [TBD] |
| 2 | [Action] | Detection | [High/Medium/Low] | [TBD] | [TBD] |
| 3 | [Action] | Response | [High/Medium/Low] | [TBD] | [TBD] |

## Lessons Learned

### What worked well
- [Positive aspect 1]
- [Positive aspect 2]

### What didn't work
- [Aspect to improve 1]
- [Aspect to improve 2]

### Where we got lucky
- [Aspect that could have been worse]

## EU AI Act Incident Reporting

> For high-risk AI systems under EU AI Act, incidents must be reported to the market surveillance authority within:
> - **15 days** (standard incidents)
> - **10 days** (incidents resulting in death)
> - **2 days** (widespread or very serious incidents)
>
> Reference: Article 73, EU AI Act.
>
> Complete this section only if `eu_ai_act_applicable` is `true`.

| Field | Value |
|-------|-------|
| Report Deadline | [YYYY-MM-DD] |
| Authority Notified | [Yes/No/NA] |
| Report Reference | [Reference number if submitted] |

## Open Questions

1. [Question that requires additional investigation]
2. [Question for the team]

---

## Post-Mortem Review

| Field | Value |
|-------|-------|
| Reviewed by | [Name] |
| Review date | [YYYY-MM-DD] |
| Status | [Draft/Reviewed/Closed] |

<!-- Template: DevTrail | https://strangedays.tech -->
