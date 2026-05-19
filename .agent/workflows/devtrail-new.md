---
description: Create DevTrail documentation. Analyzes context to suggest document type or accepts explicit type parameter. Always confirms before creating.
---

# DevTrail New Skill

Create DevTrail documentation based on recent changes.

## Instructions

When invoked, follow these steps:

### 1. Check for Parameters

If the user specified a document type (e.g., `/devtrail-new ailog`), skip to step 4 using that type.

Valid types: `ailog`, `aidec`, `adr`, `eth`, `req`, `tes`, `inc`, `tde`, `sec`, `mcard`, `sbom`, `dpia`

### 2. Analyze Context

Gather information about recent changes:

```bash
# Get current date
date +%Y-%m-%d

# Get modified files (staged and unstaged)
git status --porcelain

# Get recent changes summary
git diff --stat HEAD~1 2>/dev/null || git diff --stat

# Count lines changed
git diff --numstat HEAD~1 2>/dev/null || git diff --numstat

# Check code complexity (primary method for AILOG trigger)
devtrail analyze --output json 2>/dev/null
# If CLI unavailable, fall back to line count heuristic in step 3
```

### 3. Classify and Suggest Type

Based on the analysis, suggest a document type:

| Pattern | Suggested Type |
|---------|---------------|
| Complex code (`devtrail analyze` `above_threshold > 0`; fallback: >20 lines) | AILOG |
| Multiple implementation alternatives discussed | AIDEC |
| Structural/architectural changes, new modules | ADR |
| Files with `auth`, `user`, `privacy`, `gdpr` | ETH (draft) |
| Test files (`*.test.*`, `*.spec.*`) | TES |
| Bug fixes, hotfixes | INC |
| `TODO`, `FIXME`, `HACK` comments added | TDE |
| Requirements or spec files | REQ |

### 4. Confirm with User

**Always display this confirmation before creating:**

```
╔══════════════════════════════════════════════════════════════════╗
║  DevTrail New                                                    ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  📊 Analysis:                                                     ║
║     • Files modified: [N]                                         ║
║     • Lines changed: [+X / -Y]                                    ║
║     • Area: [detected area or "general"]                          ║
║                                                                   ║
║  🎯 Suggested type: [TYPE] ([Full Name])                          ║
║     Reason: [Brief explanation]                                   ║
║                                                                   ║
║  📝 Proposed filename:                                            ║
║     [TYPE]-YYYY-MM-DD-NNN-[description].md                        ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝

Confirm creation? [Y/n/other type]:
```

Wait for user confirmation before proceeding.

### 5. Check Language Configuration

Read `.devtrail/config.yml` to determine language:

```yaml
language: en  # or es
```

Use template path based on language:
- `en` (default): `.devtrail/templates/TEMPLATE-[TYPE].md`
- `es`: `.devtrail/templates/i18n/es/TEMPLATE-[TYPE].md`

### 6. Generate Document ID

Determine the next sequence number:

```bash
# Find existing documents of this type for today
ls .devtrail/*/[TYPE]-$(date +%Y-%m-%d)-*.md 2>/dev/null | wc -l
```

ID format: `[TYPE]-YYYY-MM-DD-NNN`

### 7. Load Template and Create Document

1. Read the appropriate template
2. Replace placeholders:
   - `YYYY-MM-DD` → Current date
   - `NNN` → Sequence number (001, 002, etc.)
   - `[agent-name-v1.0]` → Your agent identifier (e.g., `cursor-v1.0`, `copilot-v1.0`, `windsurf-v1.0`)
3. Fill in context from git analysis
4. Save to correct location:

| Type | Location |
|------|----------|
| AILOG | `.devtrail/07-ai-audit/agent-logs/` |
| AIDEC | `.devtrail/07-ai-audit/decisions/` |
| ETH | `.devtrail/07-ai-audit/ethical-reviews/` |
| ADR | `.devtrail/02-design/decisions/` |
| REQ | `.devtrail/01-requirements/` |
| TES | `.devtrail/04-testing/` |
| INC | `.devtrail/05-operations/incidents/` |
| TDE | `.devtrail/06-evolution/technical-debt/` |
| SEC | `.devtrail/08-security/` |
| MCARD | `.devtrail/09-ai-models/` |
| SBOM | `.devtrail/07-ai-audit/` |
| DPIA | `.devtrail/07-ai-audit/ethical-reviews/` |

### 7.5. Apply Automatic Review Rules

Before saving, apply these validation rules to the frontmatter:

- If `risk_level` is `high` or `critical`: set `review_required: true`
- If `eu_ai_act_risk` is `high`: set `review_required: true`
- If document type is SEC, MCARD, or DPIA: set `review_required: true`

These rules align with the CLI validation rules CROSS-001, CROSS-002, and CROSS-003.

### 8. Report Result

After creation, display:

```
✅ DevTrail document created:
   .devtrail/[path]/[TYPE]-YYYY-MM-DD-NNN-description.md

   Review required: [yes/no]
   Risk level: [low/medium/high/critical]
```

## Document Type Reference

| Type | Full Name | Purpose |
|------|-----------|---------|-
| `ailog` | AI Action Log | Record what the AI agent did |
| `aidec` | AI Decision | Document a technical decision with alternatives |
| `adr` | Architecture Decision Record | Major architectural decisions |
| `eth` | Ethical Review | Privacy, bias, responsible AI concerns |
| `req` | Requirement | System requirements |
| `tes` | Test Plan | Test strategies and plans |
| `inc` | Incident Post-mortem | Incident analysis |
| `tde` | Technical Debt | Identified technical debt |
| `sec` | Security Assessment | Threat modeling and security controls |
| `mcard` | Model/System Card | AI model documentation |
| `sbom` | Software Bill of Materials | AI component inventory |
| `dpia` | Data Protection Impact Assessment | Privacy impact analysis |

## Edge Cases

1. **No git repository**: Inform user that git is required for context analysis
2. **No changes detected**: Ask user to describe what to document
3. **User declines**: Acknowledge and exit gracefully
4. **Invalid type parameter**: Show valid types and ask again
