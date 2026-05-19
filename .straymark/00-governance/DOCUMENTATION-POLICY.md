# Documentation Policy - DevTrail

## Governance Framework

This policy aligns DevTrail documentation with **ISO/IEC 42001:2023** (vertebral standard for AI Management Systems) and operationalizes:

- **EU AI Act** (effective August 2026) — risk classification, transparency, incident reporting
- **NIST AI RMF 1.0 + AI 600-1** — AI risk management functions and generative AI profiles
- **ISO/IEC 23894:2023** — AI risk management framework
- **GDPR** — data protection and privacy impact assessments

All document types, metadata fields, and governance rules contribute to evidence that satisfies these regulatory frameworks. See Section 8 for the complete standards reference.

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

Related references link documents to other **DevTrail documents** within the same project. They enable cross-navigation in tools like `devtrail explore`.

**Format rules:**
- Use the **document filename** (with `.md` extension): `AILOG-2026-02-03-001-implement-sync-item.md`
- For governance or non-typed documents, use the filename as-is: `AGENT-RULES.md`, `DOCUMENTATION-POLICY.md`
- Paths are resolved relative to `.devtrail/` — if the document is in a subdirectory, include the path from `.devtrail/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md`
- When the file is in the same directory as the referencing document, the filename alone is sufficient
- **Do not use** external task IDs (`T001`, `US3`), issue numbers, or URLs — those belong in the document body, not in frontmatter
- **Do not use** partial IDs without description (prefer `AILOG-2026-02-03-001-implement-sync-item.md` over `AILOG-2026-02-03-001`)

**Examples:**
```yaml
# Same directory or well-known location — filename is enough
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# Documents in specific subdirectories — include path from .devtrail/
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implement-sync-item.md
  - 02-design/decisions/ADR-2026-01-15-001-use-hexagonal-architecture.md
```

**Resolution:** The CLI resolves references by searching: (1) exact ID match, (2) filename match anywhere in `.devtrail/`, (3) path suffix match. Using the full filename provides the most reliable resolution.

---

## 3. Document Statuses

```
draft ──────► accepted ──────► deprecated
                │                   │
                │                   ▼
                └──────► superseded
```

| Status | Description |
|--------|-------------|
| `draft` | In draft, pending review |
| `accepted` | Approved and current |
| `deprecated` | Obsolete, but kept as reference |
| `superseded` | Replaced by another document |

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
.devtrail/
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
| `TDE` | Technical Debt | `06-evolution/technical-debt/` | `identified` | No |
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

| Standard | Version | Scope in DevTrail |
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

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
