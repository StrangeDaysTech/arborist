---
name: straymark-charter-new
description: Scaffold a Charter — StrayMark's bounded unit of work that pairs declarative ex-ante scope with telemetry ex-post. Use when starting a multi-session implementation block (>1 day, >5 tasks, multi-phase).
allowed-tools: Read, Write, Glob, Bash(git diff *, git log *, git status *, date *, wc *, ls *, straymark charter *)
---

# StrayMark Charter Scaffold Skill

Declare a Charter at the start of a bounded, auditable unit of work — *not* every change. Charters wrap multi-session work that warrants a stable scope contract you can drift-check at close. They are conceptually distinct from the 12+4 governance document types (AILOG, ADR, AIDEC, …): Charters live at `.straymark/charters/NN-slug.md` and use a sequential prefix instead of a date prefix.

> See `STRAYMARK.md §15` and `.straymark/00-governance/SPECKIT-CHARTER-BRIDGE.md` for the lifecycle and the SpecKit ↔ Charter bridge.

## When to use this skill

Trigger on any of:

- Multi-session implementation block (>1 day, >5 tasks across phases).
- Work that warrants external audit at completion.
- A SpecKit feature has reached `tasks.md` and the operator wants a stable scope contract before `/speckit-implement`.
- The user asks to "declare", "open", or "start" a Charter.

If the work is a single-session change → use `/straymark-ailog` instead.

## Instructions

### 1. Gather context

```bash
# Sequential number of the Charter being created
straymark charter list 2>/dev/null | tail -5 || true

# Surrounding spec / AILOG context — Charters frequently originate from one
ls specs/*/spec.md 2>/dev/null | head -5
ls .straymark/07-ai-audit/agent-logs/AILOG-*.md 2>/dev/null | tail -5
```

### 2. Confirm with user

```
╔══════════════════════════════════════════════════════════════════╗
║  StrayMark Charter — declare a bounded unit of work               ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  📋 A Charter pairs declarative scope (ex-ante) with telemetry    ║
║     (ex-post). Lifecycle: declared → in-progress → closed.        ║
║                                                                   ║
║  Required:                                                        ║
║  1. Title (one line)                                              ║
║  2. Effort estimate: XS | S | M | L                               ║
║                                                                   ║
║  One of (optional but encouraged):                                ║
║  3a. --from-ailog AILOG-ID    (post-MVP / maintenance origin)     ║
║  3b. --from-spec specs/.../spec.md  (greenfield / SpecKit origin) ║
║                                                                   ║
║  Default effort: M.                                               ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

### 3. Pick origin and effort

- **Effort heuristic**: XS (≤ half day), S (≤ 1 day), M (1–3 days, default), L (≥ 1 week).
- **Origin precedence**: a single live AILOG → `--from-ailog`; a SpecKit feature spec → `--from-spec`; otherwise omit (the operator can fill `originating_*` fields manually).

### 4. Run the CLI

The CLI does the scaffolding — slug derivation, sequential numbering, template substitution, write to `.straymark/charters/NN-slug.md`. The skill's job is to drive it with the right flags.

```bash
# Greenfield, SpecKit-driven
straymark charter new \
  --title "Workspace foundation for peek MVP" \
  --type M \
  --from-spec specs/001-peek-mvp-foundation/spec.md

# Post-MVP / maintenance, AILOG-driven
straymark charter new \
  --title "Per-service anomaly thresholds" \
  --type S \
  --from-ailog AILOG-2026-04-28-021

# No explicit origin (operator fills frontmatter manually)
straymark charter new --title "Refactor signal pipeline" --type M
```

If the title would derive a poor slug, pass `--slug <slug>` explicitly.

### 5. Hand off to the operator

After the file is created, the CLI's "Next steps" output already lists what to fill. Surface it verbatim, then add:

> **Reminder**: Charter status starts at `declared`. Flip to `in-progress` only when execution actually begins. Run `straymark charter drift CHARTER-NN` before `straymark charter close` to catch declared-but-not-modified files (or modified-but-not-declared ones).

### 6. Report result

```
✓ Charter declared:
   .straymark/charters/NN-slug.md

   charter_id: CHARTER-NN-slug
   status: declared
   effort_estimate: <XS|S|M|L>
   origin: <ailog | spec | none>

StrayMark: Created CHARTER-NN-slug
```

## What this skill does NOT do

- **It does not flip status to `in-progress` or `closed`.** Lifecycle transitions are operator decisions; pumps the operator through `straymark charter close` (or manual frontmatter edit for `in-progress`).
- **It does not run drift or audit.** Use `straymark charter drift` and `/straymark-audit-prompt` / `/straymark-audit-execute` / `/straymark-audit-review` for those phases.
- **It does not replace AILOGs.** Day-to-day work inside the Charter still produces AILOGs; the Charter aggregates them via `originating_ailogs:`.

> **Terminal compatibility**: If the terminal does not support box-drawing characters (Unicode), use plain-text formatting with dashes and pipes instead (e.g., `+--+` instead of `╔══╗`).
