# StrayMark - Quick Reference

> One-page reference for AI agents and developers.
>
> **This is a derived document** — DOCUMENTATION-POLICY.md is the authoritative source.

**Languages**: English | [Español](i18n/es/QUICK-REFERENCE.md) | [简体中文](i18n/zh-CN/QUICK-REFERENCE.md)

---

## Language Configuration

**File**: `.straymark/config.yml`

```yaml
language: en  # Options: en, es (default: en)
```

| Language | Templates Path |
|----------|---------------|
| `en` | `.straymark/templates/TEMPLATE-*.md` |
| `es` | `.straymark/templates/i18n/es/TEMPLATE-*.md` |

---

## Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

**Example**: `AILOG-2026-03-25-001-implement-oauth.md`

---

## Document Types (16)

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

### China Regulatory Types (4) — opt-in via `regional_scope: china`

| Type | Name | Folder | Agent Autonomy |
|------|------|--------|---------------|
| `PIPIA` | Personal Information Protection Impact Assessment (PIPL Art. 55-56) | `07-ai-audit/ethical-reviews/` | Draft → approval (always) |
| `CACFILE` | CAC Algorithm Filing | `07-ai-audit/regulatory-filings/` | Draft → approval (always) |
| `TC260RA` | TC260 v2.0 Risk Assessment | `07-ai-audit/risk-assessments/` | Draft → approval (always) |
| `AILABEL` | GB 45438 Content Labeling Plan | `09-ai-models/labeling/` | Draft → approval (always) |

### Bounded Units of Work — Charter

Charters are **not** doc types — they wrap a multi-session implementation block. Filename uses a sequential prefix (`NN-slug.md`), not a date prefix. Lifecycle: `declared` → `in-progress` → `closed`.

| Concept | Folder | Agent Autonomy |
|---------|--------|---------------|
| `Charter` | `.straymark/charters/` (declarative `NN-slug.md` + telemetry `NN-slug.telemetry.yaml`) | Scaffold via `charter new`; operator owns trigger and lifecycle transitions |

> See section 15 of `STRAYMARK.md` and `.straymark/00-governance/SPECKIT-CHARTER-BRIDGE.md` for granularity heuristics, lifecycle, and the SpecKit ↔ Charter bridge.

---

## When to Document

| Situation | Action |
|-----------|--------|
| Complex code (`straymark analyze`; fallback: >20 lines) | AILOG |
| Decision between alternatives | AIDEC |
| Auth/authorization/PII changes | AILOG + `risk_level: high` + ETH |
| Public API or DB schema changes | AILOG + consider ADR |
| ML model/prompt changes | AILOG + human review |
| Security-critical dependency changes | AILOG + human review |
| OTel instrumentation changes | AILOG + tag `observabilidad` |
| Multi-session implementation block (>1 day, >5 tasks across phases) | Declare a **Charter** (`straymark charter new`) |
| Transversal technical debt (heritage from prior Charter, applies to multiple modules, requires dedicated Charter, needs human prioritization) | **TDE** — distinct from per-Charter `R<N>`; see AGENT-RULES.md §3 |

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
.straymark/
├── 00-governance/               ← Policies, AI-GOVERNANCE-POLICY.md, CHINA-REGULATORY-FRAMEWORK.md*
├── 01-requirements/             ← REQ
├── 02-design/decisions/         ← ADR
├── 03-implementation/           ← Guides
├── 04-testing/                  ← TES
├── 05-operations/incidents/     ← INC
├── 06-evolution/technical-debt/ ← TDE
├── 07-ai-audit/
│   ├── agent-logs/              ← AILOG
│   ├── decisions/               ← AIDEC
│   ├── ethical-reviews/         ← ETH, DPIA, PIPIA*
│   ├── regulatory-filings/      ← CACFILE*
│   └── risk-assessments/        ← TC260RA*
├── 08-security/                 ← SEC
├── 09-ai-models/                ← MCARD
│   └── labeling/                ← AILABEL*
├── charters/                    ← Charter (NN-slug.md + NN-slug.telemetry.yaml)
└── templates/                   ← Templates (incl. charter/ subdir)

* Only created when regional_scope: china is enabled.
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

### China — opt-in via `regional_scope: china`

| Standard | Key Documents |
|----------|--------------|
| TC260 AI Safety Governance Framework v2.0 | TC260RA — see TC260-IMPLEMENTATION-GUIDE |
| PIPL / PIPIA (Art. 55-56) | PIPIA — see PIPL-PIPIA-GUIDE |
| GB 45438-2025 (AI content labeling, mandatory) | AILABEL — see GB-45438-LABELING-GUIDE |
| CAC Algorithm Filing | CACFILE — see CAC-FILING-GUIDE |
| GB/T 45652-2025 (training-data security) | SBOM, MCARD |
| CSL 2026 (incident reporting amendments) | INC (CSL fields) |

---

## Skills (Claude Code)

| Command | Purpose |
|---------|---------|
| `/straymark-status` | Check documentation status and compliance |
| `/straymark-new` | Create any document type (interactive) |
| `/straymark-ailog` / `/straymark-aidec` / `/straymark-adr` | Quick shortcuts for AILOG / AIDEC / ADR |
| `/straymark-mcard` / `/straymark-sec` | Interactive flows for Model Card / SEC assessment |
| `/straymark-charter-new` | Scaffold a Charter (declarative ex-ante work unit) |
| `/straymark-audit-prompt CHARTER-XX` *(fw-4.9.0+, refactored in fw-4.9.0)* | External multi-model audit — write unified prompt at canonical path |
| `/straymark-audit-execute [CHARTER-XX]` *(fw-4.9.0+)* | Run inside an auditor CLI — read prompt, audit with tool use, write report |
| `/straymark-audit-review CHARTER-XX` *(fw-4.9.0+, expanded in fw-4.9.0)* | Consolidate N reports into review.md (6 sections) + merge YAML into telemetry |

---

## Patterns

| Pattern | Document |
|---------|----------|
| Follow-ups backlog (central registry + drift detection) *(fw-4.10.0+)* | [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) |

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
