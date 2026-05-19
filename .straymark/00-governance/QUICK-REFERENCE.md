# DevTrail - Quick Reference

> One-page reference for AI agents and developers.
>
> **This is a derived document** — DOCUMENTATION-POLICY.md is the authoritative source.

**Languages**: English | [Español](i18n/es/QUICK-REFERENCE.md)

---

## Language Configuration

**File**: `.devtrail/config.yml`

```yaml
language: en  # Options: en, es (default: en)
```

| Language | Templates Path |
|----------|---------------|
| `en` | `.devtrail/templates/TEMPLATE-*.md` |
| `es` | `.devtrail/templates/i18n/es/TEMPLATE-*.md` |

---

## Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

**Example**: `AILOG-2026-03-25-001-implement-oauth.md`

---

## Document Types (12)

### Core Types (8)

| Type | Name | Folder | Agent Autonomy |
|------|------|--------|---------------|
| `AILOG` | AI Action Log | `07-ai-audit/agent-logs/` | Create freely |
| `AIDEC` | AI Decision | `07-ai-audit/decisions/` | Create freely |
| `ETH` | Ethical Review | `07-ai-audit/ethical-reviews/` | Draft only |
| `ADR` | Architecture Decision | `02-design/decisions/` | Requires review |
| `REQ` | Requirement | `01-requirements/` | Propose |
| `TES` | Test Plan | `04-testing/` | Propose |
| `INC` | Incident Post-mortem | `05-operations/incidents/` | Contribute |
| `TDE` | Technical Debt | `06-evolution/technical-debt/` | Identify |

### Extended Types (4)

| Type | Name | Folder | Agent Autonomy |
|------|------|--------|---------------|
| `SEC` | Security Assessment | `08-security/` | Draft → approval (always) |
| `MCARD` | Model/System Card | `09-ai-models/` | Draft → approval (always) |
| `SBOM` | Software Bill of Materials | `07-ai-audit/` | Create freely |
| `DPIA` | Data Protection Impact Assessment | `07-ai-audit/ethical-reviews/` | Draft → approval (always) |

---

## When to Document

| Situation | Action |
|-----------|--------|
| Complex code (`devtrail analyze`; fallback: >20 lines) | AILOG |
| Decision between alternatives | AIDEC |
| Auth/authorization/PII changes | AILOG + `risk_level: high` + ETH |
| Public API or DB schema changes | AILOG + consider ADR |
| ML model/prompt changes | AILOG + human review |
| Security-critical dependency changes | AILOG + human review |
| OTel instrumentation changes | AILOG + tag `observabilidad` |

**DO NOT document**: credentials, tokens, PII, secrets.

---

## Minimum Metadata

```yaml
---
id: AILOG-2026-03-25-001
title: Brief description
status: accepted
created: 2026-03-25
agent: agent-name-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
# Optional regulatory fields (activate by context):
# eu_ai_act_risk: not_applicable
# nist_genai_risks: []
# iso_42001_clause: []
# observability_scope: none
---
```

---

## Human Review Required

Mark `review_required: true` when:

- `confidence: low`
- `risk_level: high | critical`
- Security decisions
- Irreversible changes
- ML model or prompt changes
- Security-critical dependency changes
- Documents: ETH, ADR, REQ, SEC, MCARD, DPIA

---

## Folder Structure

```
.devtrail/
├── 00-governance/               ← Policies, AI-GOVERNANCE-POLICY.md
├── 01-requirements/             ← REQ
├── 02-design/decisions/         ← ADR
├── 03-implementation/           ← Guides
├── 04-testing/                  ← TES
├── 05-operations/incidents/     ← INC
├── 06-evolution/technical-debt/ ← TDE
├── 07-ai-audit/
│   ├── agent-logs/              ← AILOG
│   ├── decisions/               ← AIDEC
│   └── ethical-reviews/         ← ETH, DPIA
├── 08-security/                 ← SEC
├── 09-ai-models/                ← MCARD
└── templates/                   ← Templates
```

---

## Workflow

```
1. EVALUATE → Does this require documentation?
       ↓
2. LOAD    → Corresponding template
       ↓
3. CREATE  → With correct naming convention
       ↓
4. MARK    → review_required if applicable
```

---

## Levels

### Confidence
| Level | Action |
|-------|--------|
| `high` | Proceed |
| `medium` | Document alternatives |
| `low` | `review_required: true` |

### Risk
| Level | Examples |
|-------|----------|
| `low` | Docs, formatting |
| `medium` | New functionality |
| `high` | Security, APIs |
| `critical` | Production, irreversible |

---

## Regulatory Alignment

| Standard | Key Documents |
|----------|--------------|
| ISO/IEC 42001:2023 | AI-GOVERNANCE-POLICY.md (vertebral) |
| EU AI Act | ETH (risk classification), INC (incident reporting) |
| NIST AI RMF / 600-1 | ETH (12 GenAI risk categories), AILOG |
| GDPR | ETH (Data Privacy), DPIA |
| ISO/IEC 25010:2023 | REQ (quality), ADR (quality impact) |
| OpenTelemetry | Optional — see OBSERVABILITY-GUIDE |
| C4 Model | ADR diagrams — see C4-DIAGRAM-GUIDE |

---

## Skills (Claude Code)

| Command | Purpose |
|---------|---------|
| `/devtrail-status` | Check documentation status and compliance |

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
