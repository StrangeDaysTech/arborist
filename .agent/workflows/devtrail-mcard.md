---
description: Create a Model/System Card (MCARD) document through an interactive step-by-step flow. Guides the user through each section with specific questions and example responses.
---

# DevTrail MCARD Skill

Create AI Model/System Card documentation through an interactive guided flow.

## Instructions

When invoked, follow these steps:

### 1. Check for Parameters

If the user specified a model name (e.g., `/devtrail-mcard GPT-4o`), use it as the model name and proceed to step 2 asking only for the model type.

If no parameter is given, proceed to step 2 asking for both model name and type.

### 2. Gather Model Identity

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  DevTrail MCARD — Model/System Card                             ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Step 1/7: Model Identity                                         ║
║                                                                   ║
║  Please provide the following:                                    ║
║                                                                   ║
║  1. Model name: [e.g., "GPT-4o", "BERT-base-uncased"]            ║
║  2. Model type:                                                   ║
║     • LLM — Large Language Model                                  ║
║     • classifier — Classification model                           ║
║     • regressor — Regression model                                ║
║     • generator — Generative model (image, audio, etc.)           ║
║     • recommender — Recommendation system                         ║
║     • other — Specify                                             ║
║  3. Provider: [e.g., "OpenAI", "Google", "Hugging Face"]          ║
║  4. Version: [e.g., "2024-05-13", "v1.0", "gpt-4o-2024-05-13"]   ║
║  5. License: [e.g., "Proprietary", "Apache 2.0", "MIT"]          ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 3. Gather Intended Use

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 2/7: Intended Use                                          ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  1. Primary intended uses:                                        ║
║     Example: "Code generation", "Customer support chatbot",       ║
║              "Sentiment analysis on product reviews"              ║
║                                                                   ║
║  2. Primary intended users:                                       ║
║     Example: "Internal engineering team", "End users via API",    ║
║              "Data science team"                                  ║
║                                                                   ║
║  3. Out-of-scope uses (what the model should NOT be used for):    ║
║     Example: "Medical diagnosis", "Legal advice",                 ║
║              "Autonomous decision-making without human review"    ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 4. Gather Training Data Details

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 3/7: Training Data                                         ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  1. Dataset name: [e.g., "Common Crawl", "Internal corpus v3"]   ║
║  2. Source: [e.g., "Web scraping", "Licensed data provider"]      ║
║  3. Size: [e.g., "1.5T tokens", "500K samples", "200GB"]         ║
║  4. Collection methodology:                                       ║
║     Example: "Web crawling with quality filters"                  ║
║  5. Preprocessing steps:                                          ║
║     Example: "Deduplication, PII removal, language filtering"     ║
║  6. Known limitations in the data:                                ║
║     Example: "English-centric, underrepresents African languages" ║
║  7. PII assessment:                                               ║
║     Example: "PII filtered using regex + NER; residual risk low"  ║
║  8. Data license: [e.g., "CC-BY-4.0", "Proprietary"]             ║
║                                                                   ║
║  Type "unknown" or "N/A" for fields you cannot fill.              ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 5. Gather Performance and Evaluation

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 4/7: Performance & Evaluation                              ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Performance Metrics (provide at least one):                      ║
║     Format: Metric | Value | Test Dataset | Conditions            ║
║     Example: "Accuracy | 92.3% | MMLU | 5-shot"                  ║
║              "F1 | 0.87 | SQuAD v2 | zero-shot"                  ║
║              "BLEU | 34.2 | WMT-22 en-de | beam search k=5"      ║
║                                                                   ║
║  Disaggregated evaluation (optional):                             ║
║     Format: Subgroup | Metric | Value                             ║
║     Example: "English | Accuracy | 95.1%"                         ║
║              "Spanish | Accuracy | 88.4%"                         ║
║                                                                   ║
║  Type "unknown" if metrics are not available.                     ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 6. Gather Bias, Security, and Ethics

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 5/7: Bias, Security & Ethics                               ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Bias & Fairness (if evaluated):                                  ║
║     Format: Group | Metric | Performance | Mitigation             ║
║     Example: "Female | Toxicity | 0.02 | Content filter applied"  ║
║     Type "not evaluated" if no bias analysis was performed.       ║
║                                                                   ║
║  Security Concerns:                                               ║
║     • Known vulnerabilities: [None / describe]                    ║
║     • Adversarial robustness: [Low / Medium / High]               ║
║     • Prompt injection risk: [Low / Medium / High / N/A]          ║
║     • Data poisoning risk: [Low / Medium / High]                  ║
║     • Model extraction risk: [Low / Medium / High]                ║
║                                                                   ║
║  Ethical Considerations:                                          ║
║     • Was sensitive data used in training? [Yes / No / Unknown]   ║
║     • Human subjects involved? [Yes / No / Unknown]               ║
║     • Dual-use potential? [Yes / No]                              ║
║     • Societal impact notes: [Brief description]                  ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 7. Gather Environmental Impact

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 6/7: Environmental Impact                                  ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  1. Training energy (kWh): [e.g., "1,287 kWh", "Unknown"]        ║
║  2. CO2 equivalent (tons): [e.g., "0.58 tCO2", "Unknown"]        ║
║  3. Hardware used: [e.g., "8x NVIDIA A100 80GB"]                  ║
║  4. Training duration: [e.g., "72 hours", "Unknown"]              ║
║  5. Inference cost: [e.g., "$0.005 per 1K tokens", "Unknown"]    ║
║  6. Region / Grid carbon intensity: [e.g., "US-East", "Unknown"] ║
║                                                                   ║
║  Type "unknown" for fields you cannot fill.                       ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 8. Gather Limitations and Recommendations

Ask the user:

```
╔══════════════════════════════════════════════════════════════════╗
║  Step 7/7: Limitations & Recommendations                         ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Known limitations:                                               ║
║     Example: "Poor performance on low-resource languages",        ║
║              "Context window limited to 128K tokens",             ║
║              "Hallucination rate ~5% on factual queries"          ║
║                                                                   ║
║  Known failure modes:                                             ║
║     Example: "Produces incorrect math on multi-step problems",    ║
║              "May refuse safe queries due to over-filtering"      ║
║                                                                   ║
║  Recommendations for deployers:                                   ║
║     Example: "Implement output filtering for PII",               ║
║              "Use human-in-the-loop for high-stakes decisions",   ║
║              "Monitor for performance drift monthly"              ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝
```

Wait for user response before proceeding.

### 9. Determine Regulatory Classifications

Based on the gathered information, determine:

- **EU AI Act risk level**: Classify based on intended use:
  - `unacceptable`: social scoring, real-time biometric identification
  - `high`: employment decisions, credit scoring, law enforcement, education
  - `limited`: chatbots, emotion recognition, deepfake generation
  - `minimal`: spam filters, game AI, search optimization
  - `not_applicable`: no EU deployment planned

- **NIST GenAI risks**: Select applicable risks from:
  `privacy`, `bias`, `confabulation`, `cbrn`, `dangerous_content`, `environmental`, `human_ai_config`, `information_integrity`, `information_security`, `intellectual_property`, `obscene_content`, `value_chain`

- **Overall risk level**: `low`, `medium`, `high`, or `critical`

### 10. Check Language Configuration

Read `.devtrail/config.yml` to determine language:

```yaml
language: en  # or es
```

Use template path based on language:
- `en` (default): `.devtrail/templates/TEMPLATE-MCARD.md`
- `es`: `.devtrail/templates/i18n/es/TEMPLATE-MCARD.md`

### 11. Generate Document ID

Determine the next sequence number:

```bash
# Get current date
date +%Y-%m-%d

# Find existing MCARD documents for today
ls .devtrail/09-ai-models/MCARD-$(date +%Y-%m-%d)-*.md 2>/dev/null | wc -l
```

ID format: `MCARD-YYYY-MM-DD-NNN`

### 12. Confirm Before Creating

Display a summary and ask for confirmation:

```
╔══════════════════════════════════════════════════════════════════╗
║  DevTrail MCARD — Summary                                       ║
╠══════════════════════════════════════════════════════════════════╣
║                                                                   ║
║  Model: [model_name] ([model_type])                               ║
║  Provider: [provider] — Version: [version]                        ║
║  EU AI Act Risk: [risk classification]                            ║
║  NIST GenAI Risks: [applicable risks]                             ║
║  Overall Risk: [risk level]                                       ║
║                                                                   ║
║  Proposed filename:                                               ║
║     MCARD-YYYY-MM-DD-NNN-[model-name-slug].md                    ║
║                                                                   ║
║  Location:                                                        ║
║     .devtrail/09-ai-models/                                       ║
║                                                                   ║
║  Review required: YES                                             ║
║                                                                   ║
╚══════════════════════════════════════════════════════════════════╝

Confirm creation? [Y/n]:
```

Wait for user confirmation before proceeding.

### 13. Load Template and Create Document

1. Read the appropriate template (EN or ES based on config)
2. Replace placeholders:
   - `YYYY-MM-DD` -> Current date
   - `NNN` -> Sequence number (001, 002, etc.)
   - `[agent-name]` -> Your agent identifier (e.g., `cursor-v1.0`, `copilot-v1.0`, `windsurf-v1.0`)
   - `[Model Name]` -> User-provided model name
3. Fill in all sections with the gathered information
4. Set `review_required: true` in the frontmatter
5. Save to `.devtrail/09-ai-models/MCARD-YYYY-MM-DD-NNN-[model-name-slug].md`

### 14. Report Result

After creation, display:

```
DevTrail MCARD created:
   .devtrail/09-ai-models/MCARD-YYYY-MM-DD-NNN-[description].md

   Model: [model_name] ([model_type])
   Provider: [provider]
   Review required: yes
   Risk level: [risk_level]
   EU AI Act: [classification]
```

## Edge Cases

1. **No `.devtrail/09-ai-models/` directory**: Create it before saving
2. **User provides partial information**: Fill known fields, mark unknown fields with `[To be determined]`
3. **User declines confirmation**: Acknowledge and exit gracefully
4. **Third-party model with limited info**: Mark unknown sections with `[Information not publicly available]` and note in limitations
5. **No `.devtrail/config.yml`**: Default to English (`en`)
