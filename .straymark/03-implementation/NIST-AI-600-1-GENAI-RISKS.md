# NIST AI 600-1 --- Generative AI Risk Categories Reference

> **Standard**: NIST AI 600-1 --- Artificial Intelligence Risk Management Framework: Generative AI Profile
>
> NIST AI 600-1 defines 12 risk categories specific to generative AI systems. This reference maps each category to DevTrail templates, frontmatter values, and practical mitigations. Use it alongside the NIST AI RMF function guides to ensure comprehensive GenAI risk coverage.

---

## 1. CBRN Information

**Identifier**: `cbrn`

Generative AI systems may produce information that could assist in the creation, acquisition, or deployment of chemical, biological, radiological, or nuclear (CBRN) weapons or materials.

- A large language model provides step-by-step synthesis instructions for controlled chemical substances
- A code generation model produces functional exploit code targeting critical infrastructure control systems
- An AI assistant responds to queries about biological agent cultivation with actionable detail

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [cbrn]` |
| Key sections | Risk Assessment, Recommendations |
| Supporting document | SEC (output filtering controls) |

### Recommended Mitigations

- Implement output filtering and content safety classifiers that detect CBRN-related content
- Restrict model capabilities in domains adjacent to CBRN through fine-tuning or system prompts
- Establish human review workflows for queries that trigger CBRN safety flags

---

## 2. Confabulation

**Identifier**: `confabulation`

Generative AI systems may produce outputs that are factually incorrect, fabricated, or inconsistent with training data while presenting them with high apparent confidence.

- A summarization model fabricates citations that do not exist in the source material
- A code generation model invents API methods that are not part of the referenced library
- A conversational AI provides medical information that contradicts established clinical guidelines

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [confabulation]` |
| Key sections | Risk Assessment, Transparency, Recommendations |
| Supporting document | MCARD (Limitations section), TES (accuracy tests) |

### Recommended Mitigations

- Implement retrieval-augmented generation (RAG) to ground outputs in verified sources
- Add confidence indicators and source attribution to model outputs
- Create TES documents with factual accuracy benchmarks and hallucination detection tests

---

## 3. Dangerous, Violent, or Hateful Content

**Identifier**: `dangerous_content`

Generative AI systems may generate content that promotes, glorifies, or provides instructions for violence, self-harm, or hatred targeting individuals or groups.

- A text generation model produces content that incites violence against a specific group
- An image generation model creates realistic depictions of graphic violence on request
- A chatbot provides detailed instructions for self-harm when prompted by a vulnerable user

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [dangerous_content]` |
| Key sections | Risk Assessment, Bias Evaluation, Recommendations |
| Supporting document | SEC (content moderation controls) |

### Recommended Mitigations

- Deploy content safety classifiers on both inputs and outputs
- Implement escalation protocols for detected harmful content attempts
- Test content filtering effectiveness with adversarial red-teaming (documented in TES)

---

## 4. Data Privacy

**Identifier**: `privacy`

Generative AI systems may memorize, leak, or inadvertently reveal personal data, sensitive information, or private details from training data or user interactions.

- A language model reproduces verbatim passages containing personal identifiable information from its training data
- A conversational AI retains and surfaces information from one user's session to another
- A model trained on proprietary data leaks trade secrets through carefully crafted prompts

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH, DPIA |
| Frontmatter value | `nist_genai_risks: [privacy]` |
| Key sections | Data Privacy section (ETH), full DPIA |
| Supporting document | SEC (access controls, data isolation) |

### Recommended Mitigations

- Conduct a DPIA before deploying systems trained on or processing personal data
- Implement differential privacy techniques or data sanitization in training pipelines
- Test for memorization and data extraction vulnerabilities (documented in TES)

---

## 5. Environmental Impacts

**Identifier**: `environmental`

The training, fine-tuning, and inference of generative AI models consume substantial computational resources, contributing to energy use, carbon emissions, and electronic waste.

- Training a large foundation model consumes energy equivalent to hundreds of households for a year
- Frequent retraining cycles multiply the environmental cost without proportional capability gains
- Deploying inference at scale generates sustained energy consumption across data center infrastructure

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [environmental]` |
| Key sections | Environmental Impact section |
| Supporting document | MCARD (compute requirements), AI-KPIS.md (efficiency metrics) |

### Recommended Mitigations

- Document compute requirements and energy estimates in the MCARD
- Track and report carbon footprint metrics in AI-KPIS.md
- Evaluate model efficiency alternatives (smaller models, distillation, quantization) in ADR documents

---

## 6. Harmful Bias and Homogenization

**Identifier**: `bias`

Generative AI systems may amplify societal biases present in training data, producing outputs that discriminate against or misrepresent specific groups. Homogenization occurs when widespread AI use reduces diversity of thought and expression.

- An image generation model consistently underrepresents certain demographic groups in professional settings
- A text model associates negative stereotypes with specific nationalities or genders
- Widespread use of a single AI writing assistant homogenizes communication styles across an industry

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [bias]` |
| Key sections | Bias Evaluation, Social Impact, Recommendations |
| Supporting document | TES (fairness tests), MCARD (training data documentation) |

### Recommended Mitigations

- Conduct bias evaluations across relevant demographic dimensions (documented in ETH Bias Evaluation)
- Implement fairness metrics and test for disparate impact in TES documents
- Document training data composition and known representational gaps in the MCARD

---

## 7. Human-AI Configuration

**Identifier**: `human_ai_config`

Risks arising from inappropriate levels of human oversight, overreliance on AI outputs, automation bias, or poor configuration of human-AI interaction boundaries.

- Operators rubber-stamp AI recommendations without meaningful review due to automation bias
- A system is deployed without clear escalation paths for cases where human judgment is needed
- Users develop excessive trust in AI outputs after prolonged interaction without encountering errors

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [human_ai_config]` |
| Key sections | Risk Assessment, Recommendations |
| Supporting document | AGENT-RULES.md (autonomy limits), MCARD (intended use boundaries) |

### Recommended Mitigations

- Define explicit human oversight requirements in AGENT-RULES.md for each autonomy level
- Document intended human-AI interaction patterns in the MCARD Intended Use section
- Implement forced human review checkpoints for high-stakes decisions (documented in ADR)

---

## 8. Information Integrity

**Identifier**: `information_integrity`

Generative AI can be used to create or amplify misinformation, disinformation, and manipulated media, undermining public trust and the integrity of information ecosystems.

- A model generates highly convincing but fabricated news articles indistinguishable from legitimate journalism
- AI-generated deepfake media is used to impersonate public figures for disinformation campaigns
- Automated content generation floods information channels with low-quality, misleading content at scale

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [information_integrity]` |
| Key sections | Risk Assessment, Transparency, Social Impact |
| Supporting document | SEC (provenance controls), MCARD (output watermarking) |

### Recommended Mitigations

- Implement content provenance and watermarking for AI-generated outputs (document approach in ADR)
- Establish usage policies that prohibit deceptive use and document them in AI-GOVERNANCE-POLICY.md
- Deploy detection mechanisms for manipulated or AI-generated content in downstream systems

---

## 9. Information Security

**Identifier**: `information_security`

Generative AI systems introduce novel attack surfaces including prompt injection, model extraction, training data poisoning, and adversarial inputs that bypass safety controls.

- An attacker uses prompt injection to override system instructions and extract sensitive configuration
- A model is manipulated through adversarial inputs to produce outputs that bypass content filters
- Training data poisoning causes a model to produce subtly incorrect outputs in specific domains

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | SEC |
| Frontmatter value | `nist_genai_risks: [information_security]` |
| Key sections | Threat Model, Controls Implemented, Vulnerabilities |
| Supporting document | ETH (Risk Assessment), TES (security tests) |

### Recommended Mitigations

- Create SEC documents with threat models specific to GenAI attack vectors (prompt injection, extraction, poisoning)
- Implement input validation and output sanitization controls documented in SEC
- Conduct adversarial red-teaming exercises and document results in TES

---

## 10. Intellectual Property

**Identifier**: `intellectual_property`

Generative AI may infringe on intellectual property rights by reproducing copyrighted material, generating outputs substantially similar to protected works, or using proprietary data without authorization.

- A code generation model reproduces verbatim sections of copyrighted source code from its training data
- An image generation model produces outputs that closely replicate the distinctive style of a living artist
- A model trained on proprietary corporate documents generates outputs that reveal protected trade secrets

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [intellectual_property]` |
| Key sections | Risk Assessment, Recommendations |
| Supporting document | SBOM (training data provenance, license compliance) |

### Recommended Mitigations

- Document training data sources and their license terms in the SBOM
- Implement output similarity detection against known copyrighted works
- Establish IP review workflows for AI-generated content used in commercial applications (documented in ADR)

---

## 11. Obscene or Degrading Content

**Identifier**: `obscene_content`

Generative AI systems may produce sexually explicit, obscene, or degrading content, either through direct generation or by exploiting weaknesses in content filters.

- An image generation model is manipulated through prompt engineering to bypass safety filters and produce explicit content
- A text model generates degrading descriptions of individuals based on their demographic characteristics
- An AI system produces non-consensual intimate imagery by combining publicly available photographs with generative techniques

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | ETH |
| Frontmatter value | `nist_genai_risks: [obscene_content]` |
| Key sections | Risk Assessment, Recommendations |
| Supporting document | SEC (content filtering controls) |

### Recommended Mitigations

- Deploy multi-layered content safety filters on both inputs and outputs
- Implement robust content classifiers that resist common jailbreaking techniques
- Conduct red-teaming exercises specifically targeting content filter bypasses (documented in TES)

---

## 12. Value Chain and Component Integration

**Identifier**: `value_chain`

Risks arising from the integration of third-party AI components, models, datasets, and services into larger systems, where upstream changes, vulnerabilities, or quality issues propagate downstream.

- A third-party embedding model introduces a subtle bias that propagates through all downstream applications
- An API provider changes model behavior without notice, breaking assumptions in dependent systems
- A fine-tuned model inherits undisclosed vulnerabilities from its base model

### DevTrail Mapping

| Aspect | Value |
|--------|-------|
| Primary template | SBOM |
| Frontmatter value | `nist_genai_risks: [value_chain]` |
| Key sections | Third-Party Services, Components, Dependencies |
| Supporting document | ETH (third-party risk), SEC (supply chain security) |

### Recommended Mitigations

- Maintain comprehensive SBOM documents covering all AI components in the value chain
- Pin model versions and document expected behavior baselines in MCARD
- Establish contractual requirements for change notification from AI service providers (documented in ADR)

---

## Summary: GenAI Risk Categories to DevTrail Mapping

| Category | Identifier | Primary DevTrail Template | Frontmatter Value |
|----------|-----------|---------------------------|-------------------|
| CBRN Information | `cbrn` | ETH | `nist_genai_risks: [cbrn]` |
| Confabulation | `confabulation` | ETH | `nist_genai_risks: [confabulation]` |
| Dangerous/Violent/Hateful Content | `dangerous_content` | ETH | `nist_genai_risks: [dangerous_content]` |
| Data Privacy | `privacy` | ETH, DPIA | `nist_genai_risks: [privacy]` |
| Environmental Impacts | `environmental` | ETH | `nist_genai_risks: [environmental]` |
| Harmful Bias and Homogenization | `bias` | ETH | `nist_genai_risks: [bias]` |
| Human-AI Configuration | `human_ai_config` | ETH | `nist_genai_risks: [human_ai_config]` |
| Information Integrity | `information_integrity` | ETH | `nist_genai_risks: [information_integrity]` |
| Information Security | `information_security` | SEC | `nist_genai_risks: [information_security]` |
| Intellectual Property | `intellectual_property` | ETH | `nist_genai_risks: [intellectual_property]` |
| Obscene/Degrading Content | `obscene_content` | ETH | `nist_genai_risks: [obscene_content]` |
| Value Chain and Component Integration | `value_chain` | SBOM | `nist_genai_risks: [value_chain]` |

---

*NIST AI 600-1 Generative AI Risk Categories Reference --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
