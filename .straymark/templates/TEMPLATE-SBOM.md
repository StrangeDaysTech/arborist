---
id: SBOM-YYYY-MM-DD-NNN
title: "[System/Component] AI SBOM"
status: accepted
created: YYYY-MM-DD
agent: [agent-name]
confidence: high
review_required: false  # Factual inventory
risk_level: low
iso_42001_clause: [8]
sbom_format_reference: SPDX-3.0 | CycloneDX-1.6 | custom
system_name: ""
tags: [sbom, supply-chain]
related: []
---

# SBOM: [System/Component] AI Software Bill of Materials

## AI/ML Components

> This section maps to CycloneDX `component` with `type: machine-learning-model`.

| Component Name | Version | Provider | Type | License | Risk Level | Vulnerability Status | Last Audit Date |
|----------------|---------|----------|------|---------|------------|---------------------|-----------------|
| [Component 1] | [x.y.z] | [Provider] | model | [License] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [Component 2] | [x.y.z] | [Provider] | library | [License] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [Component 3] | [x.y.z] | [Provider] | service | [License] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |
| [Component 4] | [x.y.z] | [Provider] | dataset | [License] | [Low/Med/High] | [Clean/Vulnerable] | [YYYY-MM-DD] |

## Training Data Sources

> Aligns with ISO 42001 Annex A.7 (Data for AI Systems).

| Dataset | Source | License | PII Included | Bias Assessment Summary | Data Provenance | Retention Policy |
|---------|--------|---------|--------------|------------------------|-----------------|-----------------|
| [Dataset 1] | [Source] | [License] | [Yes/No] | [Summary] | [Provenance] | [Policy] |
| [Dataset 2] | [Source] | [License] | [Yes/No] | [Summary] | [Provenance] | [Policy] |

## Third-Party AI Services

| Service | Provider | Purpose | Data Shared | DPA in Place | SLA | Region | Compliance Certifications |
|---------|----------|---------|-------------|--------------|-----|--------|--------------------------|
| [Service 1] | [Provider] | [Purpose] | [Data types] | [Yes/No] | [SLA terms] | [Region] | [SOC2, ISO 27001, etc.] |
| [Service 2] | [Provider] | [Purpose] | [Data types] | [Yes/No] | [SLA terms] | [Region] | [SOC2, ISO 27001, etc.] |

## Software Dependencies

> Consider generating this section automatically with tools like `syft` or `trivy`.

| Package | Version | License | Known Vulnerabilities | Last Updated |
|---------|---------|---------|----------------------|-------------|
| [Package 1] | [x.y.z] | [License] | [CVE-YYYY-NNNNN, ...] | [YYYY-MM-DD] |
| [Package 2] | [x.y.z] | [License] | [None] | [YYYY-MM-DD] |
| [Package 3] | [x.y.z] | [License] | [CVE-YYYY-NNNNN] | [YYYY-MM-DD] |

## Supply Chain Risk Assessment

> Aligns with NIST AI 600-1 Category 12: Value Chain and Component Integration.

- **Overall Risk Level**: [Low/Medium/High/Critical]

- **Key Risks Identified**:
  - [Risk 1: Description]
  - [Risk 2: Description]
  - [Risk 3: Description]

- **Mitigations**:
  - [Mitigation 1: Description]
  - [Mitigation 2: Description]
  - [Mitigation 3: Description]

- **Monitoring Plan**:
  - [Monitoring activity 1: Frequency and responsible party]
  - [Monitoring activity 2: Frequency and responsible party]
  - [Monitoring activity 3: Frequency and responsible party]

<!-- Template: DevTrail | https://strangedays.tech -->
