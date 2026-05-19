# TC260 Implementation Guide — StrayMark

> Practical guide for completing a TC260RA document according to the **AI Safety Governance Framework v2.0** (TC260, 15 Sep 2025).

**Languages**: English | [Español](i18n/es/TC260-IMPLEMENTATION-GUIDE.md) | [简体中文](i18n/zh-CN/TC260-IMPLEMENTATION-GUIDE.md)

---

## When to Create a TC260RA

Create a TC260 Risk Assessment when **any** of the following applies and `regional_scope` includes `china`:

- The system you are deploying or modifying is or includes an AI model.
- The application is destined for users in mainland China, or the operator is incorporated in mainland China.
- The system has cross-border data flows that touch mainland China.
- A new model version, scenario, or scale jump is anticipated.

A TC260RA complements the EU AI Act risk classification on ETH and is referenced via `related: [TC260RA-...]` from the relevant ETH / MCARD / AILOG.

---

## The Three Criteria

TC260 v2.0 grades risk along **three orthogonal axes**, then composes them into a single level (low / medium / high / very_high / extremely_severe).

### 1. Application Scenario (`tc260_application_scenario`)

What domain does the system operate in? Some scenarios are inherently higher-risk because errors translate into physical, financial, or societal harm.

| Scenario | Examples | Inherent risk floor |
|----------|----------|---------------------|
| `public_service` | Government chatbots, public information portals | medium |
| `healthcare` | Clinical decision support, medical imaging | high |
| `finance` | Credit scoring, KYC, fraud detection | high |
| `safety_critical` | Autonomous driving, industrial control, energy | very_high |
| `content_generation` | Text/image/video synthesis | medium |
| `social` | Recommendation, ranking, dating | medium |
| `industrial_control` | OT systems, robotics | very_high |
| `other` | Document briefly | — |

### 2. Level of Intelligence (`tc260_intelligence_level`)

How autonomous is the system?

| Level | Definition |
|-------|-----------|
| `narrow` | Single-purpose, deterministic outputs in a well-defined task |
| `foundation` | General-purpose foundation model (LLM, vision-language) without tool use |
| `agentic` | Foundation model + autonomous tool use, can take real-world actions |
| `general` | Approaching AGI — broad cross-domain competence |

### 3. Application Scale (`tc260_application_scale`)

How many users / how broad the impact?

| Scale | Definition |
|-------|-----------|
| `individual` | Single user / small team |
| `organization` | Single organization or enterprise |
| `societal` | Significant portion of the public (≥ 1M users) |
| `cross_border` | Operates across mainland China and other jurisdictions |

---

## Composing the Level

There is no published numeric formula. Use the matrix below as a starting point and document the reasoning.

| Scenario \ Intelligence | Narrow | Foundation | Agentic | General |
|-------------------------|--------|-----------|---------|---------|
| public_service | low → medium | medium | high | very_high |
| healthcare / finance | medium | high | high | very_high |
| safety_critical | high | very_high | very_high | extremely_severe |
| content_generation | low | medium | high | very_high |
| social | low | medium | high | very_high |
| industrial_control | high | very_high | very_high | extremely_severe |

**Scale modifier**:
- `individual` → no change.
- `organization` → no change.
- `societal` → bump one level (low→medium, medium→high, etc.).
- `cross_border` → bump one level **and** require explicit cross-border data analysis (see PIPL-PIPIA-GUIDE).

Always document the reasoning in section 2.4 of the TC260RA template — auditors will care about why, not what.

---

## Risk Taxonomy: How to Populate

The taxonomy in v2.0 distinguishes three families:

### Endogenous (`tc260_endogenous_risks`)

Inherent to the model:
- `hallucination` — confabulation, unsupported claims
- `bias` — protected-class disparities
- `robustness` — adversarial vulnerability
- `data_leakage` — training-data extraction
- `prompt_injection`
- `model_extraction`

### Application (`tc260_application_risks`)

Arising from technical use:
- `misuse` — repurposing for harmful tasks
- `scope_creep` — used beyond intended domain
- `dependency` — over-reliance on AI judgments
- `availability` — single point of failure for critical workflows
- `integration_flaw` — unsafe coupling with downstream systems

### Derivative (`tc260_derivative_risks`)

Second-order societal effects:
- `labor_displacement`
- `opinion_shaping`
- `ecosystem_disruption`
- `monoculture` — homogenization of outputs
- `loss_of_skill`

For `very_high` and `extremely_severe` levels, v2.0 explicitly requires **catastrophic-risk monitoring**: document this in section 5 (Governance Measures) of the TC260RA.

---

## Linking from Other Documents

When `tc260_risk_level: high` (or higher) is set on a non-TC260RA document, validation rule **CROSS-004** requires `review_required: true`. The TC260RA itself should be linked via `related:`:

```yaml
# Example: ETH document for a high-risk healthcare LLM
related:
  - TC260RA-2026-04-25-001
  - MCARD-2026-04-25-001
  - PIPIA-2026-04-25-001
```

---

## Review Cadence

| Trigger | Action |
|---------|--------|
| Model version change | Re-run section 4 (technical countermeasures) |
| Scenario expansion | Re-grade scenario × intelligence × scale |
| Scale crossing a tier (e.g., 1M users) | Bump level review |
| Regulatory update from TC260 | Full re-review |

<!-- StrayMark | https://strangedays.tech -->
