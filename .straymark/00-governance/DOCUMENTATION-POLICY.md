# Documentation Policy - StrayMark

**Languages**: English | [Español](i18n/es/DOCUMENTATION-POLICY.md) | [简体中文](i18n/zh-CN/DOCUMENTATION-POLICY.md)

## Why this policy exists

StrayMark externalizes the cognitive discipline of senior software engineering — explicit scope, declared decisions, named risks, recorded alternatives, audited trails — into versioned files alongside the code. This policy defines the document types, metadata, and governance rules that make that discipline auditable.

As a side effect of producing those artifacts, the project accumulates evidence that maps cleanly onto the major AI governance frameworks:

- **ISO/IEC 42001:2023** — vertebral standard for AI Management Systems
- **EU AI Act** (effective August 2026) — risk classification, transparency, incident reporting
- **NIST AI RMF 1.0 + AI 600-1** — AI risk management functions and generative AI profiles
- **ISO/IEC 23894:2023** — AI risk management framework
- **GDPR** — data protection and privacy impact assessments

The policy is written for the engineering work first; compliance is what falls out when the work is documented with discipline. See Section 8 for the complete standards reference and the upstream repo's `Propuesta/straymark-design-principles.md` for the product-level rationale.

---

## 1. File Naming Convention

### Standard Format

```
[TYPE]-[YYYY-MM-DD]-[NNN]-[description].md
```

| Component | Description | Example |
|-----------|-------------|---------|
| `TYPE` | Document type prefix | `AILOG`, `AIDEC`, `ADR` |
| `YYYY-MM-DD` | Creation date | `2025-01-27` |
| `NNN` | Sequential number for the day | `001`, `002` |
| `description` | Brief description in kebab-case | `implement-oauth` |

### Examples

```
AILOG-2025-01-27-001-implement-oauth.md
AIDEC-2025-01-27-001-testing-framework-selection.md
ADR-2025-01-27-001-microservices-architecture.md
REQ-2025-01-27-001-user-authentication.md
```

---

## 2. Required Metadata (Frontmatter)

All documents must include YAML metadata at the beginning:

```yaml
---
id: AILOG-2025-01-27-001
title: OAuth Authentication Implementation
status: draft | accepted | deprecated | superseded
created: 2025-01-27
updated: 2025-01-27
agent: claude-code-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
tags:
  - auth
  - security
related:
  - ADR-2025-01-20-001
  - REQ-2025-01-15-003
---
```

### Required Fields

| Field | Description |
|-------|-------------|
| `id` | Unique identifier (same as filename without .md) |
| `title` | Descriptive title |
| `status` | Current document status |
| `created` | Creation date |
| `agent` | Identifier of the agent that created the document |
| `confidence` | Agent's confidence level |
| `review_required` | Whether human review is required |
| `risk_level` | Change risk level |

### Optional Fields

| Field | Description | When to Use |
|-------|-------------|-------------|
| `updated` | Last update date | On any update |
| `tags` | Tags for categorization (see conventions below) | Always recommended |
| `related` | References to related documents (see conventions below) | When cross-references exist |
| `supersedes` | ID of the document this one replaces | When replacing a document |
| `superseded_by` | ID of the document that replaces this one | Set by the replacing document |
| `eu_ai_act_risk` | EU AI Act risk classification: `unacceptable \| high \| limited \| minimal \| not_applicable` | When the change involves AI systems under EU AI Act |
| `nist_genai_risks` | NIST AI 600-1 risk categories: `[privacy, bias, confabulation, ...]` | When the change involves generative AI components |
| `iso_42001_clause` | ISO 42001 clauses: `[4, 5, 6, 7, 8, 9, 10]` | When mapping to ISO 42001 controls |
| `lines_changed` | Lines changed count (auto-calculable) | In AILOG documents |
| `files_modified` | List of modified files (auto-calculable) | In AILOG documents |
| `observability_scope` | OTel instrumentation level: `none \| basic \| full` | When the change involves observability instrumentation |
| `api_spec_path` | Path to OpenAPI/AsyncAPI specification file | In REQ documents when the requirement involves API interfaces |
| `api_changes` | List of API endpoints affected | In ADR documents when the decision modifies public APIs |
| `reviewed_by` | Identity of the human reviewer (email, GitHub handle, or DID) | Set by the reviewer when formally approving a `review_required: true` document |
| `reviewed_at` | Date of the formal approval (`YYYY-MM-DD`, must be ≥ `created`) | Set with `reviewed_by` |
| `review_outcome` | Closure signal: `approved \| revisions_requested \| rejected` | Set with `reviewed_by`. Presence is the canonical "human has reviewed" signal — see §4.5 below |

### Tags Convention

Tags are **free-form keywords** used for categorization and search. They help discover related documents across the project.

**Format rules:**
- Use **kebab-case** (lowercase, hyphens): `gnome-integration`, `sqlite`, `api-design`
- One concept per tag — avoid compound tags like `auth-and-security`
- Recommended range: **3 to 8 tags** per document
- Tags should describe the **topic**, **technology**, **component**, or **concern** of the document

**Example:**
```yaml
tags: [sqlite, persistence, hexagonal-architecture, repository-pattern]
```

### Related Convention

Related references link documents to other **StrayMark documents** within the same project. They enable cross-navigation in tools like `straymark explore`.

**Format rules:**
- Use the **document filename** (with `.md` extension): `AILOG-2026-02-03-001-implement-sync-item.md`
- For governance or non-typed documents, use the filename as-is: `AGENT-RULES.md`, `DOCUMENTATION-POLICY.md`
- Paths are resolved relative to `.straymark/` — if the document is in a subdirectory, include the path from `.straymark/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md`
- When the file is in the same directory as the referencing document, the filename alone is sufficient
- **Do not use** external task IDs (`T001`, `US3`), issue numbers, or URLs — those belong in the document body, not in frontmatter
- **Do not use** partial IDs without description (prefer `AILOG-2026-02-03-001-implement-sync-item.md` over `AILOG-2026-02-03-001`)

**Examples:**
```yaml
# Same directory or well-known location — filename is enough
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# Documents in specific subdirectories — include path from .straymark/
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md
  - 02-design/decisions/ADR-2026-01-15-001-use-hexagonal-architecture.md
```

**Resolution:** The CLI resolves references by searching: (1) exact ID match, (2) filename match anywhere in `.straymark/`, (3) path suffix match. Using the full filename provides the most reliable resolution.

---

## 3. Document Statuses

```
identified ──┐
             ├──► draft ──────► accepted ──────► deprecated
             │                       │                   │
             │                       │                   ▼
             │                       └──────► superseded
             │
             └──► (TDE-only entry state, see §6)
                                      │
                                      ▼
                                  resolved
                                  (TDE-only terminal — debt paid; see §6)
```

| Status | Description |
|--------|-------------|
| `identified` | Entry state for agent-driven discovery types (TDE today). Functionally equivalent to `draft` for lifecycle gating — a human reviewer is expected to prioritize and promote it. Semantically distinct so adopter analytics can distinguish "agent found this debt" from "human is drafting a deliberate doc". |
| `draft` | In draft, pending review |
| `accepted` | Approved and current |
| `resolved` | **TDE-only terminal state**: the technical debt described in this document has been addressed; the file is kept on disk as audit history. Distinct from `accepted` ("we accept this debt continues to exist"), `superseded` ("another TDE replaced this one"), and `deprecated` ("the TDE concept itself is no longer relevant"). The canonical closing reference (the Charter, PR, or commit that paid the debt) goes in the `## Resolution` body section. |
| `deprecated` | Obsolete, but kept as reference |
| `superseded` | Replaced by another document |

The per-type default status mapping lives in §6 — most types enter at `draft` or `accepted`, but TDE enters at `identified` per the agent-autonomy boundary (agent identifies, human prioritizes). TDE is the only type today with a custom terminal state (`resolved`); the validator accepts `resolved` globally as a stop-gap. A future per-doc-type lifecycle vocabulary (issue #149 Option B) will scope `resolved` to TDE strictly; until then, using it on non-TDE documents is allowed by the validator but semantically incorrect.

---

## 3.5 Recording Approval

`status` records the document's lifecycle state, and `review_required: true` records that *human review is needed*. Neither field records that human review *has actually happened*. This section defines the canonical closure signal for documents that need formal approval (AIDEC, ETH, MCARD, ADR, DPIA, INC, SEC and the China-scope variants — see AGENT-RULES.md §4 for the triggers).

### Closure signal

Three optional frontmatter fields, set by the reviewer at approval time:

```yaml
reviewed_by: pepe@example.com           # email | github-handle | DID
reviewed_at: 2026-05-02
review_outcome: approved                # approved | revisions_requested | rejected
```

Semantics:

- **The presence of `review_outcome` is the closure signal.** A document with `review_required: true` and no `review_outcome` is *pending review*.
- `review_required: true` is **not** toggled to `false` after approval — it remains as historical record of why review was needed in the first place.
- `reviewed_at` must be `>= created`. If `reviewed_by` is set, `reviewed_at` and `review_outcome` must also be set (validated by `straymark validate`).
- `review_outcome: revisions_requested` allows iterative review cycles: the document is updated, and the reviewer eventually re-approves. The convention is to overwrite the three fields with the latest approval (frontmatter holds only the most recent state); the body section below preserves history.

### Body section (canonical prose form)

Add at the terminal position of the document body (e.g., before `## References` in AIDEC/ADR; after `## Review Schedule` in DPIA; after `## Post-Mortem Review` in INC). For templates that already include a `## Approval` table (ETH, MCARD, SEC, PIPIA, CACFILE, TC260RA, AILABEL), either form is canonical; the frontmatter fields are the machine-readable source of truth.

```markdown
## Approval

**Approved**: 2026-05-02 by `pepe@example.com`.

<Optional reviewer notes — observations, conditions, scope of approval. Omit
the entire section if there's nothing to add beyond the frontmatter.>
```

### Multi-reviewer flows (forward-looking)

For documents that require multiple reviewers (e.g., ETH with both legal and engineering sign-off), the canon for v1 is to append additional `## Approval` blocks chronologically in the body, with the frontmatter reflecting the *latest* approval. A structured `review:` array form (one entry per reviewer) is forward-looking and not part of v1 — it will be added when at least one adopter exercises the multi-reviewer flow with real data.

### CLI tooling

`straymark approve <doc-id> --outcome approved --reviewer <id> [--notes "..."] [--at YYYY-MM-DD]` writes both the frontmatter fields and the body section in one shot. `straymark validate --check-pending-reviews [--max-pending-days N]` lists `review_required: true` documents older than `N` days without a `review_outcome` (warn-only, no error). See CLI-REFERENCE.md.

---

## 4. Risk Levels

| Level | When to use | Requires review |
|-------|-------------|-----------------|
| `low` | Cosmetic changes, documentation | No |
| `medium` | New functionality, refactoring | Recommended |
| `high` | Security, sensitive data, public APIs | Yes |
| `critical` | Irreversible changes, production | Mandatory |

---

## 5. Confidence Levels

| Level | Meaning | Action |
|-------|---------|--------|
| `high` | The agent is certain about the decision | Proceed |
| `medium` | The agent has minor doubts | Document alternatives |
| `low` | The agent needs validation | Mark `review_required: true` |

---

## 6. Folder Structure

```
.straymark/
├── 00-governance/          # Policies and rules
├── 01-requirements/        # System requirements
├── 02-design/              # Design and architecture
│   └── decisions/          # ADRs
├── 03-implementation/      # Implementation guides
├── 04-testing/             # Test strategies
├── 05-operations/          # Operations
│   └── incidents/          # Post-mortems
├── 06-evolution/           # System evolution
│   └── technical-debt/     # Technical debt
├── 07-ai-audit/            # AI agent audit
│   ├── agent-logs/         # AILOG
│   ├── decisions/          # AIDEC
│   └── ethical-reviews/    # ETH
├── 08-security/            # SEC — Security assessments
├── 09-ai-models/           # MCARD — Model/System cards
└── templates/              # Templates
```

### Document Types

| Type | Name | Folder | Default Status | Review Required |
|------|------|--------|---------------|----------------|
| `AILOG` | AI Action Log | `07-ai-audit/agent-logs/` | `accepted` | No |
| `AIDEC` | AI Decision | `07-ai-audit/decisions/` | `accepted` | No |
| `ETH` | Ethical Review | `07-ai-audit/ethical-reviews/` | `draft` | Yes |
| `ADR` | Architecture Decision Record | `02-design/decisions/` | `draft` | Yes |
| `REQ` | Requirement | `01-requirements/` | `draft` | Yes |
| `TES` | Test Plan | `04-testing/` | `draft` | Yes |
| `INC` | Incident Post-mortem | `05-operations/incidents/` | `draft` | Yes |
| `TDE` | Technical Debt | `06-evolution/technical-debt/` | `identified` (enters here; terminal `resolved` when debt paid — TDE-only) | No |
| `SEC` | Security Assessment | `08-security/` | `draft` | Yes (always) |
| `MCARD` | Model/System Card | `09-ai-models/` | `draft` | Yes (always) |
| `SBOM` | Software Bill of Materials | `07-ai-audit/` | `accepted` | No |
| `DPIA` | Data Protection Impact Assessment | `07-ai-audit/ethical-reviews/` | `draft` | Yes (always) |

---

## 7. Cross-References

Use the `[TYPE-ID]` format for references:

```markdown
This decision is based on the requirements defined in [REQ-2025-01-15-003].
See also [ADR-2025-01-20-001] for architectural context.
```

---

## 8. Referenced Standards

| Standard | Version | Scope in StrayMark |
|----------|---------|-------------------|
| ISO/IEC/IEEE 29148:2018 | 2018 | Requirements engineering — TEMPLATE-REQ.md |
| ISO/IEC 25010:2023 | 2023 | Software quality model — TEMPLATE-REQ.md, TEMPLATE-ADR.md |
| ISO/IEC/IEEE 29119-3:2021 | 2021 | Software testing documentation — TEMPLATE-TES.md |
| ISO/IEC 42001:2023 | 2023 | AI Management System — AI-GOVERNANCE-POLICY.md (vertebral standard) |
| EU AI Act | 2024 (effective Aug 2026) | AI regulation — ETH, INC, AILOG regulatory fields |
| NIST AI RMF 1.0 | 2023 | AI risk management — ETH, AILOG risk categories |
| NIST AI 600-1 | 2024 | Generative AI profile — 12 risk categories in ETH/AILOG |
| ISO/IEC 23894:2023 | 2023 | AI risk management — AI-RISK-CATALOG |
| GDPR | 2016/679 | Data protection — ETH (Data Privacy), DPIA |
| OpenTelemetry | Current | Observability — OBSERVABILITY-GUIDE, optional |
| TC260 AI Safety Governance Framework v2.0 *(China)* | Sept 2025 | AI risk grading — TEMPLATE-TC260RA.md (opt-in) |
| PIPL — Personal Information Protection Law *(China)* | 2021 | Data protection / PIPIA — TEMPLATE-PIPIA.md (opt-in) |
| GB 45438-2025 *(China, mandatory)* | Sept 2025 | AI-generated content labeling — TEMPLATE-AILABEL.md (opt-in) |
| CAC Algorithm Filing *(China)* | 2022+ | Algorithm registration — TEMPLATE-CACFILE.md (opt-in) |
| GB/T 45652-2025 *(China)* | Nov 2025 | Training-data security — TEMPLATE-SBOM.md, TEMPLATE-MCARD.md (opt-in) |
| CSL 2026 Cybersecurity Law *(China)* | Jan 2026 | Incident reporting — TEMPLATE-INC.md (opt-in) |

> *Opt-in standards* are evaluated only when `regional_scope: china` is enabled in `.straymark/config.yml`. See `CHINA-REGULATORY-FRAMEWORK.md` for the full mapping.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
