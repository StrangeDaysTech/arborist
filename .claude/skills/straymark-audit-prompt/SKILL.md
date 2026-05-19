---
name: straymark-audit-prompt
description: Generate the unified audit prompt for a Charter at the canonical filesystem location. The operator then opens N auditor-side CLIs (gemini-cli, claude-cli, copilot-cli, etc.) and invokes /straymark-audit-execute in each — no copy/paste. Counterpart of /straymark-audit-review.
allowed-tools: Read, Bash(straymark charter audit *, straymark charter status *, ls *)
---

# StrayMark Audit Prompt Skill

Generate the unified audit prompt for a Charter and write it to the canonical filesystem location. The operator then opens auditor-side CLIs in the same repo and invokes `/straymark-audit-execute` — the audit prompt is read directly from disk by each auditor; the operator never copies/pastes.

## When to invoke

Use this skill when the developer agreed to run an external audit at the Charter checkpoint (see `.straymark/00-governance/AGENT-RULES.md` § Audit checkpoint).

The Charter should be in `in-progress` or `declared` status — auditing closed Charters is allowed but atypical (warn the operator and proceed only on confirmation).

## Instructions

### 1. Resolve the Charter

Argument: a Charter identifier (`CHARTER-04`, `04`, or full id with slug).

```bash
straymark charter status <CHARTER-ID>
```

Verify the Charter exists and capture its `status`. If `status: closed`, surface a one-line warning to the operator and ask whether to proceed.

### 2. Generate the unified audit prompt

```bash
straymark charter audit <CHARTER-ID> --prepare
```

The CLI writes the resolved prompt to:

```
.straymark/audits/<CHARTER-ID>/audit-prompt.md
```

The prompt is self-contained: it embeds the Charter content, the originating AILOGs, the git diff over the resolved range (default `origin/main..HEAD`, falls back to `HEAD~1..HEAD` if no upstream is reachable), and the discipline rules (REGLA ABSOLUTA — SOLO LECTURA, evidence-citation, severity calibration). The prompt template lifts the seven universal sections from Sentinel's pre-StrayMark audit skill and parameterizes the project-specific hardcodes.

The CLI does NOT invoke any LLM. It only resolves placeholders.

### 3. Notify the operator

After the CLI succeeds, print this guidance verbatim (substituting `<CHARTER-ID>`):

```
Audit prompt prepared for <CHARTER-ID>.

  Location: .straymark/audits/<CHARTER-ID>/audit-prompt.md
  (informational — you don't need to copy this path anywhere)

Next steps:

  1. Open one or more auditor-side CLIs (gemini-cli, claude-cli,
     copilot-cli, codex-cli — whatever you have) in this repo. Each
     CLI session uses its own model; recommendation is at least 2
     auditors of DIFFERENT model families, so cross-family blind
     spots become signal when their findings converge.

  2. In each auditor CLI, invoke:

         /straymark-audit-execute <CHARTER-ID>

     The skill is already installed (straymark init copies it to all
     three platform locations). It reads the prompt from disk
     automatically — you do not need to copy/paste anything.

  3. When and only when ALL audits you commissioned have completed,
     return to this main agent and run:

         /straymark-audit-review <CHARTER-ID>

     Reviewing with incomplete reports gives you a partial consolidated
     analysis that you will have to discard or re-run.
```

## Notes

- This skill is **orchestration-only**. It does NOT invoke LLM APIs, decide which models the operator uses, or wait for responses. It runs the CLI's `--prepare`, points the operator at the next-step skill, and exits.
- Re-running the skill on the same Charter regenerates the prompt at the same path. It does NOT touch operator-saved reports under `.straymark/audits/<CHARTER-ID>/report-*.md`.
- Heterogeneity inter-family is recommended but not enforced in v0; the operator decides the model pairing across the N CLIs they open.
- For the rationale on cross-family auditors, see `dist/.straymark/audit-prompts/audit-prompt.md` § Tu rol or `Propuesta/straymark-cli-roadmap.md` §5.2 in the upstream repo.
