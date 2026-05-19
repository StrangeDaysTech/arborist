# Guiding Principles - DevTrail

> These principles guide all documentation decisions in the project.

---

## 1. Total Traceability

> **"No significant change without a documented trace."**

Every change that affects business logic, security, or architecture must be recorded with:
- What was changed
- Why it was changed
- Who (human or agent) changed it
- When it was changed

---

## 2. AI Agent Transparency

AI agents working on the project must:
- Clearly identify themselves in every document they generate
- Declare their confidence level in decisions
- Request human review when appropriate
- Not hide relevant information

---

## 3. Mandatory Human Review

Certain types of changes **always** require human approval:
- Ethical decisions (ETH)
- Critical security changes
- Irreversible modifications
- Decisions with `confidence: low`

---

## 4. Documentation as Code

- Documents are versioned together with the code
- They follow strict naming conventions
- They use readable formats (Markdown + YAML frontmatter)
- They can be processed automatically

---

## 5. Minimum Viable, Maximum Useful

- Document what is necessary, no more
- Avoid duplicating information
- Keep documents updated or archive them
- Prefer clarity over exhaustiveness

---

## 6. Separation of Responsibilities

| Humans | AI Agents |
|--------|-----------|
| Validate requirements | Propose requirements |
| Approve ethical decisions | Create ethical drafts |
| Prioritize technical debt | Identify technical debt |
| Define architecture | Document implementation |

---

## 7. Security by Default

- **NEVER** document credentials, tokens, or secrets
- Mark security changes with `risk_level: high`
- Require review for authentication/authorization changes
- Document privacy considerations (GDPR/PII)

---

*DevTrail v1.0.0 | [Strange Days Tech](https://strangedays.tech)*
