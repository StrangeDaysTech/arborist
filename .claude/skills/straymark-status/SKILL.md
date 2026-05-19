---
name: straymark-status
description: Show StrayMark documentation status. Use to verify if AI agents properly documented their changes.
allowed-tools: Read, Glob, Bash(git diff *, git log *, git status *, date *, straymark analyze *)
---

# StrayMark Status Skill

This skill checks the documentation status of the StrayMark in the current project.

## Instructions

When invoked, perform the following checks and display the results:

### 1. Find Recent StrayMark Documents

Search for StrayMark documents created or modified in the last hour:

```bash
# Get documents modified in the last hour
git log --since="1 hour ago" --name-only --pretty=format: -- ".straymark/**/*.md" | sort -u | grep -v "^$"
```

If git is not available or the directory is not a git repo, use file modification times.
Check these directories for each document type:

| Type | Prefix | Directory |
|------|--------|-----------|
| AILOG | `AILOG-` | `.straymark/07-ai-audit/agent-logs/` |
| AIDEC | `AIDEC-` | `.straymark/07-ai-audit/decisions/` |
| ADR | `ADR-` | `.straymark/02-design/decisions/` |
| ETH | `ETH-` | `.straymark/07-ai-audit/ethical-reviews/` |
| REQ | `REQ-` | `.straymark/01-requirements/` |
| TES | `TES-` | `.straymark/04-testing/` |
| INC | `INC-` | `.straymark/05-operations/incidents/` |
| TDE | `TDE-` | `.straymark/06-evolution/technical-debt/` |
| SEC | `SEC-` | `.straymark/08-security/` |
| MCARD | `MCARD-` | `.straymark/09-ai-models/` |
| SBOM | `SBOM-` | `.straymark/07-ai-audit/` |
| DPIA | `DPIA-` | `.straymark/07-ai-audit/ethical-reviews/` |

Also enumerate **Charters** (bounded units of work — distinct from doc types; see STRAYMARK.md §15):

```bash
# Charters list with status counts (declared / in-progress / closed)
straymark charter list 2>/dev/null
```

If the project has no Charters yet but the work clearly fits the trigger (multi-session implementation block, >5 tasks across phases, audit value), surface that as a gap in the Display Results step and recommend `/straymark-charter-new`.

### 2. Find Modified Source Files

Identify source files that were modified and might need documentation:

```bash
# Get modified files (staged and unstaged)
git diff --name-only HEAD~1..HEAD 2>/dev/null || git diff --name-only
```

Filter to show only files that might need documentation:
- Exclude: `*.md`, `*.json`, `*.yml`, `*.yaml`, `*.lock`, `package-lock.json`
- Include: `*.ts`, `*.js`, `*.tsx`, `*.jsx`, `*.py`, `*.go`, `*.rs`, `*.java`, `*.cs`, `*.rb`, `*.php`

Run complexity analysis on modified source files:

```bash
# Analyze complexity of changed files (primary method for AILOG trigger)
straymark analyze --output json 2>/dev/null
# If CLI unavailable, fall back to line count heuristic in step 3
```

### 3. Analyze Documentation Gaps

For each modified source file, check if there's a corresponding StrayMark document:
- Complex code (`straymark analyze` reports `summary.above_threshold > 0`; **fallback** if CLI unavailable: >20 lines of business logic) should have an AILOG
- Security-related files (auth, crypto, secrets) should have a SEC assessment
- Architecture/structural changes should have an ADR
- AI/ML model changes should have a MCARD
- Dependency changes (`package.json`, `Cargo.toml`, `go.mod`, etc.) should have an SBOM
- Changes involving personal data processing should have a DPIA
- Test files should have a TES record
- Bug fixes or incidents should have an INC record
- Multi-session implementation blocks (>1 day, >5 tasks, multi-phase) should have an open or closed **Charter** at `.straymark/charters/`

### 4. Display Results

Format the output as follows:

```
StrayMark Status
================================================================================

Recent Documents (last hour):
  [checkmark] AILOG-2025-01-27-001-implement-auth.md
  [checkmark] AIDEC-2025-01-27-001-auth-strategy.md

Modified Files Without Documentation:
  [warning] src/auth/login.ts (cognitive: 12, threshold: 8)
  [warning] src/api/users.ts (cognitive: 9, threshold: 8)

Summary:
  Documents created: 2
  Files needing review: 2

Use /straymark-status after making changes to verify documentation compliance.
```

### Symbol Legend

- `[checkmark]` = Documentation exists (use checkmark symbol)
- `[warning]` = May need documentation (use warning symbol)
- `[info]` = Informational (use info symbol)

### Edge Cases

1. **No git repository**: Show message explaining git is required for full functionality
2. **No recent documents**: Show "No StrayMark documents created in the last hour"
3. **No modified files**: Show "No source files modified recently"
4. **Empty .straymark directory**: Show warning that StrayMark may not be properly set up

### Additional Notes

- This skill is read-only and does not create or modify files
- Run this after completing development tasks to verify documentation compliance
- If files need documentation, remind the user of the StrayMark rules
