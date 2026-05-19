---
description: Show DevTrail documentation status. Use to verify if AI agents properly documented their changes.
---

# DevTrail Status Skill

This skill checks the documentation status of the DevTrail in the current project.

## Instructions

When invoked, perform the following checks and display the results:

### 1. Find Recent DevTrail Documents

Search for DevTrail documents created or modified in the last hour:

```bash
# Get documents modified in the last hour
git log --since="1 hour ago" --name-only --pretty=format: -- ".devtrail/**/*.md" | sort -u | grep -v "^$"
```

If git is not available or the directory is not a git repo, use file modification times.
Check these directories for each document type:

| Type | Prefix | Directory |
|------|--------|-----------|
| AILOG | `AILOG-` | `.devtrail/07-ai-audit/agent-logs/` |
| AIDEC | `AIDEC-` | `.devtrail/07-ai-audit/decisions/` |
| ADR | `ADR-` | `.devtrail/04-architecture/decisions/` |
| ETH | `ETH-` | `.devtrail/07-ai-audit/ethical-reviews/` |
| REQ | `REQ-` | `.devtrail/03-requirements/` |
| TES | `TES-` | `.devtrail/05-testing/` |
| INC | `INC-` | `.devtrail/06-operations/incidents/` |
| TDE | `TDE-` | `.devtrail/06-operations/tech-debt/` |
| SEC | `SEC-` | `.devtrail/08-security/` |
| MCARD | `MCARD-` | `.devtrail/09-ai-models/` |
| SBOM | `SBOM-` | `.devtrail/07-ai-audit/` |
| DPIA | `DPIA-` | `.devtrail/07-ai-audit/ethical-reviews/` |

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
devtrail analyze --output json 2>/dev/null
# If CLI unavailable, fall back to line count heuristic in step 3
```

### 3. Analyze Documentation Gaps

For each modified source file, check if there's a corresponding DevTrail document:
- Complex code (`devtrail analyze` reports `summary.above_threshold > 0`; **fallback** if CLI unavailable: >20 lines of business logic) should have an AILOG
- Security-related files (auth, crypto, secrets) should have a SEC assessment
- Architecture/structural changes should have an ADR
- AI/ML model changes should have a MCARD
- Dependency changes (`package.json`, `Cargo.toml`, `go.mod`, etc.) should have an SBOM
- Changes involving personal data processing should have a DPIA
- Test files should have a TES record
- Bug fixes or incidents should have an INC record

### 4. Display Results

Format the output as follows:

```
DevTrail Status
================================================================================

Recent Documents (last hour):
  ✅ AILOG-2025-01-27-001-implement-auth.md
  ✅ AIDEC-2025-01-27-001-auth-strategy.md

Modified Files Without Documentation:
  ⚠️ src/auth/login.ts (cognitive: 12, threshold: 8)
  ⚠️ src/api/users.ts (cognitive: 9, threshold: 8)

Summary:
  Documents created: 2
  Files needing review: 2

Use /devtrail-new to create documentation for undocumented changes.
```

### Symbol Legend

- ✅ = Documentation exists
- ⚠️ = May need documentation
- ℹ️ = Informational

### Edge Cases

1. **No git repository**: Show message explaining git is required for full functionality
2. **No recent documents**: Show "No DevTrail documents created in the last hour"
3. **No modified files**: Show "No source files modified recently"
4. **Empty .devtrail directory**: Show warning that DevTrail may not be properly set up

### Additional Notes

- This skill is read-only and does not create or modify files
- Run this after completing development tasks to verify documentation compliance
- If files need documentation, remind the user of the DevTrail rules
