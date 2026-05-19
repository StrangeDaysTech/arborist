# StrayMark - Documentation Governance Rules

> **This file is automatically managed by StrayMark CLI.**
> Read and follow these rules when working on this project.
> For complete rules: `.straymark/00-governance/AGENT-RULES.md`

---

## Why these rules exist

StrayMark externalizes the cognitive discipline of senior software engineering — explicit scope, declared decisions, named risks, recorded alternatives, audited trails — into versioned files that live alongside the code. The intent is to constrain the agent's decision space so AI-assisted work stays coherent across many turns instead of drifting into hidden technical debt.

As a side effect of doing the engineering work this way, the artifacts produced map cleanly onto the major AI governance frameworks. The rules are written for the engineering work first; the compliance evidence is what falls out when the work is done with discipline.

**Frameworks the resulting evidence aligns with:**

- **ISO/IEC 42001:2023** (AI Management System) — vertebral standard for governance structure
- **EU AI Act** (Regulation 2024/1689) — risk classification, transparency, incident reporting
- **NIST AI RMF 1.0 + 600-1** — risk management functions and generative AI risk profiles
- **GDPR** — data protection impact assessments and privacy safeguards

**Optional regional scope** — when `.straymark/config.yml` declares `regional_scope: china`, the framework additionally produces evidence for:

- **TC260 AI Safety Governance Framework v2.0** — risk grading (TC260RA)
- **PIPL** (Personal Information Protection Law) — PIPIA, retention ≥ 3 years
- **GB 45438-2025** *(mandatory)* — AI-generated content labeling (AILABEL)
- **CAC Algorithm Filing** — algorithm registration (CACFILE)
- **GB/T 45652-2025** — pre-training & fine-tuning data security
- **CSL 2026** — cybersecurity incident reporting (1h / 4h+72h+30d windows)

> See `AI-GOVERNANCE-POLICY.md` for the ISO 42001 Annex A control mapping and `CHINA-REGULATORY-FRAMEWORK.md` for the China coverage matrix. The product-level rationale lives in [`Propuesta/straymark-design-principles.md`](https://github.com/StrangeDaysTech/straymark/blob/main/Propuesta/straymark-design-principles.md).

---

## 1. Fundamental Principle

> **"No significant change without a documented trace — and a constrained decision space for the agent."**

---

## 2. Language Configuration

Check `.straymark/config.yml` for the project's language setting:

```yaml
language: en  # Options: en, es, zh-CN (default: en)
```

**Template paths based on language:**

| Language | Template Path |
|----------|---------------|
| `en` (default) | `.straymark/templates/TEMPLATE-*.md` |
| `es` | `.straymark/templates/i18n/es/TEMPLATE-*.md` |
| `zh-CN` | `.straymark/templates/i18n/zh-CN/TEMPLATE-*.md` |

If the config file doesn't exist or `language` is not set, use English (`en`) as default.

---

## 3. Documentation Reporting

At the end of each task, you MUST report your StrayMark documentation status:

**If you created documentation:**
```
StrayMark: Created AILOG-2025-01-27-001-implement-auth.md
```

**If documentation was not needed:**
```
StrayMark: No documentation required (minor change / below complexity threshold)
```

**If you should have documented but didn't:**
```
StrayMark: Documentation pending - review required
```

This transparency helps users verify compliance with StrayMark rules.

---

## 4. Agent Identity

When working on this project:

- **Identify yourself** with your platform and version (e.g., `claude-code-v1.0`, `gemini-cli-v1.0`, `copilot-cli-v1.0`)
- **Declare** your confidence level in decisions: `high | medium | low`
- **Record** your identification in the `agent:` field of the metadata

---

## 5. Git Operations

> **CRITICAL: Never commit directly to `main` branch.**

All changes must go through feature/fix branches and Pull Requests.

### Branch Prefixes

| Prefix | Purpose |
|--------|---------|
| `feature/` or `feat/` | New features |
| `fix/` | Bug fixes |
| `hotfix/` | Urgent production fixes |
| `docs/` | Documentation only |
| `refactor/` | Code refactoring |
| `test/` | Test changes |

### Conventional Commits

| Prefix | Use Case |
|--------|----------|
| `feat:` | New feature |
| `fix:` | Bug fix |
| `docs:` | Documentation only |
| `refactor:` | No behavior change |
| `chore:` | Maintenance |

### Quick Workflow

```bash
git checkout main && git pull origin main
git checkout -b fix/descriptive-name
# ... make changes and commits ...
git push -u origin fix/descriptive-name
gh pr create --title "fix: description" --body "..."
```

> **Full details:** `.straymark/00-governance/GIT-BRANCHING-STRATEGY.md`

---

## 6. When to Document

### MANDATORY (create document)

| Situation | Action |
|-----------|--------|
| Code complexity above threshold | Create AILOG — run `straymark analyze <files> --output json`; fallback: >20 lines |
| Decision between technical alternatives | Create AIDEC |
| Changes in auth/authorization/PII | Create AILOG (`risk_level: high`) + ETH draft |
| Changes in public API or DB schema | Create AILOG + consider ADR |
| Changes in ML models or AI prompts | Create AILOG + human review |
| Security-critical dependency changes | Create AILOG + human review |
| OTel instrumentation changes | Create AILOG + tag `observabilidad` |
| Multi-session implementation block (>1 day, >5 tasks across phases) | Declare a **Charter** — `straymark charter new`. See section 15. |
| Transversal technical debt — heritage from prior Charter, applies to multiple modules, or requires dedicated Charter to remediate | Create **TDE** — distinct from per-Charter `R<N>`. See `.straymark/00-governance/AGENT-RULES.md` §3 for TDE vs `R<N>` disambiguation. |

### DO NOT DOCUMENT

- Trivial changes (whitespace, typos, formatting)
- Sensitive information (credentials, tokens, API keys)

---

## 7. File Naming Convention

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

**Example**: `AILOG-2025-01-27-001-implement-oauth.md`

---

## 8. Minimum Metadata

```yaml
---
id: AILOG-2026-03-25-001
title: Brief description
status: accepted
created: 2026-03-25
agent: your-agent-id-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
tags: [oauth, authentication, api]
related:
  - ADR-2026-01-20-001-use-jwt-tokens.md
# Optional regulatory fields (activate by context):
# eu_ai_act_risk: not_applicable
# nist_genai_risks: []
# iso_42001_clause: []
# observability_scope: none
---
```

### Tags

- Use **kebab-case** keywords: `sqlite`, `api-design`, `gnome-integration`
- 3 to 8 tags per document — describe topic, technology, or component
- Tags enable search and categorization in `straymark explore`

### Related

- Reference other **StrayMark documents** by filename (with `.md`): `AILOG-2025-01-27-001-implement-oauth.md`
- For documents in subdirectories, include path from `.straymark/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-file.md`
- For documents in the same directory, filename alone is sufficient
- **Do not** use task IDs, issue numbers, or URLs — those go in the document body

---

## 9. Autonomy Limits

| Type | Agent can do | Requires human |
|------|----------|----------------|
| **AILOG** | Create freely | — |
| **AIDEC** | Create freely | — |
| **SBOM** | Create freely | — |
| **ETH** | Create draft | Approval |
| **ADR** | Create draft | Review |
| **SEC** | Create draft | Approval (always) |
| **MCARD** | Create draft | Approval (always) |
| **DPIA** | Create draft | Approval (always) |
| **REQ** | Propose | Validation |
| **TES** | Propose | Validation |
| **INC** | Contribute analysis | Conclusions |
| **TDE** | Identify | Prioritize |
| **PIPIA** *(china)* | Create draft | Approval (always) |
| **CACFILE** *(china)* | Create draft | Approval (always — counsel + compliance officer before submission) |
| **TC260RA** *(china)* | Create draft | Approval (always) |
| **AILABEL** *(china)* | Create draft | Approval (always — before deployment) |
| **Charter** | Scaffold via `charter new`; fill scope, files, risks, tasks | Operator owns the *trigger* and lifecycle transitions (`declared` → `in-progress` → `closed`). See section 15. |

---

## 10. Documentation Map

> **IMPORTANT**: This is the complete project structure.
> Not everything is loaded in this session, but any document can be accessed when needed.

```
.straymark/
├── 00-governance/              ← POLICIES AND RULES
│   ├── PRINCIPLES.md           # Project guiding principles
│   ├── DOCUMENTATION-POLICY.md # Complete documentation policy
│   ├── AGENT-RULES.md          # Detailed rules for AI agents
│   └── exceptions/             # Documented exceptions
│
├── 01-requirements/            ← REQUIREMENTS (REQ)
│   └── [REQ-*.md]              # System requirements
│
├── 02-design/                  ← DESIGN
│   └── decisions/              # ADRs (Architecture Decision Records)
│       └── [ADR-*.md]
│
├── 03-implementation/          ← IMPLEMENTATION GUIDES
│   └── [technical guides]
│
├── 04-testing/                 ← TESTING (TES)
│   └── [TES-*.md]              # Test strategies and plans
│
├── 05-operations/              ← OPERATIONS
│   ├── [runbooks]
│   └── incidents/              # Post-mortems (INC)
│       └── [INC-*.md]
│
├── 06-evolution/               ← EVOLUTION
│   └── technical-debt/         # Technical debt (TDE)
│       └── [TDE-*.md]
│
├── 07-ai-audit/                ← AI AGENT AUDIT
│   ├── agent-logs/             # Action logs (AILOG)
│   │   └── [AILOG-*.md]
│   ├── decisions/              # Agent decisions (AIDEC)
│   │   └── [AIDEC-*.md]
│   ├── ethical-reviews/        # Ethical reviews (ETH, DPIA, PIPIA*)
│   │   └── [ETH-*.md]
│   ├── regulatory-filings/     # CAC algorithm filings (CACFILE*)
│   │   └── [CACFILE-*.md]
│   └── risk-assessments/       # TC260 risk assessments (TC260RA*)
│       └── [TC260RA-*.md]
│
├── 08-security/                ← SECURITY ASSESSMENTS (SEC)
│   └── [SEC-*.md]
│
├── 09-ai-models/               ← AI MODEL CARDS (MCARD)
│   ├── [MCARD-*.md]
│   └── labeling/               # GB 45438 content labeling plans (AILABEL*)
│       └── [AILABEL-*.md]
│
├── charters/                   ← CHARTERS — BOUNDED UNITS OF WORK
│   ├── [NN-slug.md]            # Declarative ex-ante scope (filename: NN-slug.md, NOT TYPE-YYYY-...)
│   └── [NN-slug.telemetry.yaml] # Post-close telemetry (created by `straymark charter close`)
│
├── templates/                  ← TEMPLATES (12 base + 4 China* + Charter)

* Only created when regional_scope: china is enabled in .straymark/config.yml.
│
└── QUICK-REFERENCE.md          ← 1-page quick reference
```

---

## 11. When to Load Additional Documents

| Situation | Document to load |
|-----------|------------------|
| Going to create an AILOG | `.straymark/templates/TEMPLATE-AILOG.md` |
| Going to create an AIDEC | `.straymark/templates/TEMPLATE-AIDEC.md` |
| Going to create an ADR | `.straymark/templates/TEMPLATE-ADR.md` |
| Going to create a REQ | `.straymark/templates/TEMPLATE-REQ.md` |
| Questions about naming or metadata | `.straymark/00-governance/DOCUMENTATION-POLICY.md` |
| Questions about autonomy limits | `.straymark/00-governance/AGENT-RULES.md` |
| Need to see existing requirements | List `.straymark/01-requirements/` |
| Need to see existing ADRs | List `.straymark/02-design/decisions/` |
| Need to see technical debt | List `.straymark/06-evolution/technical-debt/` |
| Going to declare a Charter | `.straymark/templates/charter/charter-template.md` (or `straymark charter new`) |
| Need to see existing Charters | `straymark charter list` (or list `.straymark/charters/`) |
| SpecKit user — when does a feature yield a Charter? | `.straymark/00-governance/SPECKIT-CHARTER-BRIDGE.md` |
| Wondering why agents surface things you didn't ask | `.straymark/00-governance/EMERGENT-OBSERVATION-DESIGN.md` |

---

## 12. Workflow

```
1. EVALUATE if the change requires documentation (see section 6)
                            |
                            v
2. LOAD the corresponding template (see section 11)
                            |
                            v
3. CREATE the document with correct naming
   [TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
                            |
                            v
4. If risk_level: high/critical or confidence: low
   -> Mark review_required: true
```

---

## 13. Quick Type Reference

| Prefix | Name | Location |
|--------|------|----------|
| `AILOG` | AI Action Log | `.straymark/07-ai-audit/agent-logs/` |
| `AIDEC` | AI Decision | `.straymark/07-ai-audit/decisions/` |
| `ETH` | Ethical Review | `.straymark/07-ai-audit/ethical-reviews/` |
| `ADR` | Architecture Decision Record | `.straymark/02-design/decisions/` |
| `REQ` | Requirement | `.straymark/01-requirements/` |
| `TES` | Test Plan | `.straymark/04-testing/` |
| `INC` | Incident Post-mortem | `.straymark/05-operations/incidents/` |
| `TDE` | Technical Debt | `.straymark/06-evolution/technical-debt/` |
| `SEC` | Security Assessment | `.straymark/08-security/` |
| `MCARD` | Model/System Card | `.straymark/09-ai-models/` |
| `SBOM` | Software Bill of Materials | `.straymark/07-ai-audit/` |
| `DPIA` | Data Protection Impact Assessment | `.straymark/07-ai-audit/ethical-reviews/` |
| `Charter` | Bounded unit of work (filename `NN-slug.md`, not `TYPE-YYYY-…`) | `.straymark/charters/` |

---

## 14. Regulatory Alignment

StrayMark is aligned with the following standards and regulations:

| Standard | Role in StrayMark | Key Documents |
|----------|-----------------|---------------|
| **ISO/IEC 42001:2023** | Vertebral standard — AI Management System | AI-GOVERNANCE-POLICY.md |
| **EU AI Act** | Risk classification, incident reporting, transparency | ETH, INC, AILOG regulatory fields |
| **NIST AI RMF / 600-1** | Risk management, 12 GenAI risk categories | ETH, AILOG |
| **ISO/IEC 25010:2023** | Software quality model (9 characteristics) | REQ, ADR |
| **ISO/IEC/IEEE 29148:2018** | Requirements engineering | REQ |
| **ISO/IEC/IEEE 29119-3:2021** | Software testing documentation | TES |
| **GDPR** | Data protection and privacy | ETH (Data Privacy) |
| **OpenTelemetry** | Observability (optional, complementary) | Tag `observabilidad` |
| **C4 Model** | Architecture visualization in ADR documents | ADR (Mermaid diagrams) |

> **Reference**: See `AI-GOVERNANCE-POLICY.md` for the full ISO 42001 Annex A mapping to StrayMark documents.

---

## 15. Charters — bounded units of work

A **Charter** is StrayMark's artifact for an *implementation block that is too big to track as a single AILOG and too small to deserve its own product spec*. It pairs **declarative ex-ante scope** (files to modify, risks, tasks, effort estimate) with **ex-post telemetry** (drift detection, external audit, lessons learned). Concretely: the kind of work that takes >1 day, spans >5 tasks, or crosses multiple sessions where an agent might lose the thread.

Charters are **conceptually distinct** from the 12+4 document types listed in section 13:

- They live at `.straymark/charters/NN-slug.md` (sequential prefix, not date-prefix).
- Their lifecycle is `declared` → `in-progress` → `closed`, persisted in frontmatter as the source of truth.
- Telemetry sits beside them as `NN-slug.telemetry.yaml`, written by `straymark charter close`.
- External audits resolve to `.straymark/audits/CHARTER-NN/` and merge back into telemetry.

### When to declare a Charter

Trigger row in section 6 names the heuristic; expanded:

- The work spans **more than one working session** and you need a stable scope contract you can drift-check at close.
- The work has **5+ named tasks** with a topological order (e.g., a SpecKit `tasks.md`).
- The work warrants **external audit** at completion (cross-model review, cross-team review).
- You want **measurable telemetry**: time-to-close, drift count, lessons that feed forward.

### Charter lifecycle

| Stage | What happens | CLI |
|-------|--------------|-----|
| **Declared** | Operator scaffolds Charter, fills scope/files/risks/tasks. Status `declared`. | `straymark charter new` |
| **In progress** | Operator flips status to `in-progress` when execution starts. Day-to-day work continues to produce AILOGs. | (manual frontmatter edit) |
| **Batch update** *(multi-batch only, fw-4.14.0+)* | For Charters spanning 3+ batches or >1 day, the AILOG carries a `## Batch Ledger` with one `### Batch N` entry per batch starting as `(pending)`. After each batch commit lands, fill the entry **before pushing** so the AILOG update and the work ride the same push. Single-batch Charters skip this step entirely. | `straymark charter batch-complete CHARTER-NN <N>` |
| **Drift check** | Before closing, verify declared files vs. actual git diff. AILOG-suppressed paths are noise-filtered; pending `### Batch N` entries cause a hard fail (`--no-batch-ledger-check` bypasses). | `straymark charter drift` |
| **External audit** *(optional)* | Generate prompt → run N auditor CLIs → consolidate → merge into telemetry. | `straymark charter audit` + `/straymark-audit-prompt` + `/straymark-audit-execute` + `/straymark-audit-review` |
| **Closed** | Telemetry yaml emitted; status flips to `closed`; charter is now an audit artifact. | `straymark charter close` |

### How Charters relate to existing artifacts

- A Charter **does not replace** AILOGs. AILOGs document individual blocks of code work; the Charter wraps them as a unit. `originating_ailogs:` in the Charter frontmatter declares which AILOGs the Charter contains.
- A Charter **does not replace** ADRs. ADRs document architectural decisions; a Charter may reference them in `originating_spec` or in body sections.
- A Charter **bridges SpecKit features** (`specs/NNN-feature/spec.md`) and execution. See `.straymark/00-governance/SPECKIT-CHARTER-BRIDGE.md` for granularity heuristics and creation timing.

### Quick CLI surface

```bash
straymark charter new                       # scaffold a Charter (interactive)
straymark charter list                      # enumerate Charters with status/effort/origin
straymark charter status CHARTER-NN         # detailed view of one Charter
straymark charter drift CHARTER-NN          # file-vs-commit drift, AILOG-aware + Batch Ledger gate
straymark charter batch-complete CHARTER-NN <N>  # fill AILOG `## Batch Ledger` Batch <N> (multi-batch only)
straymark charter audit CHARTER-NN          # multi-model external audit (orchestration only)
straymark charter close CHARTER-NN          # record telemetry; flip status to `closed`
straymark charter refresh-suggest <module>  # SpecKit refresh recommendation (chain-level, fw-4.16.0+) — see §15.A
straymark charter amend CHARTER-NN          # scaffold post-close Batch N.4 amendment (fw-4.16.0+) — see §15.B
```

> **Schema**: `.straymark/schemas/charter.schema.v0.json` (declarative) and `.straymark/schemas/charter-telemetry.schema.v0.json` (telemetry). Both are experimental v0 — patterns crystallize after validation against a second domain (Principle #12).

### §15.A — Pre-declare SpecKit refresh *(fw-4.16.0+)*

Multi-Charter modules accumulate spec drift over time. After 3+ Charters in a chain, SpecKit artifacts (`plan.md`, `data-model.md`, `contracts/*`, `quickstart.md`, `research.md`) authored at chain start tend to lag behind the implementation. The fix is a dedicated **refresh PR** between Charter-N close and Charter-(N+1) declare that integrates accumulated learnings into `research.md` (categorized buckets: reusable patterns, code gaps, discipline patterns, empirical corrections) and ratifies operator decisions. The next Charter's `## Context` cites each learning by id.

**Trigger heuristic** — adopt when (a) chain length ≥ 3 closed Charters in the module AND (b) rolling mean of `agent_quality.r_n_plus_one_emergent_count` across last 3 > 6 AND (c) no refresh PR landed since the chain's branch point. Run:

```bash
straymark charter refresh-suggest <module>  # read-only — prints recommendation
```

**Telemetry** — `charter_telemetry.pre_declare_refresh:` (optional, opt-in). See `.straymark/00-governance/CHARTER-CHAIN-EVOLUTION.md` Pattern 1 for full mechanics.

### §15.B — Post-close audit-driven amendment (Batch N.4) *(fw-4.16.0+)*

External audit cycles run post-close. When Critical or High findings arrive after a Charter has been marked `status: closed`, the bounded remediation pattern is a **Batch N.4 amendment** that rides on the same execute branch: a new AILOG documents the amendment, the original AILOG receives a `## Historical correction` subsection pointing forward, and the telemetry gains a `post_close_amendment:` block. Apply only when (a) findings are Critical/High, (b) the Charter's closure criterion is materially unmet, AND (c) the fix surface fits in one cohesive PR (~< 25 files, no architectural reopen). Larger or architectural fixes warrant a new Charter.

```bash
straymark charter amend CHARTER-NN \         # scaffolds the new AILOG, edits the original, prints YAML
  --trigger external_audit \
  --findings-closed 5 \
  --ailog-title "post-close remediation — DI wiring + retry"
```

The command does not touch git — the operator decides when to commit. The `straymark charter audit <id> --merge-reports --merge-into <telemetry-yaml>` path tolerates `external_audit: []` placeholders in v0.2+ schema so the round-trip with `post_close_amendment` is smooth. See `.straymark/00-governance/CHARTER-CHAIN-EVOLUTION.md` Pattern 2 for full mechanics.

---

## Directive Injection Markers

StrayMark uses HTML comment markers to manage injected content in agent configuration files (`AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md`, `.cursorrules`, `.cursor/rules/straymark.md`):

```html
<!-- straymark:begin -->
... managed content ...
<!-- straymark:end -->
```

- Content between these markers is managed by `straymark init`, `update`, and `repair`
- Do not remove or modify these markers manually — they are required for safe updates
- If markers are missing from a target file, StrayMark appends the content block at the end
- **Missing-target behavior** *(refined in fw-4.16.2 / cli-3.14.1)*: `straymark init`, `straymark update-framework`, and `straymark repair` all walk `dist-manifest.yml::injections:` and **create any missing target file** (with the marker block) per individual target — a single deleted directive file like `AGENTS.md` is restored without needing to re-run init or remove anything else. There is no opt-out per target short of editing the manifest. *(Historical note: through `cli-3.14.0` / `fw-4.16.1`, `straymark repair` was gated on `STRAYMARK.md` being absent and would silently skip a per-target restore; that gate was removed in `cli-3.14.1`.)*
- `AGENTS.md` is the open standard donated to the Agentic AI Foundation (Linux Foundation, 2025) and is read by Claude Code, OpenAI Codex CLI, Cursor, Aider, Devin, Sourcegraph Amp, Google Jules, Zed AI, Continue, Roo Code, Factory Droids, GitHub Copilot, Gemini CLI, Windsurf, Amazon Q and others. CLI-specific files (`CLAUDE.md`, `GEMINI.md`, etc.) coexist with `AGENTS.md` and provide platform-specific identity strings.

---

*StrayMark | [GitHub](https://github.com/StrangeDaysTech/straymark)*
*[Strange Days Tech](https://strangedays.tech) — Because every change tells a story.*
