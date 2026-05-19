# Emergent Observation Design — StrayMark

> Why agents reading StrayMark documentation surface what was not asked: the structural and cultural properties that make cross-source dissonance detectable, and the pyramid of applied patterns that already instantiate this meta.

**Languages**: English | [Español](i18n/es/EMERGENT-OBSERVATION-DESIGN.md) | [简体中文](i18n/zh-CN/EMERGENT-OBSERVATION-DESIGN.md)

---

## Status

**v0 — proven in N=1 domain** (`StrangeDaysTech/sentinel`, Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) → Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156), 2026-04-21 through 2026-05-15).

This document names a *design property* of the StrayMark framework that produced an empirically observable emergent behaviour. The property is **not new** — it has been present since `00-governance/` was canonicalized — but it had not been *named*, which made it invisible to framework evolution and therefore at risk of accidental erosion. Naming it here protects it under the second-domain validation discipline of Principle #12.

---

## Why this document exists

An agent working in Sentinel surfaced — **without explicit trigger, without operator request, and without a CLI command designed to produce that output** — that `specs/002-commshub/plan.md` had accumulated twelve unreflected empirical learnings across seven consecutive Charters (CHARTER-07..17, ~1 month), and that filling CHARTER-18 against the stale spec carried ~50% probability of critical/high findings in the next audit cycle. That observation produced the upstream cycle that crystallized as `CHARTER-CHAIN-EVOLUTION.md` Pattern 1 in fw-4.16.0.

The behaviour was reproduced because two properties of the documentation aparatus *coexisted consistently*. Neither alone would have sufficed. Naming both, and naming their composition, lets the framework's future evolution preserve them deliberately rather than by inheritance.

The bridge `SPECKIT-CHARTER-BRIDGE.md` and the chain-evolution doc `CHARTER-CHAIN-EVOLUTION.md` document *one application* of this meta. This document names the meta itself and enumerates the other applications already shipped.

---

## The two design properties

### Property 1 — Structural cross-referencing (formal linkage)

The framework does **not** delegate cross-document linkage to agent intuition or to prose. Every document type has *required* frontmatter fields and *canonical* sections that declare, in the document's own structure, which other documents it points to and which sections of itself are open to specific kinds of surfacing.

Concrete instances that the agent encounters during routine reading:

- **Frontmatter linkage fields** that resolve to other StrayMark documents:
  - `originating_charter:` in AILOG / AIDEC frontmatter ([`AGENT-RULES.md` §5](AGENT-RULES.md), [`SPECKIT-CHARTER-BRIDGE.md` Charter↔AILOG section](SPECKIT-CHARTER-BRIDGE.md))
  - `originating_spec:` in Charter frontmatter ([`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) §Frontmatter linkage)
  - `originating_ailogs:` in Charter frontmatter (aggregation inverse)
  - `amends:` in amendment AILOG frontmatter ([`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2)
  - `promoted_from_followup:` in TDE frontmatter ([`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md))
  - `related:`, `supersedes:`, `superseded_by:` ([`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md))
- **Canonical sections inside templates** that hold deltas in queryable form:
  - `§Risk: R<N> (new, not in Charter)` in AILOGs ([`AGENT-RULES.md` §3](AGENT-RULES.md))
  - `## Follow-ups` per-AILOG ([`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md))
  - `## Batch Ledger` for multi-batch AILOGs
  - `## Historical correction (YYYY-MM-DD)` appended to original AILOG on amendment ([`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md))
- **Stable ID conventions** that make linkage cheap:
  - `[TYPE]-[YYYY-MM-DD]-[NNN]-[description]` for date-bounded docs
  - `CHARTER-NN-slug` (no timestamp, stable across renames)
  - `FU-NNN` monotonic global, never renumbered
- **Bridge documents** that declare canonical relations between layers:
  - `SPECKIT-CHARTER-BRIDGE.md` (Spec ↔ Charter ↔ AILOG)
  - `CHARTER-CHAIN-EVOLUTION.md` (Charter ↔ Charter chain ↔ Spec refresh)
- **CLI commands** that cross sources mechanically:
  - `straymark charter drift <id>` (declared scope ↔ commits)
  - `straymark charter refresh-suggest <module>` (telemetry rolling-mean ↔ refresh need)
  - `straymark validate` (frontmatter ↔ schema ↔ link integrity)

**The point of Property 1**: when the agent encounters a divergence between two sources, the divergence is *structurally visible* — not buried in prose. The agent does not need to invent the connection; the connection is declared by the framework.

### Property 2 — Cultural permission without gatekeeping

The framework gives the agent explicit, repeated permission to surface beyond the asked task — and pairs that permission with autonomy to *act* on the surfacing (create the AILOG, file the TDE, open the AIDEC) without pre-approval. The operator retains prioritization, not creation.

Concrete passages the agent encounters:

- **`AGENT-RULES.md` §6 "Be Proactive"** — *"Identify potential risks, Suggest improvements when evident, Alert about technical debt"*.
- **`AGENT-RULES.md` §6 "Be Transparent"** — *"Explain the reasoning behind decisions, Document considered alternatives, Admit uncertainty when it exists"*.
- **`AGENT-RULES.md` §12 Audit Checkpoint** — *"the agent proactively offers an external multi-model audit"* — institutionalizes the *act* of surfacing as part of the workflow.
- **`PRINCIPLES.md` §2 "AI Agent Transparency"** — *"Not hide relevant information"*.
- **`AGENT-RULES.md` §3 "Create Freely" autonomy table** — AILOG, AIDEC, TDE creation requires no pre-approval; the agent files and the operator prioritizes.
- **`FOLLOW-UPS-BACKLOG-PATTERN.md` script auto-append** — `check-followups-drift.sh --apply` adds FU-NNN entries to the central registry without operator intervention.

**The point of Property 2**: the agent has externalized *"should I say something?"* into *"is there a canonical section where this belongs?"*. If yes, surfacing is not a judgment call — it is execution of a documented rule. The cost of surfacing is low because the destination is pre-built.

### Why the composition matters

Property 1 *alone* — formal linkage without cultural permission — would produce a queryable corpus that no agent dares to query proactively. Property 2 *alone* — permission without structural cross-referencing — would produce vague surfacing ("I think something might be wrong somewhere") that operators cannot act on.

Composed, they produce the observed behaviour: an agent reads the AILOGs, counts `R<N>(new, not in Charter)` entries that materially diverge from the originating spec, sees that the spec has not been refreshed in a month, and — because `§6 Be Proactive` told it to alert and because the divergence has a name in the framework's vocabulary — surfaces *the specific, structurally-grounded delta* to the operator before proceeding with the asked task.

This is the meta-pattern.

---

## Empirical case: Sentinel spec-drift detection

The case is described in detail in Issues [#150](https://github.com/StrangeDaysTech/straymark/issues/150) and [#156](https://github.com/StrangeDaysTech/straymark/issues/156), and codified as Pattern 1 in [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md). The compressed sequence:

1. Sentinel runs `specs/002-commshub/plan.md` (committed 2026-04-21) through CHARTER-07..17 over ~1 month. Twelve empirical learnings accumulate across the AILOG chain in `§Risk: R<N>(new, not in Charter)` sections and `## Follow-ups`. Pattern propagation (handler shapes, table-reuse conventions, RLS helper, etc.) crystallizes during execution.
2. CHARTER-18 is about to be declared. The agent — without instruction to do so — triangulates `plan.md` against the AILOGs (where `§Risk` entries name the spec gaps) and against the code (where `straymark charter drift` would have detected per-Charter divergence had it been run cross-Charter). The framework's `originating_spec:` linkage in each Charter, `originating_charter:` in each AILOG, and the `§Risk: R<N>` convention make the triangulation mechanical, not heroic.
3. The agent surfaces *"if we fill CHARTER-18 reading the stale plan, the next audit cycle's H1/M1 findings will be remediating divergences atomically pre-close — ~50% probability of ≥1 critical/high finding from stale-premise inheritance"* — citing specific AILOGs by ID and specific code references.
4. The operator files Issue #150 as an RFC. The Sentinel-local AIDEC documents the proposed scope-limited refresh discipline + three mechanical gates.
5. Issue #156 upstreams the pattern. `CHARTER-CHAIN-EVOLUTION.md` Pattern 1 lands in fw-4.16.0 with telemetry slot `pre_declare_refresh:`, the helper `straymark charter refresh-suggest`, and the categorized learnings table contract.

The observation is empirically reproducible: any spec that produces ≥3 Charters spaced ≥1 week apart will exhibit some degree of plan-vs-code drift, and an agent reading the framework's documentation has the structural and cultural permission to detect and surface it.

---

## Pyramid of instances — applications of the meta-pattern

The meta-pattern sits above several already-canonicalized patterns. Each is an *application* of the same underlying composition (formal linkage + cultural permission) to a specific source pair.

| Application | Source pair | Where canonicalized |
|---|---|---|
| Pre-declare SpecKit refresh (Pattern 1) | spec ↔ AILOGs ↔ code | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 1 |
| Post-close audit-driven amendment (Pattern 2) | audit findings ↔ closed Charter | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2 |
| Charter drift detection | declared scope ↔ commits | [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) + `straymark charter drift` |
| Follow-ups backlog drift | per-AILOG `§Follow-ups` ↔ central registry | [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) + `check-followups-drift.sh` |
| TDE-vs-`R<N>` escalation | accumulated `§Risk: R<N>` ↔ TDE backlog | [`AGENT-RULES.md`](AGENT-RULES.md) §3 |
| External audit checkpoint | implementation-complete state ↔ multi-model review | [`AGENT-RULES.md`](AGENT-RULES.md) §12 |

These are not ad-hoc conventions. They share the same shape: *two canonical sources connected by frontmatter or section linkage, with the agent permitted (sometimes required) to surface the delta*. The next application axis — whatever it turns out to be — will recognize itself in this table.

---

## Anti-patterns: how the meta is broken

The meta-pattern is fragile. Each of the following, if introduced, regresses the framework's ability to produce emergent observations.

- **Frontmatter linkage as optional**. If a new document type ships with `related:` / `originating_*` as advisory rather than required, the cross-referencing graph develops blind spots and the agent loses the ability to triangulate across that type.
- **Canonical sections collapsed into prose**. If `§Risk: R<N>` is replaced by *"discussion of risks"*, the queryability evaporates. The agent can no longer count `R<N>` entries to detect the saturation threshold that drives `refresh-suggest`. Free-form prose is not queryable; structured sections are.
- **Gatekeeping on agent-created docs**. Requiring pre-approval to file AILOG / AIDEC / TDE kills Property 2. The agent reverts to surfacing only what was asked, because the cost of surfacing rises above the local benefit.
- **Telemetry without emergent signals**. If `.telemetry.yaml` schemas evolve without preserving signals like `r_n_plus_one_emergent_count`, the operator loses visibility into how often the agent is surfacing emergent risks. The feedback loop breaks; the meta becomes invisible to framework evolution.
- **CLI commands that bypass the surface**. A CLI that emits decisions directly (no AILOG written, no `R<N>` section populated) bypasses the structural surface. The agent's downstream triangulation degrades because the source pair is no longer connected via documents.

---

## Open application axes — where the meta could replicate

The audit underlying this document identified four loci where the structural infrastructure exists *partially* but the cultural permission or the application pattern has not been named. These are candidates for future application of the meta, not commitments to ship.

- **MCARD ↔ deployed model code** — `TEMPLATE-MCARD.md` exists; no `model-version-at-close` field in Charter telemetry, no AILOG `deployed_mcard:` linkage field, no drift detection pattern. A model deployment that diverges from the MCARD on file is currently invisible.
- **SBOM ↔ lockfiles** — `AI-RISK-CATALOG.md` §RISK-004 mentions SBOM maintenance for AI components; no canonical AILOG field linking to SBOM, no drift script (analogous to `check-followups-drift.sh`) that compares declared SBOM against actual `package.lock` / `requirements.txt`, no telemetry signal for dependency-change events.
- **ADR vigente ↔ contradicting implementation** — `.telemetry.yaml` schema captures `decisions_contradicting_prior_adrs` but no protocol tells the agent *when* to surface a contradiction it observes during implementation. The signal exists; the surfacing convention does not.
- **Constitution Check ↔ framework version bump** — `SPECKIT-CHARTER-BRIDGE.md §Constitution Check re-evaluation cadence` codifies the cadence verbally; no automatic alert fires on `straymark update-framework`. A framework bump between Charters can change Constitution gates silently.

These four are tracked in a single upstream RFC issue (filed after this document lands). Each requires empirical N=1 adopter validation before crystallizing as a named pattern — Principle #12 applies.

---

## Authority / acceptance flow for naming new meta-applications

The same upstream-acceptance flow that `CHARTER-CHAIN-EVOLUTION.md` documents applies recursively to this meta. A new application axis (one of the four above, or a fifth that emerges) lands as:

1. **Adopter-local RFC** at `.straymark/06-evolution/<axis>-rfc.md` describing the structural connection that already exists (or is being added) and the cultural permission rule the agent should follow.
2. **Upstream Issue** mirroring the RFC, citing the AILOGs/Charters/telemetry where the empirical observation occurred.
3. **Upstream acceptance** as: (a) update to the relevant template / schema / governance doc to add the missing structural piece (frontmatter field, canonical section, telemetry signal); (b) addition of the axis to the "Pyramid of instances" table in this document; (c) optional CLI scaffolding for mechanical detection.
4. **Second-domain validation** before the axis's schema additions graduate from optional to recommended.

This document itself instantiates step 3.b for the meta — the upstream-acceptance output of recognizing that the existing applications share a single underlying property.

---

## Open questions

- **Operationalizing "material divergence"**. The Principle #8 wording ([`PRINCIPLES.md`](PRINCIPLES.md)) leaves "material" to agent judgement. Per-application thresholds (Pattern 1 uses `r_n_plus_one_emergent_count > 6` rolling mean) are calibrated empirically. Whether a cross-axis threshold is achievable, or whether each axis must calibrate its own, is open.
- **Telemetry consolidation**. Each application currently emits its own telemetry slot (`pre_declare_refresh:`, `post_close_amendment:`, `r_n_plus_one_emergent_count`). A consolidated *"emergent observations surfaced this Charter"* counter might make the meta visible at the metrics level. Deferred — premature aggregation risks losing per-axis signal granularity.
- **Adopter onboarding**. New adopters reading `STRAYMARK.md` for the first time should encounter the meta early enough that they recognize the pattern when they experience it. Whether that lives in `QUICK-REFERENCE.md`, in `STRAYMARK.md` itself, or in a new onboarding section is open.

---

## Related

- [`PRINCIPLES.md`](PRINCIPLES.md) §8 — *Cross-Source Dissonance Surfacing* (the cultural rule, condensed).
- [`AGENT-RULES.md`](AGENT-RULES.md) §6 — *Be Proactive* (the operative mandate); §3 — *TDE vs `R<N>`* (one application surface); §12 — *Audit Checkpoint* (institutionalized surfacing).
- [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) — Pattern 1, Pattern 2 (the two highest-level applications).
- [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) — Charter as the bridge layer where Property 1's linkage is densest.
- [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) — drift detection at the per-AILOG ↔ registry surface.
- [`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md) — frontmatter and `related:` field canon.
- [`../../STRAYMARK.md`](../../STRAYMARK.md) §15 — Charter as the bounded unit where applications converge.

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) · [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
