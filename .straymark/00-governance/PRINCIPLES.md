# Guiding Principles - StrayMark

> These principles guide all documentation decisions in the project.

**Languages**: English | [Español](i18n/es/PRINCIPLES.md) | [简体中文](i18n/zh-CN/PRINCIPLES.md)

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

## 8. Cross-Source Dissonance Surfacing

> **"When two canonical sources disagree, surface before proceeding."**

When the agent detects material divergence between two canonical sources of StrayMark documentation (spec ↔ code, AILOG `§Risk` ↔ TDE backlog, ADR ↔ implementation, Charter declared scope ↔ commits, etc.), surface it before proceeding with the asked task.

StrayMark documentation is deliberately designed to make these divergences detectable: formal cross-referencing (frontmatter fields, canonical sections, stable IDs) + cultural permission to surface beyond the task. The agent's role is to consume that infrastructure and report what it sees.

See [`EMERGENT-OBSERVATION-DESIGN.md`](EMERGENT-OBSERVATION-DESIGN.md) for the meta-pattern and the pyramid of existing applications.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
