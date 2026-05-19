---
name: straymark-audit-execute
description: Execute an external audit of a Charter inside an auditor-side CLI (gemini-cli, claude-cli, copilot-cli, codex-cli). Reads the resolved audit prompt from the canonical location, audits with tool use, writes the report. Operator invokes one instance per CLI.
allowed-tools: Read, Write, Glob, Grep, Bash(go vet *, go build *, go test *, cargo check *, cargo clippy *, cargo test --no-run, npm run lint, npm run typecheck, npm test --, pytest --co, mypy *, ruff check *, ls *, find *, wc *, git log *, git diff *, git status *)
argument-hint: "CHARTER-NN (optional — auto-discovers pending prompts)"
---

# StrayMark Audit Execute Skill

Execute an external audit of a Charter inside this CLI session. Read the resolved audit prompt that StrayMark prepared at the canonical location, audit the implementation with tool use (citing `path:line` of files actually opened), and write the report at the canonical location for the audit-review skill to consolidate later.

## When to invoke

This skill runs **inside an auditor-side CLI** (gemini-cli, claude-cli, copilot-cli, codex-cli, or any agent runtime configured with read-only access to the adopter's repo). The operator opens the CLI in the repo, then invokes `/straymark-audit-execute <CHARTER-ID>`.

The skill is the second step of the v1 audit cycle:

1. In the main IDE: operator runs `/straymark-audit-prompt CHARTER-NN` → StrayMark writes `.straymark/audits/CHARTER-NN/audit-prompt.md`.
2. **(this skill)** Operator opens an auditor-side CLI in the repo and runs `/straymark-audit-execute CHARTER-NN`. Repeat in N CLIs (recommended: ≥2 of different model families).
3. When ALL audits commissioned have completed, operator returns to the main IDE and runs `/straymark-audit-review CHARTER-NN`.

## Instructions

### 1. Resolve the Charter

The argument is optional. Two cases:

**Case A — argument provided** (`/straymark-audit-execute CHARTER-04`):
Use the literal value. Construct the audit dir path: `.straymark/audits/CHARTER-04/`.

**Case B — argument omitted** (`/straymark-audit-execute`):
Auto-discover pending prompts. Detect this auditor's model identifier (see step 2 for how) and produce its slug. Then:

```bash
# List all audit prompts that exist
ls .straymark/audits/*/audit-prompt.md 2>/dev/null
```

For each found `.straymark/audits/<CHARTER-ID>/audit-prompt.md`, check whether a report from this model already exists at `.straymark/audits/<CHARTER-ID>/report-<self-model-slug>.md`. The set of "pending" prompts is the ones WITHOUT a corresponding report.

- **Exactly one pending** → proceed with that CHARTER-ID, announcing the choice to the operator.
- **Multiple pending** → list them numerically with their Charter titles (read the title from the resolved prompt's `# Auditoría de Charter — CHARTER-NN` heading) and ask the operator to pick one.
- **None pending** → message: "No pending audit prompts for this model under `.straymark/audits/`. Either the operator has not run `/straymark-audit-prompt` in the main agent yet, or all the prompts already have a report from this model. Verify with the operator."

### 2. Detect this auditor's model identifier

Identify the model running this CLI session and produce a filesystem-safe slug.

The operator will know the canonical model id of the auditor running this skill (for example `claude-sonnet-4-6`, `claude-opus-4-7`, `gemini-2.5-pro`, `gpt-5.3-codex`, `copilot-v1.0.40`). Slug rules:

- Lowercase ASCII.
- Replace any character that isn't `[a-z0-9-]` with `-`.
- Collapse consecutive dashes to one.
- Trim leading/trailing dashes.

Examples:

| Model identifier | Slug |
|---|---|
| `claude-sonnet-4-6` | `claude-sonnet-4-6` |
| `gemini-2.5-pro` | `gemini-2-5-pro` |
| `gpt-5.3-codex` | `gpt-5-3-codex` |
| `copilot-v1.0.40` | `copilot-v1-0-40` |

If the runtime does not expose the model identifier directly, ask the operator to confirm before proceeding. Do NOT fabricate a slug — collisions or wrong identifiers corrupt the review step.

### 3. Read the audit prompt

```bash
cat .straymark/audits/<CHARTER-ID>/audit-prompt.md
```

The prompt is self-contained: it includes the Charter content, originating AILOGs, git diff, and the discipline rules (REGLA ABSOLUTA — SOLO LECTURA, evidence-citation discipline, severity calibration). Read it carefully before auditing.

### 4. Audit with tool use

Follow the prompt literally, with these expectations:

- **Read-only**: never write to project files. The only output you are allowed to produce is the report at the canonical path in step 5.
- **Tool-use evidence**: every finding you record must cite `path:line` of files you actually opened via `Read`, `Grep`, or equivalent. Do not infer behavior from file names alone.
- **Severity calibration**: open the active configuration (factories, env defaults, build tags, deployment scaling) before declaring Critical/High severity. The Etapa 12 example in the prompt is a real case of inflation that the calibration discipline catches.
- **Scope discipline**: only report findings inside the Charter's declared scope. Out-of-scope observations go in their own section, not as defects.

Track how many `path:line` citations you accumulate — it goes in the report frontmatter as `evidence_citations`.

### 5. Write the report

Output path:

```
.straymark/audits/<CHARTER-ID>/report-<self-model-slug>.md
```

If a report at that exact path already exists (re-run on the same Charter with the same model), warn the operator before overwriting. The default is to overwrite — re-runs replace stale reports rather than coexist with them.

If, by some unusual reason, two distinct sessions of the SAME model audited the same Charter and the operator wants both, append a numeric suffix manually: `report-<slug>-2.md`.

The report frontmatter MUST conform to `audit-output.schema.v0.json`:

```yaml
---
audit_role: auditor
auditor: <self-model-id>          # e.g., claude-sonnet-4-6
charter_id: <CHARTER-ID>
git_range: "<range from prompt>"
prompt_used: audit-prompt.md
audited_at: <YYYY-MM-DD>
findings_total: <N>
findings_by_category:
  hallucination: <N>
  implementation_gap: <N>
  real_debt: <N>
  false_positive: <N>
evidence_citations: <N>           # how many path:line citations the body contains
audit_quality: high | medium | low
---

# (body following the format declared in the prompt's "Formato de salida" section)
```

### 6. Notify the operator — with the wait warning

After writing the report, print this message verbatim (substituting `<CHARTER-ID>`, `<slug>`, and the finding count):

```
Audit complete for <CHARTER-ID> (this auditor: <self-model-id>).

  Report: .straymark/audits/<CHARTER-ID>/report-<slug>.md
  Findings: <N> total (<by-category breakdown>)

IMPORTANT: do NOT return to the main agent for /straymark-audit-review yet
unless ALL audits you commissioned have completed.

If you opened other auditor CLIs (gemini-cli, copilot-cli, codex-cli, ...)
and have not yet seen their /straymark-audit-execute finish, wait for them.
Invoking /straymark-audit-review with incomplete reports produces a partial
consolidated analysis that you will have to discard or re-run — costing
you the audit cycle.

When and only when ALL audits you commissioned are complete, return to
your main IDE and run:

    /straymark-audit-review <CHARTER-ID>
```

This wait warning matters: an operator with three CLIs open in parallel can be tempted to invoke review as soon as the first audit finishes. The review skill iterates whatever reports are present at the time it runs; it cannot wait for additional reports to arrive.

## Notes

- **No HTTP API calls.** This skill runs inside an auditor CLI that the operator chose; that CLI handles all model invocation, API keys, and rate limits. StrayMark orchestrates prompt resolution and report shape — nothing else.
- **Re-runs**: if you invoke this skill on a Charter whose report from this model already exists, the existing one is overwritten. The previous report is lost — if you wanted to keep it, copy it manually before re-running.
- **Multiple Charters in the same CLI session**: invoke the skill once per Charter. Reports do not collide because the filename is keyed on Charter id + model slug.
- **Heterogeneity inter-family**: the skill does not enforce that the operator uses different model families across the N audit-execute invocations. The recommendation is in the audit prompt itself and in `AGENT-RULES.md` §12; trust the operator.
