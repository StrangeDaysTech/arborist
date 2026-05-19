---
name: straymark-sec
description: Create a StrayMark Security Assessment (SEC) document. Interactive step-by-step flow that gathers scope, threat model methodology, and OWASP ASVS level, then generates a pre-filled SEC document based on code context. Always marks as draft with review_required.
allowed-tools: Read, Write, Glob, Bash(git diff *, git log *, git status *, date *, wc *)
---

# StrayMark Security Assessment Skill

Create a Security Assessment document with threat modeling and OWASP ASVS compliance checks.

## Instructions

When invoked via `/straymark-sec [component-name]`, follow these steps:

### 1. Gather Assessment Scope

If the user provided a `[component-name]`, use it as the system/component under assessment.

If no component was specified, **ask the user**:

```
What system or component would you like to assess?
Examples: "authentication API", "payment gateway", "user data pipeline"
```

Wait for the user to respond before proceeding.

### 2. Ask Threat Model Methodology

Present the available methodologies and ask the user to choose:

```
Which threat modeling methodology would you like to use?

  1. STRIDE (recommended) - Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege
  2. PASTA - Process for Attack Simulation and Threat Analysis
  3. LINDDUN - Linkability, Identifiability, Non-repudiation, Detectability, Disclosure, Unawareness, Non-compliance
  4. Custom - Define your own methodology

Select [1/2/3/4] (default: 1):
```

Wait for the user to respond before proceeding.

### 3. Ask Target OWASP ASVS Level

Ask the user to select the verification level:

```
Which OWASP ASVS verification level should be targeted?

  L1 - Opportunistic: Basic security controls (default for most apps)
  L2 - Standard: For apps handling sensitive data
  L3 - Advanced: For critical infrastructure, medical, financial systems

Select [L1/L2/L3] (default: L1):
```

Wait for the user to respond before proceeding.

### 4. Analyze Code Context

Gather information about the component:

```bash
# Get current date
date +%Y-%m-%d

# Get modified files (staged and unstaged)
git status --porcelain

# Get recent changes summary
git diff --stat HEAD~1 2>/dev/null || git diff --stat

# Count lines changed
git diff --numstat HEAD~1 2>/dev/null || git diff --numstat
```

Also scan the codebase for security-relevant patterns in the component area:
- Authentication/authorization code
- Input validation
- Cryptographic operations
- Network/API endpoints
- Data storage and access patterns
- Dependency files (package.json, Cargo.toml, requirements.txt, etc.)

### 5. Confirm with User

**Always display this confirmation before creating:**

```
+------------------------------------------------------------------+
|  StrayMark Security Assessment                                      |
+------------------------------------------------------------------+
|                                                                    |
|  Component: [component-name]                                       |
|  Methodology: [STRIDE / PASTA / LINDDUN / custom]                 |
|  ASVS Level: [L1 / L2 / L3]                                       |
|                                                                    |
|  Code Context:                                                     |
|     Files scanned: [N]                                             |
|     Security-relevant patterns: [list]                             |
|                                                                    |
|  Proposed filename:                                                |
|     SEC-YYYY-MM-DD-NNN-[description].md                            |
|                                                                    |
+------------------------------------------------------------------+

Confirm creation? [Y/n]:
```

Wait for user confirmation before proceeding.

### 6. Check Language Configuration

Read `.straymark/config.yml` to determine language:

```yaml
language: en  # or es
```

Use template path based on language:
- `en` (default): `.straymark/templates/TEMPLATE-SEC.md`
- `es`: `.straymark/templates/i18n/es/TEMPLATE-SEC.md`

### 7. Generate Document ID

Determine the next sequence number:

```bash
# Find existing SEC documents for today
ls .straymark/08-security/SEC-$(date +%Y-%m-%d)-*.md 2>/dev/null | wc -l
```

ID format: `SEC-YYYY-MM-DD-NNN`

### 8. Load Template and Create Document

1. Read the appropriate template
2. Replace placeholders:
   - `YYYY-MM-DD` -> Current date
   - `NNN` -> Sequence number (001, 002, etc.)
   - `[agent-name]` -> `claude-code-v1.0`
   - `[System/Component]` -> The component name from step 1
   - `threat_model_methodology: STRIDE` -> The methodology chosen in step 2
   - `owasp_asvs_level: 1` -> The ASVS level chosen in step 3
3. Pre-fill fields based on code context analysis:
   - Populate "Scope and Objectives" with detected component boundaries
   - Fill threat model rows with identified threats from code patterns
   - Mark relevant OWASP ASVS controls based on detected patterns
   - List discovered vulnerabilities or potential weaknesses
4. Ensure frontmatter contains:
   - `status: draft`
   - `review_required: true`
   - `risk_level: high`
   - `confidence: medium`
5. Save to: `.straymark/08-security/SEC-YYYY-MM-DD-NNN-[description].md`

### 9. Report Result

After creation, display:

```
StrayMark Security Assessment created:
   .straymark/08-security/SEC-YYYY-MM-DD-NNN-description.md

   Status: draft
   Review required: yes
   Risk level: high
   Methodology: [STRIDE / PASTA / LINDDUN / custom]
   ASVS Level: [L1 / L2 / L3]

   IMPORTANT: This is a DRAFT. A qualified security reviewer must
   validate all findings before any remediation decisions are made.
```

## Expected Output Example

For invocation `/straymark-sec authentication-api`, after the interactive flow selects STRIDE and L1, the generated document would look like:

```markdown
---
id: SEC-2026-03-24-001
title: "Authentication API Security Assessment"
status: draft
created: 2026-03-24
agent: claude-code-v1.0
confidence: medium
review_required: true
risk_level: high
eu_ai_act_risk: not_applicable
iso_42001_clause: [6, 8]
threat_model_methodology: STRIDE
owasp_asvs_level: 1
tags: [security]
related: []
---

# SEC: Authentication API Security Assessment

> **IMPORTANT**: This document is a DRAFT created by an AI agent.
> It requires human review and approval before proceeding.

## Scope and Objectives

| Field | Value |
|-------|-------|
| System Under Assessment | Authentication API v1.0 |
| Assessment Type | code review / threat model |
| Assessment Date | 2026-03-24 |
| Assessor | claude-code-v1.0 |

**Objectives**:

- Identify authentication and session management vulnerabilities
- Validate input handling on login and token endpoints

**In Scope**:

- /api/auth/* endpoints
- JWT token generation and validation
- Password hashing and storage

**Out of Scope**:

- Frontend client code
- Third-party OAuth providers

## Threat Model

> Methodology: **STRIDE**

### Spoofing

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| S-001 | Credential stuffing on login endpoint | 4 | 4 | 16 | Rate limiting, account lockout |
| S-002 | JWT token forgery via weak signing key | 2 | 5 | 10 | Use RS256 with rotated keys |

### Tampering

| Threat ID | Description | Likelihood (1-5) | Impact (1-5) | Risk Score | Mitigation |
|-----------|-------------|:-----------------:|:------------:|:----------:|------------|
| T-001 | Parameter manipulation on password reset | 3 | 4 | 12 | Server-side validation, signed tokens |

(... remaining STRIDE categories ...)

## OWASP ASVS Compliance

| Control ID | Description | Level (L1/L2/L3) | Status | Evidence | Notes |
|------------|-------------|:-----------------:|:------:|----------|-------|
| V2.1.1 | Verify password length >= 12 chars | L1 | Fail | src/auth/validation.rs:45 | Min length set to 8 |
| V3.1.1 | Verify session token generation uses CSPRNG | L1 | Pass | src/auth/session.rs:12 | Uses ring::rand |

(... remaining relevant controls ...)

## Vulnerabilities Found

| Vuln ID | CWE | Severity (CVSS) | Description | Affected Component | Remediation | Status |
|---------|-----|:----------------:|-------------|-------------------|-------------|:------:|
| VULN-001 | CWE-521 | 5.3 | Weak password policy (min 8 chars) | auth/validation.rs | Enforce min 12 chars | open |

## Security Controls

| Business Function | Practice | Maturity Level (1-3) | Current Status | Gaps |
|-------------------|----------|:--------------------:|----------------|------|
| Implementation | Secure Build | 1 | Basic dependency scanning | No SAST configured |

## Recommendations

| Priority | Description | Effort | Impact |
|:--------:|-------------|:------:|:------:|
| High | Increase minimum password length to 12 characters | Low | Medium |
| High | Implement rate limiting on authentication endpoints | Medium | High |
| Medium | Add SAST to CI pipeline | Medium | Medium |

---

## Approval

| Field | Value |
|-------|-------|
| Approved by | [Name] |
| Date | [YYYY-MM-DD] |
| Decision | [APPROVED / REJECTED / CONDITIONAL] |
| Conditions | [If applicable] |

<!-- Template: StrayMark | https://strangedays.tech -->
```

## Edge Cases

1. **No git repository**: Inform user that git is required for context analysis
2. **Component not found in codebase**: Proceed with a generic assessment, note limited code context
3. **User declines**: Acknowledge and exit gracefully
4. **No security-relevant code found**: Generate template with empty threat rows for manual completion
5. **Multiple components match**: Ask user to clarify which specific component to assess
