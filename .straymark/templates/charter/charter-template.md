---
charter_id: CHARTER-NN
status: declared
effort_estimate: M
trigger: "[1-line: what concrete signal — observable event, declared decision, metric threshold, or infrastructure milestone — justifies executing this Charter now]"
# Exactly one of the following two should be set when the Charter has a known origin.
# Both absent is valid for a Charter scaffolded without an explicit origin (must be
# filled before status moves to in-progress).
# originating_ailogs: [AILOG-YYYY-MM-DD-NNN]
# originating_spec: specs/001-feature/spec.md
---

# Charter: [BRIEF TITLE]

> **Status (mirrored from frontmatter — source of truth is above):** declared. Effort: [XS | S | M | L] (~[N] min).
>
> **Origin:** [human-readable summary; the machine-readable form is `originating_ailogs` or `originating_spec` in frontmatter].

<!-- Charter template — 6 format conventions distilled from the Sentinel /plan-audit
     experiment (6 cycles, 2026-04-28). See the comment block at the end of this file
     for each convention with its empirical justification, and straymark-cli-roadmap.md §3
     plus straymark-thesis-validation.md §3-§5 for the source evidence. -->

## Context

[1-2 paragraphs. What problem this Charter solves, what operational or regulatory
motivation makes it urgent, what has been attempted before (if anything). Cite the
originating AILOGs here too if it helps the reader understand why the work was deferred.]

## Scope

**In scope:**

[Numbered list of concrete changes to apply. Each item must be verifiable: "X file
gains Y method", "Z test covers W case". Avoid vague items like "improve performance"
— those are objectives, not scope.]

1. [Item 1]
2. [Item 2]
3. [...]

**Out of scope:**

[List of things explicitly NOT covered by this Charter. Important so external auditors
do not classify them as gaps. Ideally cite the Charter or initiative where they belong.]

- [Item 1] — deferred to [Charter/initiative].
- [Item 2] — out of scope because [reason].

## Files to modify

| File | Change |
|---|---|
| `path/to/file.ext` | [Concrete description of the change] |
| `...` | `...` |
| `.straymark/07-ai-audit/agent-logs/AILOG-...md` | New, `risk_level: [low|medium|high]` |

## Verification

### Local checks

Commands executable literal in a clean shell — include explicit setup of dependencies.
Any failure of these commands indicates real debt.

```bash
# Build & test (adapt to your stack)
<build-command>
<test-command>

# Security/vulnerability scanners with explicit setup
# (Pattern validated in Sentinel PLAN-01..05: implicit PATH lookups generated
# false-positive 'real_debt' classifications from external auditors.)
<install-and-run-security-scanner>
<install-and-run-vulnerability-scanner>

# Other local commands here. If they require integration infra, document explicitly:
<integration-test-command>
```

### Production smoke (after deploy)

Commands that **only apply after deploy to a real environment**. NOT executable in a
clean shell without infrastructure. External auditors should skip this section —
failures here are NOT `real_debt`.

```bash
# Example: verify a new endpoint is live in production.
TOKEN="$(<auth-cli> print-identity-token)"
curl -X PUT "https://${SERVICE_HOST}/api/v1/.../..." \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"...": "..."}'

# Example: SQL query in production DB to verify event persistence.
<production-db-cli> connect <service-db> -- \
  -c "SELECT context FROM audit_records WHERE action='...' \
      ORDER BY timestamp DESC LIMIT 1"
```

## Risks

[List of risks R1, R2, ... that the implementation commits to mitigate. Each with its
mitigation documented. Convention: if a NEW risk emerges during execution that was not
anticipated, document it in the AILOG under `## Risk` as `R<N+1> (new, not in Charter)`
— Gemini and other external auditors validate these cross-document.

Each mitigation should specify: (a) concrete trigger or threshold (no "eventually"),
(b) action committed, (c) what happens if the mitigation itself fails, (d) where
follow-up insights are captured if the risk surfaces lessons for a later cycle.]

- **R1 — [risk description]**: [probability/severity].
  Mitigation: [concrete action taken in implementation].
- **R2 — ...**: ...
- [...]

## Tasks

1. Sync main, branch `<branch-prefix>/[slug]`.
2. [Implementation task 1].
3. [Implementation task 2].
4. [...]
5. AILOG (`risk_level: [low|medium|high]`, `review_required: [true|false]`).
6. **For multi-batch execution (3+ batches or >1 day)**: maintain a
   `## Batch Ledger` in the AILOG. After each batch commit, run
   `straymark charter batch-complete <CHARTER-ID> <N>` to update the
   ledger before pushing. The drift gate at close will reject any
   `### Batch N` left as `(pending)`. Skip this step for single-batch
   Charters — `## Actions Performed` in the AILOG suffices.
   *Note*: if audit findings arrive **after** `status: closed` and warrant
   a bounded code-level fix, see STRAYMARK.md §15.B (post-close Batch N.4
   amendment) and `straymark charter amend` instead of opening a new Charter.
7. Local verification passes clean.
8. **Auto-checklist drift** (when Phase 2 of the CLI roadmap ships):
   `straymark charter drift CHARTER-NN <range>` to detect drifts between declared
   and modified files **before** commit. If it reports omissions, complete the work
   or document in the AILOG under `## Risk` as `R<N+1> (new, not in Charter)`. If it
   reports scope expansion, document in the AILOG the reason (mock updates, generated
   files, drift fix pre-existing, etc.). Until Phase 2 ships, run Sentinel's
   `check-plan-drift.sh` manually for the same effect.
9. Commit + push + open PR.

## Charter Closure

When closing this Charter:

1. **Atomic update (format v4)**: if the drift check (Tasks #7) reported any drift
   not already captured in the AILOG, edit `## Files to modify` and/or add a
   `## Closing notes` block in **this same commit/PR**, before submitting. Do not
   defer to a post-merge housekeeping PR. The atomic-update pattern is the canonical
   way to keep the Charter coherent with execution; deferring it leaves the Charter
   stale and confuses future readers (PLAN-07 of Sentinel demonstrated the failure
   mode that this step prevents).

2. **Post-merge drift check** (automated when Phase 2 ships + manual review):
   - Run `straymark charter drift CHARTER-NN origin/main..HEAD` (Phase 2) or the
     equivalent Sentinel script, and validate the output is clean or that all
     drifts are documented in the AILOG.
   - This catches the rare case where drift is introduced post-merge (squash
     mangling, admin amendments, etc.) and the atomic step in #1 could not apply.

3. **Move the row** in `.straymark/charters/README.md` to `## Closed` and reference the PR.

4. **Status frontmatter** moves from `in-progress` to `closed` (and optionally
   `closed_at: YYYY-MM-DD` is added — the schema allows arbitrary additional fields).

5. **Do not delete** this file — the planning history matters as much as the AILOG
   of execution.

## Closing notes

> Add this section ONLY when Tasks #7 drift check reported drift that the
> implementer chose to remediate atomically (rather than redoing the implementation
> to match `## Files to modify` exactly). Each bullet: what changed vs declaration,
> why, reference to the AILOG that documented the decision. Omit the section
> entirely if no drift was detected — empty `## Closing notes` is noise.
>
> Historical examples in Sentinel: PLAN-05 (`docs/plans/05-per-service-anomaly-thresholds.md`)
> §Notas de cierre — files removed because the implementation chose a different
> injection point; PLAN-07 (`docs/plans/07-fix-distribution-aligner.md`) §Notas de
> cierre — file removed because the live test was agnostic to the change. Both
> demonstrate the pattern in production usage.

- `[path/file-from-declaration.ext]` [removed | relocated to X | repurposed]:
  [1-2 lines explaining what the implementation did instead and why the original
  declaration is no longer accurate]. Reference: AILOG-YYYY-MM-DD-NNN §[section].

---

<!--
Format conventions — 6 patterns embedded in this template, distilled from the
6-cycle Sentinel /plan-audit experiment (2026-04-28). The provenance is part of the
historical record (in StrayMark terms these are simply "the conventions", not "v2 +
v3 addition" — the partition was Sentinel's iteration log, not structural).

1. Verification splits into `### Local checks` (executable literal in clean shell)
   and `### Production smoke (after deploy)` (not executable without infrastructure).
   Reason: external auditors classified prod-only command failures as `real_debt` —
   avoidable noise. Validated 5/5 cycles after the convention was named.

2. Effort is measured in TIME (XS/S/M/L), not in `~N lines`. Reason: time met the
   estimate (1.0x) in 4/5 cycles; line count drifted 1.0x → 3.1x → 8.1x due to
   AILOG/tests/mocks. Lines are not predictive of cognitive effort.

3. Modifiers like `(optional)` or `(after deploy)` live as structured sub-sections,
   never as inline parenthetical comments. Reason: the Gemini auditor consistently
   ignored parenthetical modifiers and classified marked-optional commands as
   `real_debt`. Validated 2/2 cycles where the pattern applied.

4. R<N> risks are enumerated in the Charter; new risks emergent during execution are
   documented in the AILOG as `R<N+1> (new, not in Charter)`. Reason: cross-validable
   signal by external auditors — they triangulate Charter declarations against AILOG
   emergence. Validated 4/4 cycles where new risks emerged.

5. The `## Charter Closure` section requires the implementer to update the Charter
   doc atomically (same PR as the fix) when drift is detected by Tasks #7, not in
   a separate post-merge housekeeping PR. The `## Closing notes` block is the
   canonical place to document each atomic edit (what changed vs `## Files to
   modify`, why, AILOG reference). Reason: PLAN-07 of Sentinel demonstrated that
   without an explicit atomic-update step, drift remediation can lag the main PR
   by days, leaving the Charter stale and confusing future readers — AIDEC of
   Sentinel 2026-05-02-001 formalized the gap and proposed format v4 (this template
   embodies it).

6. Auto-checklist drift (`straymark charter drift`, Phase 2 of the CLI roadmap;
   Sentinel had `scripts/check-plan-drift.sh`) runs in pre-commit (Tasks #7) and at
   Charter closure. Detects OMISSION drifts (file declared, not touched) and SCOPE
   EXPANSION drifts (file touched, not declared). Reason: external auditors caught
   implementation-gap and hallucination drifts that the implementer did not document
   in their AILOG. The script catches the same drifts BEFORE commit, separating
   "known and documented" from "forgotten". Zero false positives on 2/2 empirical
   tests against the canonical Sentinel Plans.
-->
