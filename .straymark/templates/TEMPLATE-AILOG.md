---
id: AILOG-YYYY-MM-DD-NNN
title: [Descriptive title of the action]
status: accepted
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high | critical
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
lines_changed: 0                # Auto-calculable
files_modified: []              # Auto-calculable
observability_scope: none        # none | basic | full — set when OTel instrumentation is relevant
tags: []
related: []
---

# AILOG: [Title]

## Summary

[Brief description of what was done and why]

## Context

[Situation that motivated this action]

## Actions Performed

1. [Action 1]
2. [Action 2]
3. [Action 3]

## Modified Files

| File | Lines Changed (+/-) | Change Description |
|------|--------------------|--------------------|
| `path/to/file.ts` | +N/-M | [Description of change] |

## Decisions Made

[If there were relevant decisions, document them briefly or reference AIDEC]

## Impact

- **Functionality**: [Description]
- **Performance**: [Description or N/A]
- **Security**: [Description or N/A]
- **Privacy**: [Description or N/A]
- **Environmental**: [Description or N/A]

## Verification

- [ ] Code compiles without errors
- [ ] Tests pass
- [ ] Manual review performed
- [ ] Security scan passed (if risk_level: high/critical)
- [ ] Privacy review completed (if handling PII)

## EU AI Act Considerations

> Complete this section only if `eu_ai_act_risk` is `high` or `limited`.

- **Risk Classification**: [high | limited]
- **Annex III Category**: [If applicable — specify category]
- **Conformity Assessment Required**: [Yes/No]
- **Transparency Obligations**: [Description of applicable obligations]

## NIST GenAI Risk Assessment

> Complete this section when the change involves generative AI components.
> Reference: NIST AI 600-1 (Generative AI Profile).

| # | Category | Applicable | Description | Mitigation |
|---|----------|-----------|-------------|------------|
| 1 | CBRN Information | [Yes/No] | | |
| 2 | Confabulation | [Yes/No] | | |
| 3 | Dangerous/Violent/Hateful Content | [Yes/No] | | |
| 4 | Data Privacy | [Yes/No] | | |
| 5 | Environmental Impacts | [Yes/No] | | |
| 6 | Harmful Bias / Homogenization | [Yes/No] | | |
| 7 | Human-AI Configuration | [Yes/No] | | |
| 8 | Information Integrity | [Yes/No] | | |
| 9 | Information Security | [Yes/No] | | |
| 10 | Intellectual Property | [Yes/No] | | |
| 11 | Obscene/Abusive Content | [Yes/No] | | |
| 12 | Value Chain / Component Integration | [Yes/No] | | |

## Additional Notes

[Any additional relevant information]

> **Observability note**: If this change modifies observability instrumentation (new spans, changed attributes, pipeline configuration), describe the observability impact and include tag `observabilidad`.

---

<!-- Template: DevTrail | https://strangedays.tech -->
