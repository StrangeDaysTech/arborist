# Charter Chain Evolution — StrayMark

> Two complementary patterns for keeping a multi-Charter module honest: refreshing SpecKit artifacts before a Charter is declared, and amending a closed Charter when audit findings arrive after `status: closed`.

**Languages**: English | [Español](i18n/es/CHARTER-CHAIN-EVOLUTION.md) | [简体中文](i18n/zh-CN/CHARTER-CHAIN-EVOLUTION.md)

---

## Status

**v0 — proven in N=1 domain** (`StrangeDaysTech/sentinel` CHARTER-18, 2026-05-15, Issue #156).

Both patterns are conventions documented here as canonical framework guidance. The CLI ships read-only and scaffolding helpers (`straymark charter refresh-suggest`, `straymark charter amend`); the patterns themselves are operator-driven. Either pattern may evolve once a second adopter validates them — until then, the N=1-domain caveat applies (Principle #12).

---

## Why this document exists

StrayMark's Charter pattern (`STRAYMARK.md` §15) assumes a single Charter is the bounded unit of work. That works for isolated Charters. It also works for the *first* Charter in a chain. But when a module accumulates many user-story Charters over months, two failure modes surface that the per-Charter pattern alone does not address:

1. **Spec drift at the chain level** — the SpecKit artifacts (`plan.md`, `data-model.md`, `contracts/*`, `quickstart.md`, `research.md`) were authored against the framework version and reality of the module at chain start. After 3+ Charters, accumulated learnings (reusable patterns extracted, code gaps found, framework conventions evolved, operator decisions ratified) have drifted the spec away from the implementation. Going directly from Charter-N close into Charter-(N+1) declare causes systematic mid-flight scope expansions and emergent `R<N+1> (new, not in Charter)` entries.
2. **Audit findings at the cycle level** — external audit cycles run post-close (auditors execute asynchronously after the close ceremony). Critical or High findings can arrive after the Charter has been marked `status: closed`. The framework's options are then: (a) open a new Charter for remediation (heavy — full declare + Tasks + ceremony for ~5 file edits), or (b) leave the findings in `review.md` and lose the "atomic with the Charter" property.

Pattern 1 below addresses (1). Pattern 2 addresses (2). The two compose — a Charter that *received* Pattern 1 is more likely to *avoid* Pattern 2, because the refresh absorbs the pre-execution risk that the audit would surface post-close. They are complementary, not substitutable.

---

## Pattern 1 — Pre-declare SpecKit refresh

### When this pattern applies

Adopt this pattern when **all** of these hold for a SpecKit-driven module:

- The module has **3 or more closed Charters** (chain length ≥ 3).
- The rolling mean of `charter_telemetry.agent_quality.r_n_plus_one_emergent_count` across the last 3 closed Charters is **> 6**.
- No refresh PR has landed in the SpecKit artifacts since the chain's last branch point.

Run `straymark charter refresh-suggest <module>` to evaluate the heuristic against your `.telemetry.yaml` history. The CLI reads the last closed Charters of the named module and prints a recommendation; it does not mutate anything.

Below the threshold, the per-Charter pattern alone is sufficient — adopting the refresh too early adds a PR's worth of overhead without payoff.

### Shape

A **dedicated refresh PR** lands between Charter-N close and Charter-(N+1) declare. It touches only the **non-locked sections** of the SpecKit artifacts:

- `specs/<module>/plan.md` — phase plans, dependency notes, sequencing.
- `specs/<module>/data-model.md` — entities, fields, conventions.
- `specs/<module>/contracts/*.md` — interface contracts, request/response shapes.
- `specs/<module>/quickstart.md` — runnable scenarios.
- `specs/<module>/research.md` — accumulated knowledge (see "Categorized learnings table" below).

`research.md` carries the load-bearing artifact: a **categorized learnings table** consolidating what the chain learned. Minimum buckets:

| Bucket | What goes here |
|---|---|
| Reusable patterns | Idioms / utilities / wrappers that emerged across Charters and should be inherited going forward (e.g. `withRLS` wrapper, brand-cache LRU, dedup table pattern). |
| Code gaps | Identified-but-unfixed work the chain discovered but did not close (e.g. unwired tables, stub implementations, missing columns). Each gap is a `Gn` entry with description + owning Charter (current or future). |
| Discipline patterns | Process learnings the chain ratified (e.g. cross-family audit pair, batch-complete discipline, per-batch close cadence). |
| Empirical corrections | Places where the spec drifted from the implementation. `EC1...ECn` entries: spec said X, reality is Y, reconciliation chosen. |

Optional **operator decisions (Dn)** are ratified pre-declare with: decision, alternatives considered, chosen path, rationale. Subsequent Charters inherit Dn as contracts.

### Mechanics

1. **Refresh PR** before the next Charter declare. Optional AIDEC documenting the refresh decision + alternatives considered. The PR title should make the scope explicit (e.g. `spec(<module>): US<n> plan refresh — LOCKED-aware Phase 7+8 redesign`).
2. **Categorized learnings table** in `research.md` with the four buckets above. Each entry has a stable id (Pn / Gn / DPn / ECn) so subsequent Charters can cite by id.
3. **Operator decisions (Dn)** if applicable — listed explicitly with alternatives + chosen path + rationale.
4. **Next Charter's `## Context` section** cites each pattern, correction, and decision by id. Charter scope is grounded in refreshed reality, not in the chain-start spec.

### Telemetry

Populate `charter_telemetry.pre_declare_refresh:` in the *next* Charter's telemetry (the one that consumed the refresh, not the refresh PR itself):

```yaml
pre_declare_refresh:
  enabled: true
  refresh_pr: "owner/repo#76"
  refresh_aidec: "AIDEC-YYYY-MM-DD-NNN-speckit-refresh"
  reusable_patterns_integrated: 7
  code_gaps_integrated: 4
  discipline_patterns_integrated: 3
  empirical_corrections_integrated: 15
  operator_decisions_ratified: 3
```

Omit the block entirely if no refresh occurred — absence means "pattern not used".

### Why this works (empirical)

Sentinel CHARTER-18 was the first Charter in a 7-Charter chain to close cleanly without a mid-flight remediation Charter. `estimation_drift_factor: 1.0`, `pre_work.items_discovered_during_planning: 0`, `overall_satisfaction: 5/5`. Operator's drift reason statement: *"the SpecKit refresh from PR #76 ... eliminated most ambiguity that drove drift in prior Charters. No mid-flight remediation Charter required — the EC1..EC15 empirical-corrections inventory in research.md absorbed what would have been pre-execution risk into in-execution awareness."*

---

## Pattern 2 — Post-close audit-driven amendment (Batch N.4)

### When this pattern applies

Adopt this pattern when **all** of these hold after a Charter has been marked `status: closed`:

- One or more external audit findings emerge in the post-close `review.md` graded **Critical** or **High**.
- The Charter's `closure_criterion` is materially unmet by the un-remediated findings (i.e. shipping as-is would invalidate the close).
- The fix surface fits in **one cohesive PR** (~< 25 files, no architectural reopen — no new abstractions, no migrations, no API breaks).

If the fix surface is larger or architectural, open a new Charter instead. The amendment pattern exists for the bounded case; it is not a Charter-evasion mechanism.

### Shape

The amendment rides **the same execute branch** as the original Charter (the branch is still mergeable to `main`; the amendment commit lands on top). A **new AILOG** documents the amendment — not an edit of the original AILOG.

```
charter-<N>-execute branch
├── (original commits — Charter execute work)
├── commit X: charter close (status: closed, telemetry.yaml written)
└── commit Y: charter-<N>(batch-7.4): audit-driven remediation — <short summary>
    ↑
    AILOG-YYYY-MM-DD-MMM (NEW) documents this commit
    AILOG-YYYY-MM-DD-NNN (ORIGINAL) gets a `## Historical correction` subsection
                                    pointing forward to AILOG-...-MMM
```

### Mechanics

1. **Same execute branch** — do not branch off `main`. The original Charter's execute branch is still the unit; the amendment commit rides along.
2. **New AILOG** under `.straymark/07-ai-audit/agent-logs/` documents the amendment. Convention: `risk_level: high` and `review_required: true`. The new AILOG carries an `amends:` field pointing back to the original AILOG id.
3. **Historical correction in the original AILOG** — append a `## Historical correction (YYYY-MM-DD)` subsection at the end of the original AILOG with the forward pointer to the new AILOG. Audit decisions are distinct from execute decisions; the original's body remains intact as the historical record.
4. **PR commentary** — if the execute PR has not yet merged, add the amendment commit to it and update the PR description with a "Batch N.4 amendment" subsection listing the closed findings. If the PR already merged, open a follow-up PR that references the original PR and the AILOG.
5. **Telemetry** — populate `charter_telemetry.post_close_amendment:` (see below). Use `straymark charter audit <id> --merge-reports --merge-into <telemetry-yaml>` to merge external audit findings into the same file; the CLI tolerates `external_audit: []` placeholder rewrites in v0.2+.

`straymark charter amend <id>` scaffolds steps 2, 3, and 5 (creates the new AILOG stub, edits the original AILOG with the Historical correction subsection, prints the YAML block). It does not touch git — the operator decides when to commit.

### Telemetry

Populate `charter_telemetry.post_close_amendment:` in the Charter's `.telemetry.yaml`:

```yaml
post_close_amendment:
  applied: true
  trigger: "external_audit"           # external_audit | production_incident | deferred_implementation
  ailog_id: "AILOG-YYYY-MM-DD-MMM"    # the NEW AILOG, not the original
  findings_closed: 5
  files_modified: 19
  effort_hours: 6.0
```

Omit the block entirely if no amendment occurred.

### Why this works (empirical)

Sentinel CHARTER-18 closed 2026-05-15 with `external-audit-pending.yaml`. Audit reports landed 2026-05-15..05-17. Five findings (4 Critical/High from `gpt-5.3-codex`, 1 Critical from `gemini-2.5-pro`, 1 Medium found by the calibrator) were code-level fixes — DI wiring, retry header parsing, multi-tenant filter, timeout default. The Batch 7.4 amendment closed all five in one cohesive commit (19 files, +2257/-106 lines). A new Charter would have created multi-week governance overhead for ~6h of focused engineering.

---

## Cross-pattern composition

The two patterns operate at different levels of the chain and compose:

| Pattern | Level | Frequency | Absorbs |
|---|---|---|---|
| Pre-declare SpecKit refresh | Chain / module | Once per 3+ Charters | Spec-level drift (architectural assumptions, table naming, framework version evolution) |
| Post-close audit-driven amendment | Cycle / Charter | Per Charter when triggered | Runtime-level drift (DI wiring, retry semantics, multi-tenant filters) |

A Charter that *received* Pattern 1 is more likely to *avoid* Pattern 2 — the refresh absorbs pre-execution risk that would otherwise surface as post-close findings. But CHARTER-18 needed *both* — the refresh handled spec-level drift; the amendment handled runtime-level drift the refresh did not reach into. Encourage Pattern 1 at the chain level; tolerate Pattern 2 at the cycle level.

---

## Authority / acceptance flow for upstreaming new patterns

This document is itself the output of the acceptance flow Sentinel walked through for these two patterns (Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)). The canonical flow for upstreaming a new Charter-chain pattern is:

1. **Adopter-local RFC** lives at `.straymark/06-evolution/<name>-rfc.md` in the adopter's own tree. The adopter ships the pattern there first — N=1 evidence is necessary but not sufficient.
2. **Upstream Issue** in `StrangeDaysTech/straymark` mirroring the local RFC body, with telemetry citations and PR links.
3. **Upstream acceptance** lands as: (a) a doc here in `00-governance/` describing the pattern canonically, (b) telemetry schema additions (opt-in), (c) optional CLI scaffolding for the operator-facing mechanics. The N=1-domain caveat carries forward to v1 stabilization.
4. **Second-domain validation** before the pattern's schema fields graduate from optional to recommended.

`06-evolution/` is the canonical adopter-local home for in-flight RFCs. Once accepted upstream, the canonical home is `00-governance/<NAME>.md` — the convention this document instantiates.

---

## Open questions

- **Threshold tuning** — the rolling-mean threshold of 6 for `r_n_plus_one_emergent_count` is Sentinel-derived. A second domain may move it. The `straymark charter refresh-suggest` CLI exposes `--threshold N` for adopter calibration.
- **Module heuristic** — `refresh-suggest <module>` currently matches `<module>` against the Charter title and slug. SpecKit-conventional modules (`specs/<NNN>-<module>/`) could provide a stricter binding via the Charter's `originating_spec` field in a future fw bump.
- **Amendment frequency cap** — Pattern 2 is bounded by "one cohesive PR". A Charter receiving two or more amendment commits over time should be re-evaluated as a sign that the original close was premature.

---

## Related

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) — the meta-pattern of which Pattern 1 and Pattern 2 are applications (formal cross-referencing + cultural permission to surface).
- [STRAYMARK.md §15](../../STRAYMARK.md) — Charter lifecycle and the per-Charter pattern this document extends.
- [SPECKIT-CHARTER-BRIDGE.md](SPECKIT-CHARTER-BRIDGE.md) — how SpecKit artifacts map to Charters; Pattern 1 lives on this seam.
- [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) — sibling pattern for accumulated `§Follow-ups` across many AILOGs.
- [`.straymark/schemas/charter-telemetry.schema.v0.json`](../schemas/charter-telemetry.schema.v0.json) — `pre_declare_refresh` and `post_close_amendment` are defined here.

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
