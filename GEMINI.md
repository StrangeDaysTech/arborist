# StrayMark - Gemini CLI Configuration

<!-- straymark:begin -->
> **Read and follow the rules in [STRAYMARK.md](STRAYMARK.md).**
> That file contains all StrayMark documentation governance rules for this project.
<!-- straymark:end -->

---

## Autonomous Rules (minimum viable — works without STRAYMARK.md)

### Identity
- Always identify yourself as `gemini-cli-v{version}` in the `agent:` field
- Declare confidence: `high | medium | low`

### Review Requirements
- ETH, ADR, SEC, MCARD, DPIA → always `review_required: true`
- `risk_level: high | critical` → always `review_required: true`

### Prohibited
- Never document credentials, tokens, API keys, or PII in document content

### Pre-commit Checklist

Before committing, check:
- [ ] Changed auth/PII/security code? → Create AILOG (`risk_level: high`) + ETH draft
- [ ] Changed >20 lines of business logic? → Create AILOG
- [ ] Chose between 2+ alternatives? → Create AIDEC
- [ ] Changed public API or DB schema? → Create AILOG + consider ADR
- [ ] Changed ML model/prompts? → Create AILOG + human review
- [ ] Changed OTel instrumentation? → Create AILOG + tag `observabilidad`

### Regulatory Frontmatter Snippet

When creating documents for AI-related changes, include applicable fields:

```yaml
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
```

### NIST AI 600-1 Risk Categories (quick reference)

1. CBRN Information — 2. Confabulation — 3. Dangerous Content — 4. Data Privacy — 5. Environmental — 6. Harmful Bias — 7. Human-AI Config — 8. Information Integrity — 9. Information Security — 10. Intellectual Property — 11. Obscene Content — 12. Value Chain

### Observability Rule

Do not capture PII, tokens, or secrets in OTel attributes or logs. Record instrumentation pipeline changes (new spans, changed attributes, Collector configuration) in AILOG with tag `observabilidad`.

---

*StrayMark | [Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*
