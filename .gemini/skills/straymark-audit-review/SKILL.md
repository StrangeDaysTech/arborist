---
name: straymark-audit-review
description: Consolidate N external auditor reports into a critical review document with verdicts, remediation plan, and auditor ratings. Then merge the external_audit YAML block into the Charter telemetry. Counterpart of /straymark-audit-prompt and /straymark-audit-execute.
---

# StrayMark Audit Review Skill

Critically evaluate the N external auditor reports for a Charter, cross-reference each finding against the actual source code, and produce a consolidated `review.md` with verdicts, a prioritized remediation plan, and per-auditor ratings. Then merge the `external_audit` YAML block into the Charter telemetry.

This is the third and final step of the v1 audit cycle, and it is where the substance lives — the calibrator role (definitional reconciliation across heterogeneous auditor verdicts) is now performed by the main agent inline, replacing the v0 paste-based calibrator-reconciler prompt.

## When to invoke

After the operator has commissioned the audits in N auditor-side CLIs (each running `/straymark-audit-execute <CHARTER-ID>`) and **all of them have completed**. The operator returns to the main IDE and invokes:

```
/straymark-audit-review <CHARTER-ID>
```

If only some audits have completed, **do not proceed** — invoking the skill with incomplete reports produces a partial consolidated analysis. Verify with the operator that all the CLIs they opened have finished writing their reports under `.straymark/audits/<CHARTER-ID>/report-*.md`.

## Instructions

### 1. Resolve the Charter and verify report set

Argument: a Charter identifier.

```bash
ls -la .straymark/audits/<CHARTER-ID>/
```

Confirm:

- `audit-prompt.md` exists (the prepared prompt the auditors read).
- `report-*.md` files exist (one per auditor that completed). At least 2 reports are recommended for cross-family heterogeneity to deliver signal; warn (do not block) if only 1 report is present.
- Match each `report-<slug>.md` filename against the model identifier in its frontmatter (`auditor:` field) to confirm consistency. Discrepancies are surfaced to the operator.

If only the prompt is present and no reports exist, instruct the operator to run `/straymark-audit-execute` in the auditor CLIs first.

### 2. Read all auditor reports + extract finding master list

For each `.straymark/audits/<CHARTER-ID>/report-*.md`:

- Validate the frontmatter against `audit-output.schema.v0.json` (the CLI does this in step 6, but a soft check here gives an earlier and more readable error if any report is malformed).
- Extract: model identifier, total findings, findings by category, evidence_citations count, audit_quality.
- Parse the body for finding entries (typically `## Findings` → `### F1 — title — category` blocks with `Where:`, `What I observed:`, `Why I'm flagging it:`).

Build a **master finding list** — every unique claim across all auditors, deduplicated when two auditors clearly describe the same thing.

### 3. Verify every finding against actual code

This is the substantive step. For EACH finding in the master list:

**Launch Explore agents in parallel** (up to 3 at a time) to verify findings. Group findings by the file they reference and send related findings to the same agent.

For each finding, the verification answers four questions:

1. **Does the code actually have this problem?** Read the cited `path:line`. Is the claim accurate?
2. **Is it in scope?** Does the finding affect a task or file declared in the Charter? If it's outside the Charter's scope, classify as `MISATTRIBUTED`.
3. **What's the real severity given the CURRENT configuration?** Check active driver, feature flags, build tags, DB role, deployment scope (the calibration discipline from the audit prompt). The Etapa 12 example in the prompt is a real case of inflation that careful calibration catches.
4. **Is it a duplicate?** Does another auditor report the same finding differently?

#### Verdict classification

Each finding gets one of these verdicts:

- **VALID** — Real problem, in scope, correctly described, correctly severized.
- **PARTIALLY VALID** — Real observation but wrong severity, missing nuance, or triggers only under a config not active in main. Include the reclassification.
- **MISATTRIBUTED** — Real observation but belongs to a different Charter or scope unit.
- **FALSE POSITIVE** — Claim is factually incorrect (file exists, code works as expected, the assumed driver isn't active, etc.).
- **DUPLICATE** — Same finding reported by another auditor (reference the original).

#### Severity reclassification

When the auditor's severity does not match the active configuration:

- **Inflation** (auditor says Critical/High, code confirms only conditional): downgrade to Medium with explicit note on what config would activate it, OR move to "post-Charter / no bloqueante" if the trigger is a component not yet implemented.
- **Deflation** (auditor says Low/None, code shows a real trigger in current config): upgrade with evidence from the code path that activates it.
- **Correct inflation** (auditor says Critical and config confirms it): keep.

An auditor that consistently inflates or deflates loses points in the auditor rating (step 5).

### 4. Identify findings the auditors missed

Based on your code exploration, check whether there are problems the auditors did NOT find. Focus on:

- Security: SQL queries missing ownership filters, missing transaction boundaries, secrets in logs.
- Logic: input parameters ignored, dead code paths, unreachable branches.
- Consistency: naming mismatches between layers (model vs handler vs API).

Mark these as "Missed by all auditors" in the remediation plan.

### 5. Build the consolidated review.md

Write the consolidated analysis to `.straymark/audits/<CHARTER-ID>/review.md` with this structure (six sections, lifted from Sentinel's pre-StrayMark audit-review skill):

```markdown
---
audit_role: calibrator-reconciler
calibrator: <self-model-id>
charter_id: <CHARTER-ID>
git_range: "<range from prompt>"
prompt_used: ../audit-prompt.md
calibrated_at: <today YYYY-MM-DD>
auditors_reconciled:
  - report-<auditor-1-slug>.md
  - report-<auditor-2-slug>.md
  - ...
findings_consolidated: <N>
findings_by_status:
  agreed: <N>
  disputed: <N>
  unique_<auditor-1-slug>: <N>
  unique_<auditor-2-slug>: <N>
  rejected: <N>
---

# Consolidated audit review — <CHARTER-ID>

**Reviewer:** <self-model-id>
**Date:** <YYYY-MM-DD>
**Confidence:** [High | Medium]

## 1. Executive summary

[2-3 paragraphs. Total findings count, scope confusion if any, most critical bug,
overall verdict on the Charter's implementation.]

## 2. Scope definition

[Table of Charter tasks, the closing criterion, and what IS vs IS NOT in scope
of this Charter. The auditors' findings are evaluated against THIS scope.]

## 3. Per-auditor evaluation

### 3.1 <auditor-1-slug> (model: <auditor-1-model-id>)

| # | Finding | Reported severity | Verdict | Justification |
|---|---------|-------------------|---------|---------------|

**Summary:** [2-3 sentences on this auditor's overall performance.]

### 3.2 <auditor-2-slug> ... (same shape)

## 4. Remediation plan — VALID and PARTIALLY VALID findings

### P0 — Security
- **Files:** `path:line`
- **Problem:** [description with code evidence]
- **Remediation:** [specific approach]
- **Complexity:** [Low / Medium / High]
- **Detected by:** [auditor slug(s), or "Missed by all auditors" if you found it]

### P1 — Integrity
[Same shape]

### P2 — Consistency
[Same shape]

### P3 — Robustness
[Same shape]

### P4 — Documentation
[Same shape]

## 5. Discarded findings — misattributions and false positives

| Finding | Type | Charter / area | Auditor |
|---------|------|----------------|---------|

## 6. Auditor ratings

Score each auditor 1-10 across four criteria with weights:

| Auditor | Scope precision (25%) | Technical depth (25%) | Bug detection (30%) | False positive rate (20%) | **Overall** |
|---------|:-:|:-:|:-:|:-:|:-:|
| <auditor-1-slug> | /10 | /10 | /10 | /10 | **/10** |
| ...

### Justifications

**<auditor-1-slug> — <score>/10**: [2-3 sentences on what this auditor did well and where they slipped.]

## 7. Conclusion

[State of the Charter — clean / partial / deviated. Critical findings count.
Key remediation items the operator should address before close.
Recommended next step.]
```

### 6. Validate + emit external_audit YAML

Run the CLI's merge step to validate all reports against the schema and emit the YAML block (combined with `--merge-into` if the operator's Charter telemetry already exists):

**Branch A — telemetry exists** (operator already ran `straymark charter close` for this Charter, perhaps without audit, and is now adding audit findings retroactively):

```bash
straymark charter audit <CHARTER-ID> --merge-reports \
  --merge-into .straymark/charters/<CHARTER-ID>.telemetry.yaml
```

The CLI appends the `external_audit:` array to the telemetry YAML. The CLI v1 deliberately rejects re-audit (file already has `external_audit:`) — if that fires, surface the message to the operator. Manual append is the fallback.

**Branch B — telemetry does NOT exist** (the typical case: operator audits BEFORE closing):

```bash
straymark charter audit <CHARTER-ID> --merge-reports
```

The YAML block prints to stdout. Capture it and write it to `.straymark/audits/<CHARTER-ID>/external-audit-pending.yaml` so the operator can paste it into the telemetry when running `straymark charter close <CHARTER-ID>`. Tell the operator: "When you run `straymark charter close <CHARTER-ID>`, paste the `external_audit:` block from `.straymark/audits/<CHARTER-ID>/external-audit-pending.yaml` when prompted."

### 7. Notify the operator with a summary

```
Consolidated audit review complete for <CHARTER-ID>.

  Auditors reconciled:    <count>
  Findings reported:      <total>
  Findings VALID:         <valid> (<percent>%)
  False positives:        <fp>
  Misattributions:        <misattr>
  Missed by all (you):    <missed>

  Remediation plan:
    P0 (Security):        <count or 'none'>
    P1 (Integrity):       <count or 'none'>
    P2-P4 (Other):        <count or 'none'>

  Auditor ratings:
    <auditor-1-slug>:     <score>/10
    <auditor-2-slug>:     <score>/10
    ...

  Documents written:
    .straymark/audits/<CHARTER-ID>/review.md
    [either: .straymark/charters/<CHARTER-ID>.telemetry.yaml (merged)]
    [or:     .straymark/audits/<CHARTER-ID>/external-audit-pending.yaml]

Run `git diff` to review the changes before commit.
```

## Notes

- **The calibrator role is in-conversation, not in a paste-based prompt.** v0 had a `calibrator-reconciler.md` template that the operator pasted into a separate model. v1 eliminates that round-trip — the main agent (this conversation) has filesystem access and can verify findings against code, which is what makes the consolidated review substantive instead of mechanical.
- **Heterogeneity inter-family is for the auditor pair, not the calibrator.** The calibrator (this skill, running in the main agent) may be of any family — even the same family as one of the auditors — because its task is definitional (apply the schema to already-produced verdicts), not discovery.
- **The review.md is the human-readable delivery; the YAML block is the machine-readable input to telemetry.** Both coexist by design. The operator reads `review.md`; `straymark metrics` and the Phase 2 telemetry aggregation read the YAML.
- **Re-running the skill** overwrites `review.md`. If the operator wants to keep the previous review (e.g., after a re-audit cycle), instruct them to copy it manually before re-running.
- **No HTTP API calls.** The skill runs the CLI's `--merge-reports` (which validates schemas and emits YAML), invokes Explore agents internally for code verification (which run in the main agent's context, not external APIs), and writes markdown. StrayMark does not invoke LLM APIs at any point.

## Credit

The six-section structure of `review.md` and the verdict vocabulary (VALID / PARTIALLY VALID / MISATTRIBUTED / FALSE POSITIVE / DUPLICATE) are lifted from the `audit-review/SKILL.md` skill mature pre-StrayMark in Sentinel, contributed via [issue #102](https://github.com/StrangeDaysTech/straymark/issues/102) by José Villaseñor Montfort (StrangeDaysTech). The four-criterion weighted auditor rating (Scope precision 25% / Technical depth 25% / Bug detection 30% / False positive rate 20%) is the same. Project-specific hardcodes parameterized.
