# Follow-ups Backlog Pattern - StrayMark

> Reproducible convention for managing accumulated `§Follow-ups` and `R<N> (new, not in Charter)` entries across many AILOGs and Charters.

**Languages**: English | [Español](i18n/es/FOLLOW-UPS-BACKLOG-PATTERN.md) | [简体中文](i18n/zh-CN/FOLLOW-UPS-BACKLOG-PATTERN.md)

---

## Status

**v0 — proven in N=1 domain** (`StrangeDaysTech/sentinel` CHARTER-12, 2026-05-06).

This is a **convention**, not a CLI feature. Adopters reproduce it locally with markdown + a portable bash script. The pattern may evolve into a `straymark followups` subcommand once a second adopter validates it (see [Open questions](#open-questions)).

---

## When this pattern applies

StrayMark's per-AILOG `§Follow-ups` convention works at write time — when an AILOG is created, the implementer documents what is deferred to subsequent Charters or operational triggers. That works fine until the cumulative list grows past what an operator can scan from memory.

Adopt this pattern when **any** of these conditions hold:

- The project has accumulated **~20 or more AILOGs** with non-trivial `§Follow-ups` sections.
- Operators repeatedly ask agents to "list what's pending across the project" and the answer requires a multi-file scan.
- A "do this when X arrives" follow-up was almost lost because the originating AILOG was never re-read after X arrived.
- A Charter retrospective surfaces follow-ups that should have been classified as `closed` weeks earlier but were never indexed.

Below that volume, the per-AILOG convention alone is sufficient — adopting this pattern early adds maintenance overhead without payoff.

---

## Shape

### Registry file

Single markdown file at the canonical path:

```
.straymark/follow-ups-backlog.md
```

### Frontmatter (YAML)

```yaml
---
last_scan: 2026-05-06
last_scan_range: AILOG-NNNN-NN-NN-NNN..AILOG-NNNN-NN-NN-NNN  # optional — first..last AILOG covered
schema_version: v0
total_open: 0           # count of entries currently `open`
total_promoted: 0       # count of entries currently `promoted` (added in schema v0.1 — see "Promotion to TDE")
total_closed_in_session: 0   # count of `closed` entries since last session (optional, operator-maintained)
total_phase_blocked: 0  # count of `phase-blocked` entries (optional)
buckets:
  - ready
  - time-triggered
  - charter-triggered
  - phase-blocked
  - operational
fully_extracted_ailogs:
  - AILOG-2026-04-11-001
  - AILOG-2026-04-12-001
  # ... one entry per AILOG whose follow-ups have been processed
---
```

The `total_*` counters are **operator-maintained metadata**. The drift script doesn't update them automatically — they live in the header so a session-start glance shows the registry's pulse without scrolling through buckets. `total_promoted` was canonicalized in schema v0.1 (Sentinel adopter empirical signal, fw-4.13.1) to mirror the existing `total_open` / `total_closed_in_session` / `total_phase_blocked` pattern.

The `fully_extracted_ailogs` list is the **load-bearing metadata** for drift detection. Every AILOG whose `§Follow-ups` and `R<N>` entries have been transferred into the registry (or explicitly classified as superseded) belongs in this list. Drift detection compares this list against AILOGs that have follow-up content in the repo.

### Buckets

Five buckets organize entries by trigger type:

- `ready` — actionable now, no dependency on external trigger.
- `time-triggered` — calendar-based trigger (audit cycle, periodic review).
- `charter-triggered` — gated on a future Charter that touches the relevant area.
- `phase-blocked` — gated on a future component or phase that does not yet exist.
- `operational` — manual operator decision or external system action.

### Entry schema

Each entry inside a bucket follows this shape:

```markdown
### FU-NNN — <short description>
- **Origin**: AILOG-NNNN-NN-NN-NNN <pointer to source section>
- **Status**: open | in-progress | closed | superseded | promoted
- **Trigger**: ready | <calendar date> | when <X> | <other>
- **Destination**: <Charter id, "operations", future phase, or TDE-YYYY-MM-DD-NNN>
- **Cost**: <effort estimate>
- **Notes**: <free-form context>
- **Promoted to**: <TDE id, when Status: promoted — see "Promotion to TDE" below>
```

`FU-NNN` is monotonically increasing across the registry's lifetime; do not renumber when entries close.

### Status vocabulary

- `open` — pending, not yet acted on.
- `in-progress` — a Charter has been declared or is executing that addresses this entry.
- `closed` — entry resolved (Charter merged, operational task done, time elapsed and reviewed).
- `superseded` — addressed by other work that did not reference this entry directly.
- `promoted` — the entry was elevated to a TDE document because it met the transversal-debt criteria (see "Promotion to TDE" below). The `Promoted to:` field carries the TDE id.

Closed, superseded, and promoted entries stay in the file (auditable history). Operators may move them to a `## Bucket: closed` section at the bottom for visual decluttering, but they are never deleted.

---

## Promotion to TDE

Some FU entries are not just deferred tasks — they describe **transversal technical debt** that warrants its own governance document (TDE). The criteria for promotion mirror the TDE-vs-`R<N>` disambiguation in `AGENT-RULES.md §3`:

- The entry is *heritage from a prior Charter* (already lived through ≥1 Charter close without remediation).
- The entry *applies to multiple modules or multiple Charters* — the central registry has fragmented it into bullets that share a root cause.
- The entry *requires a dedicated Charter outside the current scope envelope* to remediate.
- The entry *requires human prioritization or assignment* that the periodic operator review cannot decide from the bullet alone (impact × effort matrix, ownership).

When any of these holds, promote the FU entry to a TDE document under `.straymark/06-evolution/technical-debt/`:

1. Create the TDE via `/straymark-new tde` (or `straymark new --type tde`). Fill `impact`, `effort`, `type`, and the body sections from the FU entry's context.
2. Add `promoted_from_followup: FU-NNN` to the TDE frontmatter for traceability.
3. In the FU entry, set `Status: promoted`, set `Destination: TDE-YYYY-MM-DD-NNN`, and add `Promoted to: TDE-YYYY-MM-DD-NNN`. Move the entry to the `## Bucket: closed` section if you maintain one; otherwise leave it in place with the new status.

The FU entry is **not deleted** after promotion — its presence in the registry is the audit trail showing where the TDE came from.

### Two promotion shapes — promotion-of-existing vs retroactive-at-creation

The workflow above covers the **standard case**: an `open` FU entry already exists in the registry and gets elevated to a TDE during periodic review. There is a second case that is equally valid and that emerged empirically from the Sentinel CHARTER-13 retrospective:

- **Promotion of existing entry** — an FU was registered (typically via `--apply`) as `open` weeks or Charters ago, lived through ≥1 Charter close without resolution, and meets the four criteria above. Standard flow.
- **Retroactive promotion at creation** — the debt is recognized as TDE-worthy *during* a retrospective (Charter close ceremony, audit cycle, RFC writeup) and never existed as an `open` FU. The TDE is created first; an FU entry is added to the registry *with `Status: promoted`* from birth, providing the audit trail back from the TDE to the originating context (an `R<N>` in an AILOG, a calibrator finding, a deferred classification).

Both shapes produce the same end state in the registry: an entry with `Status: promoted` and a `Promoted to: TDE-YYYY-MM-DD-NNN` pointer. The difference is whether the entry pre-existed as `open` or was born `promoted`. The drift script treats them identically (it doesn't differentiate by birth status), and adopter analytics counting `total_promoted` get the same number either way.

When in doubt, prefer creating the FU entry — even retroactively — because it cross-references the TDE back to the AILOG / R-number / source context that triggered the recognition. A TDE with `promoted_from_followup: FU-NNN` pointing to an entry that exists in the backlog is more navigable than one pointing to a fictional FU.

### When to promote

- **Periodic review** — when the operator does the manual reclassification pass, promote any entry that has lived through ≥2 Charter closes without resolution and meets the criteria above.
- **Charter close** — when reviewing entries the just-closed Charter resolved, if you find entries that were *not* resolved and meet the criteria above, promote them rather than leaving them as `open`.
- **Pre-Charter declaration** — if you're about to declare a Charter and notice the registry contains entries that this Charter would *partially* address, the un-addressed portion may belong as a TDE rather than as another deferred FU.

The drift script (`scripts/check-followups-drift.sh`) is **not extended** for promotion candidates in v0 — promotion is operator-driven. A future v1 enhancement could flag entries meeting the "lived through ≥2 Charters" heuristic, but that crystallizes after a second adopter validates the pattern (same gate as the rest of v0 → v1).

---

## Drift detection

A small bash script is the verification layer that keeps the registry in sync with new AILOGs. The script lives in the adopter's repo (suggested path: `scripts/check-followups-drift.sh`) and has three modes.

### Modes

- **Default** — scan AILOGs modified in `git diff origin/main..HEAD` (with `HEAD~1..HEAD` fallback). Warn on any AILOG with `§Follow-ups` / `R<N> (new)` content that is not in `fully_extracted_ailogs`. Exit 1 on drift.
- **`--apply`** — same scan, but auto-append new entries under `## Bucket: ready` with auto-generated `FU-NNN` ids and append the AILOG id to `fully_extracted_ailogs`. The operator reclassifies into the correct bucket later.
- **`--scan-all`** — scan every AILOG in the project (periodic full sweep).

### Per-AILOG vs per-bullet granularity

Tracking is **per-AILOG**, not per-bullet. An AILOG is either fully extracted (its id is in `fully_extracted_ailogs` — trust the registry) or it is not (extract everything). Per-bullet matching would require fingerprinting (text hashing or fuzzy comparison), which produces false positives whenever a registry entry paraphrases the AILOG bullet — and curated entries always paraphrase.

The cost of per-AILOG granularity: when a follow-up is added to an already-extracted AILOG post-Charter close, drift detection misses it. The remediation is operator-driven — manually remove the AILOG from `fully_extracted_ailogs` and re-run with `--apply`. This trade-off is intentional for v0 because most AILOGs are write-once after Charter close.

### Script template

A reference implementation (~290 lines of POSIX bash) is in `StrangeDaysTech/sentinel` at `scripts/check-followups-drift.sh`. Copy it into your repo and adjust the constants at the top:

```bash
BACKLOG_FILE=".straymark/follow-ups-backlog.md"
AILOG_DIR=".straymark/07-ai-audit/agent-logs"
```

The script uses `awk` and `grep` only — no jq, no yq, no heavyweight dependencies. Portable across Linux and macOS.

---

## Agent integration

The agent (Claude / Gemini / etc.) becomes the primary maintainer of the registry. Add to your `CLAUDE.md` / `AGENT.md`:

```markdown
## Follow-ups backlog

- **Session start**: glance at `.straymark/follow-ups-backlog.md` to know what is pending across the project.
- **Pre-commit checklist**: created or modified any AILOG with `## Follow-ups` or `R<N> (new, not in Charter)` entries? → run `scripts/check-followups-drift.sh --apply` to extend the backlog in the same commit.
- **Post-Charter close**: review entries the Charter resolved; mark them `closed` (with the closing Charter id in `Notes`) or `superseded`. For un-resolved entries that meet the TDE criteria (heritage from prior Charter, transversal, requires dedicated Charter, needs human prioritization), promote them to a TDE document (see "Promotion to TDE" in this pattern + `AGENT-RULES.md §3`).
```

This makes the agent the maintainer, the script the verification layer, and the operator the periodic reviewer (re-bucketing, marking closed, pruning superseded, promoting to TDE when criteria apply).

---

## Adoption walkthrough

For an adopter starting fresh:

1. Create `.straymark/follow-ups-backlog.md` with the frontmatter above (empty `fully_extracted_ailogs:` list initially) and the five `## Bucket: <name>` headers.
2. Copy the reference script from `StrangeDaysTech/sentinel` to `scripts/check-followups-drift.sh`. Adjust `AILOG_DIR` if your AILOGs live elsewhere.
3. Run `scripts/check-followups-drift.sh --scan-all --apply` to seed the registry from existing AILOGs.
4. Reclassify the auto-generated `## Bucket: ready` entries into the correct buckets manually. This is one-time triage, typically 30-60 min for a backlog of ~50 entries.
5. Add the agent integration block to `CLAUDE.md` / `AGENT.md`.
6. Optionally wire `scripts/check-followups-drift.sh` into a pre-commit hook for hard enforcement.

For an adopter migrating from ad-hoc tracking: the same flow, but step 4 may require deciding which entries are already `closed` or `superseded` — that classification is what makes the registry useful.

---

## Reference implementation

`StrangeDaysTech/sentinel` CHARTER-12, merged 2026-05-06:

- Implementation PR: [sentinel#53](https://github.com/StrangeDaysTech/sentinel/pull/53) (registry + script + CLAUDE.md additions).
- Close-out PR: [sentinel#54](https://github.com/StrangeDaysTech/sentinel/pull/54) (post-merge verification + Charter close).

Empirical context: 47 entries seeded from CHARTER-08 → CHARTER-11 retrospective (~30 min of multi-agent triage). The chain demonstrated the gap that motivated the pattern — without the registry, the operator could not see "what is pending across the project" without re-classifying each Charter's follow-ups in isolation. With the registry, session-start glance is one file read.

---

## Open questions

These are not resolved in v0. Future revisions of this pattern, or a CLI helper, may address them:

- **Bucket classification heuristic**. Today `--apply` dumps every new entry into `## Bucket: ready`; the operator reclassifies manually. A heuristic using AILOG `tags` and Charter `effort_estimate` could suggest a bucket automatically.
- **Schema validation**. The registry follows a tacit schema; no `.straymark/schemas/follow-ups-backlog.schema.v0.json` exists yet. Validation today is human review.
- **Integration with the audit cycle**. When `straymark charter audit --merge-reports` produces real-debt findings that are not remediated atomically pre-close, those findings live only in `.straymark/audits/<id>/review.md`. They do not auto-flow into the central registry. Surfacing them automatically would close a known gap.
- **`closed` vs `superseded` semantics**. Today the difference is whether the resolving work explicitly referenced the entry. A stricter convention may emerge.
- **Cristalization as `straymark followups` CLI**. Once a second adopter validates the pattern, the framework may ship a subcommand mirroring the existing `straymark charter` trio: `list` / `close` / `drift`. Adopters at v0 (this pattern) migrate by deleting their local script and switching the agent instruction.

---

## Credits

Contributed via [issue #111](https://github.com/StrangeDaysTech/straymark/issues/111) by the Sentinel adopter. Empirical foundation: CHARTER-08 → CHARTER-11 chain in `StrangeDaysTech/sentinel`. Author: Claude Opus 4.7 on behalf of operator José Villaseñor Montfort.

---

## Related

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) — the meta-pattern that this drift-detection convention instantiates at the per-AILOG ↔ registry surface.
- [CHARTER-CHAIN-EVOLUTION.md](CHARTER-CHAIN-EVOLUTION.md) — sibling pattern that operates at chain level (Pattern 1) and cycle level (Pattern 2).
- [AGENT-RULES.md §3](AGENT-RULES.md) — TDE-vs-`R<N>` escalation criteria that may promote follow-ups to dedicated debt entries.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
