# CAC Algorithm Filing Guide — StrayMark

> Practical guide for completing a CACFILE document and navigating the dual-filing process administered by the Cyberspace Administration of China (CAC).

**Languages**: English | [Español](i18n/es/CAC-FILING-GUIDE.md) | [简体中文](i18n/zh-CN/CAC-FILING-GUIDE.md)

---

## When CAC Filing is Required

Filing is required for services with **public-opinion attributes** or **social-mobilization capabilities** under:

- *Provisions on the Administration of Algorithmic Recommendations of Internet Information Services* (in force since 2022-03-01)
- *Provisions on the Administration of Deep Synthesis* (in force since 2023-01-10)
- *Interim Measures for the Management of Generative AI Services* (in force since 2023-08-15)

Indicators that the rules apply:

- The service generates, synthesizes, or disseminates content viewable by the public.
- The service uses ranking, recommendation, search, or content selection that influences user opinion.
- The service operates in news, social media, audio/video, live-streaming, or generative-AI domains.
- The service is provided to users in mainland China.

If any of the above apply, set `cac_filing_required: true` on the MCARD and create a CACFILE. Validation rule **CROSS-007** enforces the link.

---

## Single vs. Dual Filing

Two regulatory tracks coexist:

| Track | Authority | What | When |
|-------|-----------|------|------|
| **Algorithm Filing** | National CAC (after online submission) | Disclosure of algorithm mechanism, target audience, security responsibility report | All in-scope algorithms |
| **Generative AI Large Model Filing** | Provincial CAC → National CAC | Two-stage technical testing + algorithm disclosure | Generative AI services with public-opinion / social-mobilization attributes |

Generative AI services typically need **dual filing**: provincial security assessment first, then national algorithm filing. Set `cac_filing_type: dual` in this case.

---

## Required Documents and Information

The CACFILE template captures the canonical inputs. The most commonly requested artifacts are:

| Artifact | Where in CACFILE | Notes |
|----------|------------------|-------|
| Service overview | § Service Overview | Name, provider, application domain, target audience |
| Algorithm self-assessment report | § Self-Assessment | Risks + mitigations across content compliance, privacy, bias, IP, minor protection |
| Internal algorithm management policy | § Internal Algorithm Management Policy | Designated responsible person, complaint handling, logging |
| Algorithm Security Responsibility Report | § Algorithm Security Responsibility Report | Per Art. 24 of the Algorithm Recommendation Provisions |
| Training data description | § Training Data Description | Sources, volume, lawfulness, sensitive categories filtered |
| Blocked keywords list | § Blocked Keywords Strategy | Prevent generation of content subverting state power, inciting separatism, undermining national unity, etc. |
| Testing question set | § Testing Question Set | Red-team prompts + pass thresholds |
| Service agreements + complaint channels | (free-form, link from § Internal Algorithm Management Policy) | User-facing terms and SLA |

---

## Filing Status Lifecycle

| Status | Meaning |
|--------|---------|
| `pending` | CACFILE drafted but not yet submitted |
| `provincial_submitted` | Submitted to provincial CAC; awaiting decision |
| `provincial_approved` | Provincial decision favorable; can proceed to national submission (for dual filing) |
| `national_submitted` | Submitted to national CAC |
| `national_approved` | Filing number issued (`cac_filing_number` populated) |
| `rejected` | Filing rejected; document reasons in *Filing Trail* |
| `not_required` | Initial assessment concluded the service is out of scope |

Validation rule **CAC-002** (compliance check) flags filings that have been `pending` for more than 90 days as a partial pass. **CROSS-006** requires `cac_filing_number` to be populated whenever status is any `*_approved` value.

---

## Public Disclosure of the Filing Number

Once `cac_filing_number` is issued, providers must surface it to end-users. Common patterns:
- Footer of the service homepage: "**算法备案号: 网信算备XXXXXXXXXXXX**".
- Settings / About page within the app.
- Terms of service.

Document the disclosure surface in the *Filing Trail* section, last row.

---

## Self-Assessment Risk Categories

The self-assessment must address at least these areas (CACFILE § Self-Assessment):

- **Content compliance**: prevention of illegal content (subversion, separatism, terrorism, obscenity, violence, ethnic hatred, etc.).
- **Personal information protection**: alignment with PIPL (cross-link to PIPIA).
- **Bias / unfair differentiated treatment**: covers price discrimination, ranking manipulation, demographic bias.
- **Intellectual property**: training data licensing, output similarity to copyrighted works.
- **Minor protection**: special handling of users under 18 — content filters, time limits, anti-addiction.

For services using third-party foundation models, the self-assessment must include an evaluation of the upstream provider's filing status.

---

## Linkage to Other StrayMark Documents

A complete filing artifact set typically references:

```yaml
# Inside CACFILE-2026-04-25-001 frontmatter
related:
  - MCARD-2026-04-25-001  # the model being filed
  - TC260RA-2026-04-25-001  # TC260 risk assessment
  - PIPIA-2026-04-25-001  # if personal information is processed
  - AILABEL-2026-04-25-001  # GB 45438 labeling plan
  - SBOM-2026-04-25-001  # training data inventory (GB/T 45652 compliance)
```

This linkage is what `straymark compliance --standard china-cac` walks to verify.

---

## Sources

- [China's Algorithm Filing Regime — Lexology](https://www.lexology.com/library/detail.aspx?g=3c7273cf-8f85-4702-af70-6edf394ff1c3)
- [Interim Measures for the Management of Generative AI Services — Library of Congress](https://www.loc.gov/item/global-legal-monitor/2023-07-18/china-generative-ai-measures-finalized/)
- [346 generative AI services filed with CAC — SCIO](http://english.scio.gov.cn/pressroom/2025-04/09/content_117814020.html)

<!-- StrayMark | https://strangedays.tech -->
