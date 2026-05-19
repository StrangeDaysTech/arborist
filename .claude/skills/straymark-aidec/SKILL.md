---
name: straymark-aidec
description: Create an AIDEC (AI Decision) document to record a technical decision with alternatives considered.
allowed-tools: Read, Write, Glob, Bash(git diff *, git log *, git status *, date *, wc *)
---

# StrayMark AIDEC Skill

Create an AI Decision (AIDEC) document to record technical decisions and the alternatives considered.

## Instructions

Use this skill when you've made a decision between multiple technical approaches and want to document the reasoning.

### 1. Gather Context

```bash
# Get current date
date +%Y-%m-%d

# Get recent changes for context
git diff --stat HEAD~1 2>/dev/null || git diff --stat
```

### 2. Confirm with User

**Always confirm before creating:**

```
╔══════════════════════════════════════════════════════════════════╗
║  StrayMark AIDEC                                                  ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  📋 AIDEC documents a decision between alternatives.              ║
║                                                                   ║
║  Please provide:                                                  ║
║  1. Decision title (what was decided)                             ║
║  2. Alternatives considered (2-3 options)                         ║
║  3. Which alternative was chosen and why                          ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

### 3. Determine Sequence Number

```bash
# Count existing AIDECs for today
ls .straymark/07-ai-audit/decisions/AIDEC-$(date +%Y-%m-%d)-*.md 2>/dev/null | wc -l
```

### 4. Check Language and Load Template

Read `.straymark/config.yml` for language setting:
- `en` (default): `.straymark/templates/TEMPLATE-AIDEC.md`
- `es`: `.straymark/templates/i18n/es/TEMPLATE-AIDEC.md`

### 5. Create Document

Fill template with:
- `id`: AIDEC-YYYY-MM-DD-NNN
- `title`: Decision title from user
- `created`: Current date
- `agent`: claude-code-v1.0
- `confidence`: based on decision clarity
- `risk_level`: based on decision impact

**Key sections to fill:**
- Context: Why was a decision needed?
- Problem: What specific problem needed solving?
- Alternatives Considered: At least 2 options with pros/cons
- Decision: Which was chosen and justification
- Consequences: Positive, negative, and risks

Save to: `.straymark/07-ai-audit/decisions/AIDEC-YYYY-MM-DD-NNN-description.md`

### 6. Report Result

```
✅ AIDEC created:
   .straymark/07-ai-audit/decisions/AIDEC-YYYY-MM-DD-NNN-description.md

StrayMark: Created AIDEC-YYYY-MM-DD-NNN-description.md
```

## When to Create an AIDEC

- Chose between frameworks/libraries
- Selected an algorithm or approach
- Decided on a data structure
- Made a performance vs. readability trade-off
- Selected between design patterns

> **Terminal compatibility**: If the terminal does not support box-drawing characters (Unicode), use plain-text formatting with dashes and pipes instead (e.g., `+--+` instead of `╔══╗`).
