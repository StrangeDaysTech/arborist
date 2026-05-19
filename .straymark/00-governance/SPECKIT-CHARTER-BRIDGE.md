# SpecKit ↔ StrayMark Charter Bridge

> **Status**: Empirical pattern (`v0`). Crystallizes after validation against a second domain (Principle #12). Refine via PRs as new use cases surface.

## Problem this document solves

[SpecKit](https://github.com/StrangeDaysTech/speckit) gives you `spec.md`, `plan.md`, and `tasks.md` for a feature. StrayMark gives you Charters, AILOGs, AIDECs, ADRs. **No canonical doc explained when a SpecKit feature should yield a Charter, how granular it should be, who triggers the creation, or when.** Reported as the central artifact of [issue #113](https://github.com/StrangeDaysTech/straymark/issues/113) — a discoverability gap that left agents (Claude, Gemini, Copilot) building binary mental models (`SpecKit = planning, StrayMark = audit-trail`) and silently dropping the third layer (work-as-auditable-shippable-unit) where Charters live.

This file is the answer.

## Mental model

Three layers, with handoffs:

| Layer | Lives in | Purpose | Owner |
|-------|----------|---------|-------|
| **1. Specification** | `specs/NNN-feature/{spec,plan,tasks,research,quickstart}.md` | What the feature is, why it exists, how it'll be implemented at a technical level. SpecKit's `/speckit-specify` → `/speckit-plan` → `/speckit-tasks` produces these. | Operator (with agent assist). |
| **2. Bounded execution unit** | `.straymark/charters/NN-slug.md` | The contract for a single shippable cut of the feature. Pairs ex-ante scope (files, risks, tasks subset) with ex-post telemetry (drift, audit, lessons). | Operator declares the Charter; agent executes within it. |
| **3. Implementation trace** | `.straymark/07-ai-audit/agent-logs/AILOG-*.md` (and AIDECs, ADRs as warranted) | Day-to-day record of what was actually done, why, with what confidence. Each AILOG references the Charter via `originating_charter:` (or the Charter aggregates them via `originating_ailogs:`). | Agent creates as it works; operator reviews. |

**The bridge is the Charter.** Specs are too high-level to drift-check ("did you ship the spec?" is unanswerable in a useful timeframe). AILOGs are too low-level to ship against ("did you ship this AILOG?" is the wrong unit). Charters sit at the right granularity: a stable scope contract you can audit against in days, not months.

## When does a SpecKit feature yield a Charter?

A SpecKit feature should produce **at least one Charter** when *any* of the following hold:

1. The feature's `tasks.md` has **5 or more tasks** that you cannot complete in a single session.
2. The feature spans **2 or more SpecKit phases** (Setup, Foundation, User Stories, Polish, etc.) that you intend to ship together as one unit.
3. The work warrants **external audit** (cross-model review, cross-team review) at completion.
4. You want **measurable telemetry** at close (effort estimate vs. actual, drift count, lessons).

It should **not** produce a Charter when:

- The feature is small enough to ship in one session (<1 day, <5 tasks). Use AILOGs alone — the Charter overhead exceeds the auditability gain.
- The feature is **purely planning** (no code yet). Wait until `tasks.md` exists; the Charter contract needs concrete tasks to enumerate.
- The feature is **maintenance** without a planned scope (e.g., "fix bugs as they come"). For ad-hoc maintenance, AILOGs are sufficient.

## Granularity heuristics

When a feature warrants Charters, choose granularity by **shippable unit**, not by structural unit. Concretely:

### Heuristic 1 — One Charter per shippable cut

If the feature has Phases (e.g., SpecKit's typical Foundation → US1 → US2 → US3 → Polish), the **first Charter wraps the foundation cut** (everything that ships together as `v0.1`). Subsequent Charters wrap subsequent cuts. Effort estimate **M** is the median bucket for a shippable cut; **L** for a full-feature cut.

```
specs/001-peek-mvp-foundation/
├── spec.md
├── plan.md
└── tasks.md  →  CHARTER-01 (Foundation: T001-T012, effort M)
                  CHARTER-02 (peek MVP: T013-T044, effort L)
```

### Heuristic 2 — NOT per User Story

User Stories are too granular. A US that takes 2-3 tasks belongs *inside* a Charter, not as its own Charter. Telemetry per US is noise; telemetry per shippable cut is signal.

### Heuristic 3 — NOT per feature

A feature that ships in two cuts (e.g., MVP → polish) deserves two Charters, not one. The Charter contract you can drift-check is "what shipped in this cut", not "what we eventually built".

### Heuristic 4 — Edge case: ≥10 tasks across 4+ phases

When a feature is exceptionally large, a third Charter (or splitting the foundation cut into "scaffolding" + "core") may be warranted. Use effort estimate **L** as the cap; if you'd estimate **XL**, that's a sign the feature should be re-specified.

## Creation timing

```
/speckit-specify  → spec.md
/speckit-plan     → plan.md
/speckit-tasks    → tasks.md
                    ↓
                ┌────────────────────────────────────────┐
                │  ★ CHARTER DECLARATION POINT ★         │
                │                                        │
                │  Operator runs `straymark charter new` │
                │   --from-spec specs/NNN-feature/spec.md│
                │   --type <M|L>                         │
                │                                        │
                │  Charter status: declared              │
                │  → Operator fills scope, files, tasks  │
                │  → status: in-progress when execute    │
                └────────────────────────────────────────┘
                    ↓
/speckit-implement  → tasks executed
                    → AILOGs created (`originating_charter:` → Charter)
                    ↓
straymark charter drift CHARTER-NN  → file-vs-commit check
straymark charter audit CHARTER-NN  → external audit (optional)
straymark charter close CHARTER-NN  → telemetry, status: closed
```

**Key invariant**: declare the Charter *before* `/speckit-implement` starts. The Charter is a contract; declaring it after execution defeats the drift check.

## Frontmatter linkage

The Charter's frontmatter explicitly cites the SpecKit feature:

```yaml
charter_id: CHARTER-01-workspace-foundation
status: declared
effort_estimate: M
trigger: tasks.md has 12 ordered tasks across 2 phases; ship as v0.1.
originating_spec: specs/001-peek-mvp-foundation/spec.md
```

The reverse direction (spec → Charter) is by convention — list the active Charter in the spec's "Phase 5: Implementation Tracking" section if your `plan.md` template has one. SpecKit currently has no schema slot for this; emerging convention.

AILOGs created during execution should cite the Charter:

```yaml
id: AILOG-2026-05-08-005
title: T013, T016-T026 — US1 P1 MVP core + TUI + peek bin
agent: claude-code-v4.7
confidence: high
risk_level: medium
review_required: false
originating_charter: CHARTER-02-peek-mvp-foundation
```

## Lifecycle map

| SpecKit phase | Charter event | StrayMark CLI |
|---------------|---------------|---------------|
| `/speckit-tasks` complete | **Declare Charter** | `/straymark-charter-new` skill or `straymark charter new --from-spec …` |
| First task starts | Operator flips `declared` → `in-progress` | (manual frontmatter edit) |
| Each task executed | AILOG produced (when warranted by §6 of STRAYMARK.md) | `/straymark-ailog` |
| Major decision encountered | AIDEC produced | `/straymark-aidec` |
| Architectural shift | ADR produced | `/straymark-adr` |
| Last task done, before close | Drift check | `straymark charter drift CHARTER-NN` |
| Optional external review | Multi-model audit | `straymark charter audit CHARTER-NN` + `/straymark-audit-prompt` + `/straymark-audit-execute` + `/straymark-audit-review` |
| Cut shipped | Close Charter | `straymark charter close CHARTER-NN` (status: `closed`, telemetry yaml emitted) |

## Spec maintenance during multi-Charter execution

> **Empirical anchor**: surfaced by [issue #150](https://github.com/StrangeDaysTech/straymark/issues/150) after Sentinel ran a single `specs/002-commshub/plan.md` (committed 2026-04-21) through **seven consecutive Charters** (CHARTER-07 through CHARTER-17, ~1 month). Twelve aprendizajes empíricos that materially impact the next Charter's scope were *not* reflected in the plan. The pattern below codifies what Sentinel discovered before CHARTER-18 fill.
>
> **Canonical extension** *(fw-4.16.0+)*: this section's procedural guidance (when + how + gates) is the mechanical floor. The *named pattern* — including the `r_n_plus_one_emergent_count` rolling-mean trigger heuristic, the categorized-learnings table contract, telemetry slot `pre_declare_refresh:`, and the `straymark charter refresh-suggest` helper — lives in [CHARTER-CHAIN-EVOLUTION.md](CHARTER-CHAIN-EVOLUTION.md) **Pattern 1**. Read both: this doc tells you the *how*, that one names the *pattern* and feeds it back into telemetry.

The lifecycle map above assumes **one-pass**: SpecKit artifacts are generated once, then Charters are declared and executed. This scales fine for features that produce a single Charter. When a single spec drives many Charters spaced weeks apart, **planning artifacts drift relative to shipped code** — and naively re-running `/speckit-plan` is *worse*, not better: regeneration asserts things about already-shipped user stories that the actual code does not implement, and future readers (auditors, agents, new operators) trust those regenerated artifacts as ground truth.

This section answers **how**, not **whether**: what discipline keeps the spec in sync with code during multi-Charter execution **without** the regeneration step lying about the parts that already shipped.

### When to refresh

A spec-refresh is warranted when *any* of the following hold:

1. **≥3 Charters closed against the same spec** — the volume of unreflected execution detail is high enough that the next Charter's scope decisions risk inheriting stale premises.
2. **≥4 calendar weeks** since the spec was last refreshed (or since initial generation) and ≥2 Charters closed in that window.
3. **AILOG `## Risk: R<N>(new, not in Charter)` count on the spec exceeds ~6 across closed Charters** — the spec's anticipation of risk has measurably under-described the territory.
4. **The next Charter's user story touches infrastructure the prior Charters refined empirically** (new tables/migrations created, helpers extracted, contracts crystallized) and the spec describes the pre-refinement state.

If none hold and the next Charter targets a fresh sub-system the prior Charters didn't touch, **skip the refresh**. Spec stability has value; refreshing on every Charter creates churn without proportional clarity gain.

### How to refresh: scope-limited prompt

Do **not** re-run `/speckit-plan` with a blank slate. The regenerated `plan.md` + `research.md` + `data-model.md` + `contracts/` + `quickstart.md` will assert things about already-shipped user stories that the actual code does not implement.

Instead, invoke `/speckit-plan` with a **scope-limited prompt** that:

1. **Names the target phase explicitly** (e.g., "refresh planning for US5 only — failover + tracking").
2. **Lists locked sections that must not change** (e.g., "Foundation, US1, US2, US3, US4 sections are immutable — the code shipped against them is the ground truth, not the plan").
3. **Cites the AILOGs that document refinements** (e.g., "see AILOG-2026-05-11-043 §R5 for the `processed_events` reuse pattern; reflect this in the refreshed data model").
4. **Forbids regenerating `tasks.md`** — see next subsection.

The output is a `plan.md` (and possibly `research.md` / `data-model.md` / `contracts/`) where the target-phase content is fresh and the locked sections carry forward the actually-shipped state, not the original aspirational state.

### Three mechanical gates post-refresh

Before merging the spec-refresh PR, three gates run sequentially:

**Gate (a) — Validation against code reality.**
For each non-target-phase entity in `data-model.md`, diff against the actual `db/migrations/*.sql` (or equivalent schema source). For each non-target-phase endpoint in `contracts/*.md`, diff against actual handler signatures. Any divergence in a *locked* section blocks merge — that's the regeneration lying. Adopters may script this against their stack; a CLI helper (`straymark spec-drift`) is on the roadmap (see #150 Ask 3).

**Gate (b) — Granular diff hunk-by-hunk review.**
Run `git diff specs/NNN-feature/` and review file-by-file, hunk-by-hunk. No changes to locked sections accepted without an explicit justification comment in the PR. The diff is small enough when scope-limited that this is feasible in one sitting.

**Gate (c) — Two-PR split.**
Land the spec-refresh as its own PR. Review it against the *code*, not against the plan-only output. Then fill the target Charter against the refreshed spec in a *separate* PR. Mixing the two collapses review surfaces: reviewers can no longer tell whether a hunk reflects new shipped state or new planned state.

### Why NOT re-run `/speckit-tasks` mid-spec-execution

The `tasks.md` file accumulates implementation trace state during execution: `[X]` checkboxes on completed tasks, `*CHARTER-NN: <commit-sha>*` annotations citing which Charter shipped which task, possibly `^skipped` markers with rationale. **Regenerating `tasks.md` destroys this state.** The file becomes a fresh task list with no record of what already shipped.

Discipline: **never** re-run `/speckit-tasks` while a spec is in the middle of multi-Charter execution. Instead, **manually edit `tasks.md`** for the target phase only — append new tasks for the refreshed scope, leave the already-shipped sections (`[X]` + `*CHARTER-NN:*` annotations) untouched.

If you discover that the original `tasks.md` had errors in shipped sections (e.g., a task was incorrectly marked `[X]` when its work was actually split across two Charters), correct it manually with a Git commit. Treat `tasks.md` as a historical record from the point of first execution onward; it is no longer a regeneratable artifact.

### Constitution Check re-evaluation cadence

SpecKit's Constitution Check is typically run once at `/speckit-plan` time. In multi-Charter execution against a single spec, the question of *when* to re-evaluate is left implicit. To make this explicit:

- **Per-Charter (recommended)** — re-evaluate Constitution Check at the start of each new Charter declared against the spec. The check is cheap (read the constitution; compare against the Charter's declared scope) and catches drift early, before execution commits.
- **Per-spec-refresh (mandatory when the refresh happens)** — when a scope-limited `/speckit-plan` refresh lands, the refresh PR must re-run Constitution Check against the refreshed plan. If the framework version moved (e.g., `fw-4.10.x → fw-4.14.x`), Constitution Check may yield different results because new gates exist.
- **NOT per-framework-bump alone** — a `straymark update-framework` between Charters does *not* require an immediate Constitution Check re-run on the open spec. The check applies at the next natural boundary (next Charter declaration or spec-refresh).

Codifying this as explicit cadence (rather than "whoever decides") closes a recurring ambiguity reported by Sentinel post-CHARTER-17.

### Roadmap: `straymark spec-drift`

A CLI command analogous to `straymark charter drift`, but operating at spec granularity — parse `data-model.md` → entities → diff against `db/migrations/*.sql`; parse `contracts/*.md` → endpoints → diff against handler signatures. It would mechanize Gate (a) above.

Deferred deliberately to a post-announcement Charter (tracked separately). The CLI surface is meaningful only for adopters whose spec format follows SpecKit conventions; the language-detection layer (Go vs Rust vs TypeScript vs Python handlers; SQL vs ORM-defined schemas) is non-trivial and warrants its own design cycle informed by real adopter stacks. The discipline above (Gates a/b/c executed manually) is the v0; the CLI is the v1 that mechanizes the most expensive gate.

## Anti-patterns

**Don't open a Charter "to be safe".** A Charter without a clear shippable cut becomes a wishlist. Operators end up closing it as `closed: aborted` and the telemetry is meaningless.

**Don't open a Charter per User Story.** Telemetry-per-US is too noisy to inform future estimates. Aggregate.

**Don't skip the `originating_spec` field.** Even if the Charter wraps work that doesn't have a SpecKit spec, set `originating_ailogs:` instead. Charters with no origin are an anti-pattern (they signal undocumented motivation).

**Don't run `straymark charter audit` without the auditor CLIs available.** The audit is orchestration-only — `straymark` does not call LLM APIs. If you don't have N auditor CLIs ready, skip the step; close the Charter without external audit.

**Don't flip status to `closed` before drift check + telemetry yaml.** `straymark charter close` does both atomically; manual closure skips invariants.

**Don't re-run `/speckit-tasks` mid-spec-execution.** Regenerating `tasks.md` destroys the `[X]` completion marks and `*CHARTER-NN:* …` annotations that form the historical trace. See "Spec maintenance during multi-Charter execution" above for the safe path (manual edit for the target phase only).

## When this pattern doesn't fit

This bridge assumes a SpecKit-driven feature flow with multi-task, multi-session implementation. It does not fit:

- **Single-session features** — use AILOGs alone.
- **Architecture-only work with no implementation** (e.g., "design the next-gen schema") — use ADRs.
- **Pure refactors with no new behavior** — use AILOGs + tag with `refactor:`.
- **Incident response and hotfixes** — use INC + AILOG.
- **Compliance-only deliverables** (e.g., quarterly DPIA refresh) — use the relevant doc type directly.

If your work fits one of those, *declare no Charter*. The cost of a Charter exceeds the value when there's no shippable cut to wrap.

## See also

- [`EMERGENT-OBSERVATION-DESIGN.md`](EMERGENT-OBSERVATION-DESIGN.md) — the meta-pattern that explains *why* multi-source linkage in this bridge produces emergent observations during multi-Charter execution.
- `STRAYMARK.md` §6 (When to Document) and §15 (Charters as bounded units of work)
- `.straymark/templates/charter/charter-template.md` — declarative template
- `.straymark/templates/charter/charter-telemetry-template.yaml` — telemetry template
- `.straymark/schemas/charter.schema.v0.json` — JSON Schema for declarative frontmatter
- `.straymark/schemas/charter-telemetry.schema.v0.json` — JSON Schema for telemetry
- `.claude/skills/straymark-charter-new/SKILL.md` (and Gemini / agnostic equivalents)

> **Cited the empirical context** (issue #113): Greenfield Rust CLI/TUI suite, Claude Opus 4.7 onboarding via canonical entry points (`STRAYMARK.md`, project constitution, `CLAUDE.md` checklist, available `/straymark-*` skills, `/straymark-status`). Charters were *eventually* adopted (2 Charters: foundation + MVP) only after explicit user prompt — confirming the gap was systemic, not session-specific. This document removes the gap.

---

*Languages*: English | [Español](i18n/es/SPECKIT-CHARTER-BRIDGE.md) | [简体中文](i18n/zh-CN/SPECKIT-CHARTER-BRIDGE.md)
