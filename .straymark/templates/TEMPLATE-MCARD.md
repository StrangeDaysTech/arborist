---
id: MCARD-YYYY-MM-DD-NNN
title: "[Model Name] Card"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true
risk_level: medium
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: [8]
model_name: ""
model_type: LLM  # LLM | classifier | regressor | generator | recommender | other
model_version: ""
provider: ""
license: ""
tags: [ai-model]
related: []
---

# MCARD: [Model Name] Card

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval before proceeding.

## Model Details

> Based on Mitchell et al. (2019) — "Model Cards for Model Reporting".

| Field | Value |
|-------|-------|
| Developer | [Organization or individual who developed the model] |
| Model Date | [YYYY-MM-DD — date the model was trained or released] |
| Model Version | [Version identifier] |
| Model Type | [LLM / classifier / regressor / generator / recommender / other] |
| Training Algorithms | [Algorithm(s) used for training] |
| Base Model | [Base model name and version, if fine-tuned; N/A otherwise] |
| Paper / Resource | [URL or citation to paper, blog post, or documentation] |
| Citation | [BibTeX or plain-text citation] |
| License | [Model license — e.g., Apache 2.0, MIT, proprietary] |

## Intended Use

### Primary Intended Uses

- [Primary use case 1]
- [Primary use case 2]

### Primary Intended Users

- [User group 1]
- [User group 2]

### Out-of-Scope Uses

- [Use case the model is NOT designed for 1]
- [Use case the model is NOT designed for 2]

## Training Data

> For SBOM interoperability, consider aligning with CycloneDX `modelCard.modelParameters` fields.

| Field | Value |
|-------|-------|
| Dataset Name | [Name of the training dataset] |
| Source | [Where the data was obtained] |
| Size | [Number of samples, tokens, or storage size] |
| Collection Methodology | [How the data was collected] |
| Preprocessing | [Cleaning, filtering, augmentation steps applied] |
| Known Limitations | [Biases, gaps, or quality issues in the data] |
| PII Assessment | [Whether PII is present and how it was handled] |
| License | [License governing the training data] |

## Performance Metrics

| Metric | Value | Test Dataset | Confidence Interval | Conditions |
|--------|:-----:|--------------|:--------------------:|------------|
| [Accuracy / F1 / BLEU / etc.] | [Value] | [Dataset name and split] | [95% CI range] | [Conditions or configuration] |
| [Metric 2] | [Value] | [Dataset name and split] | [95% CI range] | [Conditions or configuration] |
| [Metric 3] | [Value] | [Dataset name and split] | [95% CI range] | [Conditions or configuration] |

### Disaggregated Evaluation

> Report performance broken down by relevant subgroups when applicable.

| Subgroup | Metric | Value | Baseline Comparison |
|----------|--------|:-----:|:-------------------:|
| [Subgroup 1] | [Metric] | [Value] | [+/- vs overall] |
| [Subgroup 2] | [Metric] | [Value] | [+/- vs overall] |

## Bias and Fairness Evaluation

| Demographic Group | Metric | Performance | Disparity vs Baseline | Mitigation Applied |
|-------------------|--------|:-----------:|:---------------------:|--------------------|
| [Group 1 — e.g., age range, gender, ethnicity] | [Metric] | [Value] | [+/- percentage or absolute] | [Mitigation description] |
| [Group 2] | [Metric] | [Value] | [+/- percentage or absolute] | [Mitigation description] |
| [Group 3] | [Metric] | [Value] | [+/- percentage or absolute] | [Mitigation description] |

## Environmental Impact

| Metric | Value | Notes |
|--------|-------|-------|
| Training Energy (kWh) | [Value] | [Methodology or estimation source] |
| CO2 Equivalent (tons) | [Value] | [Grid carbon intensity used] |
| Hardware Used | [GPUs/TPUs, count, model] | [Cloud provider / region] |
| Training Duration | [Hours / days] | [Total compute time] |
| Inference Cost | [Cost per request or per 1K tokens] | [Average / peak] |
| Region / Grid Carbon Intensity | [Region name] | [gCO2/kWh] |

## Security Considerations

| Concern | Assessment | Details |
|---------|:----------:|---------|
| Known Vulnerabilities | [None / Description] | [CVE references or description of known issues] |
| Adversarial Robustness | [Low / Medium / High] | [Assessment methodology and results] |
| Prompt Injection Risk | [Low / Medium / High] | [Evaluation of susceptibility to prompt injection] |
| Data Poisoning Risk | [Low / Medium / High] | [Assessment of training data integrity] |
| Model Extraction Risk | [Low / Medium / High] | [Risk of model weights or behavior being extracted] |

## Ethical Considerations

- **Sensitive Data Used**: [Whether sensitive or personal data was used in training and how it was handled]
- **Human Subjects in Training**: [Whether human subjects were involved in data collection; IRB or ethics board review status]
- **Dual-Use Potential**: [Whether the model could be repurposed for harmful applications; safeguards in place]
- **Societal Impact Assessment**: [Broader societal implications — positive and negative]

## Limitations and Recommendations

### Known Limitations

- [Limitation 1 — e.g., poor performance on specific languages or domains]
- [Limitation 2 — e.g., context window constraints]
- [Limitation 3 — e.g., tendency to hallucinate in specific scenarios]

### Failure Modes

- [Failure mode 1 — conditions under which the model fails predictably]
- [Failure mode 2 — edge cases or adversarial inputs]

### Recommendations for Deployers

- [Recommendation 1 — e.g., implement output filtering]
- [Recommendation 2 — e.g., set up human-in-the-loop for high-risk decisions]
- [Recommendation 3 — e.g., monitor for drift over time]

---

## Approval

| Field | Value |
|-------|-------|
| Approved by | [Name] |
| Date | [YYYY-MM-DD] |
| Decision | [APPROVED / REJECTED / CONDITIONAL] |
| Conditions | [If applicable] |

<!-- Template: DevTrail | https://strangedays.tech -->
