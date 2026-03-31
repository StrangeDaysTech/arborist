# DevTrail - Claude Code Configuration

<!-- devtrail:begin -->
> **Read and follow the rules in [DEVTRAIL.md](DEVTRAIL.md).**
> That file contains all DevTrail documentation governance rules for this project.
<!-- devtrail:end -->

---

## Autonomous Rules (minimum viable — works without DEVTRAIL.md)

### Identity
- Always identify yourself as `claude-code-v{version}` in the `agent:` field
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

*DevTrail | [Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*

## Active Technologies
- Rust (edition 2024, MSRV to be determined by tree-sitter 0.25 requirements) + tree-sitter 0.25, serde (serialization), 12 tree-sitter grammar crates (10 Tier 1 + Kotlin, Swift) (001-code-metrics-library)
- N/A (pure computation library, reads files via `std::fs`) (001-code-metrics-library)

## Recent Changes
- 001-code-metrics-library: Added Kotlin and Swift (Tier 2) via tree-sitter-kotlin-ng 1.1 and tree-sitter-swift 0.7, expanding to 12 languages
