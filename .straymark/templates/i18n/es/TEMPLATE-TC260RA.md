---
id: TC260RA-YYYY-MM-DD-NNN
title: "[Sistema] Evaluación de Riesgos TC260"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true

# --- Workflow de aprobación (opcional, llenar al momento de la revisión) ---
# reviewed_by: <id-revisor>            # email | usuario-github | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
tc260_application_scenario: null  # ej: public_service | healthcare | finance | content_generation | social | safety_critical
tc260_intelligence_level: null    # narrow | foundation | agentic | general
tc260_application_scale: null     # individual | organization | societal | cross_border
tc260_risk_level: not_applicable  # low | medium | high | very_high | extremely_severe | not_applicable
tc260_endogenous_risks: []
tc260_application_risks: []
tc260_derivative_risks: []
iso_42001_clause: [6]
tags: [china, tc260, risk-assessment]
related: []
---

# TC260RA: [Sistema] Evaluación de Riesgos TC260

> **IMPORTANTE**: Borrador creado por un agente de IA. Requiere revisión y aprobación humana.
>
> Alineado con el **AI Safety Governance Framework v2.0** publicado por TC260 (Comité Técnico Nacional de Estandarización de Seguridad de la Información) el 15 de septiembre de 2025.

## 1. Mapeo a los Cuatro Pilares

Mostrar cómo el sistema cubre cada pilar:

| Pilar | Cobertura |
|-------|-----------|
| **Principios de gobernanza** (centrado en personas, IA para el bien, seguro y controlable) | [Declaración] |
| **Taxonomía de riesgos** (endógeno técnico / aplicación / derivado) | [Ver sección 3] |
| **Contramedidas técnicas** | [Ver sección 4] |
| **Medidas de gobernanza** (implementación organizacional) | [Ver sección 5] |

## 2. Clasificación por Tres Criterios (Sección 5.5 / Apéndice 1)

El nivel de riesgo se compone de **escenario de aplicación × nivel de inteligencia × escala de aplicación**.

### 2.1 Escenario de Aplicación

- **Seleccionado**: [public_service / healthcare / finance / content_generation / social / safety_critical / industrial_control / other]
- **Justificación**: [Por qué aplica este escenario]

### 2.2 Nivel de Inteligencia

| Nivel | Definición | Este sistema |
|-------|-----------|-------------|
| Narrow | Modelo de propósito único | [ ] |
| Foundation | Modelo fundacional general (LLM, visión-lenguaje) | [ ] |
| Agentic | Modelo fundacional + uso autónomo de herramientas | [ ] |
| General | Aproximándose a AGI | [ ] |

- **Seleccionado**: [narrow / foundation / agentic / general]

### 2.3 Escala de Aplicación

| Escala | Definición | Este sistema |
|--------|-----------|-------------|
| Individual | Usuario único / equipo pequeño | [ ] |
| Organization | Despliegue en una sola organización | [ ] |
| Societal | Afecta a una porción significativa del público | [ ] |
| Cross-border | Opera entre China continental y otras jurisdicciones | [ ] |

- **Seleccionado**: [individual / organization / societal / cross_border]

### 2.4 Nivel de Riesgo Resultante

| Nivel | Descripción |
|-------|-------------|
| Low | Daño esperado mínimo; controles estándar suficientes |
| Medium | Daño previsible y contenido; revisión y contramedidas básicas |
| High | Riesgo significativo para individuos o grupos específicos |
| Very High | Riesgo para la estabilidad social o grandes poblaciones |
| Extremely Severe | Riesgo de daño catastrófico o sistémico |

- **Nivel computado**: [low / medium / high / very_high / extremely_severe]
- **Justificación**: [Razonamiento combinando escenario × inteligencia × escala]

## 3. Taxonomía de Riesgos

### 3.1 Riesgos Endógenos Técnicos

> Inherentes al modelo: vulnerabilidades, sesgos, alucinación, robustez.

| Riesgo | Descripción | Probabilidad | Severidad | Mitigación |
|--------|-------------|-------------|-----------|-----------|
| [Riesgo 1] | [Descripción] | [B/M/A] | [B/M/A] | [Medida] |

### 3.2 Riesgos de Aplicación

> De cómo se aplica técnicamente: uso indebido, scope creep, dependencia.

| Riesgo | Descripción | Probabilidad | Severidad | Mitigación |
|--------|-------------|-------------|-----------|-----------|
| [Riesgo 1] | [Descripción] | [B/M/A] | [B/M/A] | [Medida] |

### 3.3 Riesgos Derivados

> Efectos sociales de segundo orden: desplazamiento laboral, modelado de opinión, disrupción del ecosistema.

| Riesgo | Descripción | Probabilidad | Severidad | Mitigación |
|--------|-------------|-------------|-----------|-----------|
| [Riesgo 1] | [Descripción] | [B/M/A] | [B/M/A] | [Medida] |

## 4. Contramedidas Técnicas

| ID Riesgo | Contramedida | Responsable | Verificación |
|-----------|--------------|-------------|-------------|
| [E.1] | [Control] | [Rol] | [Test/métrica] |
| [A.1] | [Control] | [Rol] | [Test/métrica] |
| [D.1] | [Control] | [Rol] | [Test/métrica] |

## 5. Medidas de Gobernanza

- **Responsable designado**: [Rol / Persona]
- **Cadencia de reporte interno**: [Mensual / trimestral]
- **Ruta de escalado**: [A quién y bajo qué disparadores]
- **Componentes open-source**: [Si embebe IA OSS: gobernanza per cláusulas v2.0 OSS]
- **Vigilancia de riesgo catastrófico**: [Requerido para very_high / extremely_severe]

## 6. Monitoreo y Revisión

- **Próxima revisión**: [YYYY-MM-DD]
- **Disparadores**: [Cambio de versión del modelo / expansión de escenario / salto de escala / actualización regulatoria]
- **Documentos vinculados**: [ETH-..., MCARD-..., AILABEL-..., CACFILE-...]

## Aprobación

| Aprobado por | Fecha | Decisión | Condiciones |
|--------------|-------|----------|-------------|
| [Revisor] | [YYYY-MM-DD] | [aprobado / condicional / rechazado] | [Si aplica] |

<!-- Template: StrayMark | https://strangedays.tech -->
