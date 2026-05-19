# Principios Guía - StrayMark

> Estos principios guían todas las decisiones de documentación en el proyecto.

**Idiomas**: [English](../../PRINCIPLES.md) | Español | [简体中文](../zh-CN/PRINCIPLES.md)

---

## 1. Trazabilidad Total

> **"Ningún cambio significativo sin un rastro documentado."**

Cada cambio que afecte la lógica de negocio, seguridad o arquitectura debe ser registrado con:
- Qué se cambió
- Por qué se cambió
- Quién (humano o agente) lo cambió
- Cuándo se cambió

---

## 2. Transparencia de Agentes IA

Los agentes de IA que trabajan en el proyecto deben:
- Identificarse claramente en cada documento que generan
- Declarar su nivel de confianza en las decisiones
- Solicitar revisión humana cuando sea apropiado
- No ocultar información relevante

---

## 3. Revisión Humana Obligatoria

Ciertos tipos de cambios **siempre** requieren aprobación humana:
- Decisiones éticas (ETH)
- Cambios críticos de seguridad
- Modificaciones irreversibles
- Decisiones con `confidence: low`

---

## 4. Documentación como Código

- Los documentos se versionan junto con el código
- Siguen convenciones de nomenclatura estrictas
- Usan formatos legibles (Markdown + YAML frontmatter)
- Pueden ser procesados automáticamente

---

## 5. Mínimo Viable, Máximo Útil

- Documentar lo necesario, no más
- Evitar duplicar información
- Mantener documentos actualizados o archivarlos
- Preferir claridad sobre exhaustividad

---

## 6. Separación de Responsabilidades

| Humanos | Agentes IA |
|---------|------------|
| Validar requisitos | Proponer requisitos |
| Aprobar decisiones éticas | Crear borradores éticos |
| Priorizar deuda técnica | Identificar deuda técnica |
| Definir arquitectura | Documentar implementación |

---

## 7. Seguridad por Defecto

- **NUNCA** documentar credenciales, tokens o secretos
- Marcar cambios de seguridad con `risk_level: high`
- Requerir revisión para cambios de autenticación/autorización
- Documentar consideraciones de privacidad (GDPR/PII)

---

## 8. Surfacing de Disonancia entre Fuentes

> **"Cuando dos fuentes canónicas se contradicen, surfacear antes de proceder."**

Cuando el agente detecte divergencia material entre dos fuentes canónicas de la documentación StrayMark (spec ↔ código, `§Risk` de AILOG ↔ TDE backlog, ADR ↔ implementación, scope declarado de Charter ↔ commits, etc.), debe surfacearla antes de proceder con la tarea pedida.

La documentación StrayMark está deliberadamente diseñada para hacer estas divergencias detectables: cross-referencing formal (campos de frontmatter, secciones canónicas, IDs estables) + permiso cultural para surfacear más allá de la tarea. El rol del agente es consumir esa infraestructura y reportar lo que ve.

Ver [`EMERGENT-OBSERVATION-DESIGN.md`](EMERGENT-OBSERVATION-DESIGN.md) para el meta-patrón y la pirámide de aplicaciones existentes.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
