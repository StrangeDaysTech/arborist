# Guía de Implementación TC260 — StrayMark

> Guía práctica para completar un documento TC260RA bajo el **AI Safety Governance Framework v2.0** (TC260, 15-sep-2025).

**Languages**: [English](../../TC260-IMPLEMENTATION-GUIDE.md) | Español | [简体中文](../zh-CN/TC260-IMPLEMENTATION-GUIDE.md)

---

## Cuándo Crear un TC260RA

Cree un TC260 Risk Assessment cuando `regional_scope` incluya `china` y se cumpla cualquiera de:

- El sistema desplegado o modificado es o incluye un modelo de IA.
- La aplicación es para usuarios en China continental, o el operador está incorporado en China continental.
- El sistema tiene flujos de datos transfronterizos que tocan China continental.
- Se anticipa una nueva versión de modelo, escenario, o salto de escala.

El TC260RA complementa la clasificación EU AI Act en ETH y se referencia vía `related: [TC260RA-...]` desde la ETH / MCARD / AILOG correspondiente.

---

## Los Tres Criterios

TC260 v2.0 evalúa el riesgo en **tres ejes ortogonales** y los compone en un único nivel (low / medium / high / very_high / extremely_severe).

### 1. Escenario de Aplicación (`tc260_application_scenario`)

| Escenario | Ejemplos | Nivel mínimo inherente |
|-----------|----------|------------------------|
| `public_service` | Chatbots gubernamentales | medium |
| `healthcare` | Soporte clínico, imagen médica | high |
| `finance` | Credit scoring, KYC, fraude | high |
| `safety_critical` | Conducción autónoma, control industrial | very_high |
| `content_generation` | Síntesis de texto/imagen/vídeo | medium |
| `social` | Recomendación, ranking, citas | medium |
| `industrial_control` | OT, robótica | very_high |

### 2. Nivel de Inteligencia (`tc260_intelligence_level`)

| Nivel | Definición |
|-------|-----------|
| `narrow` | Modelo de propósito único, salida determinista |
| `foundation` | Modelo fundacional general (LLM, visión-lenguaje), sin uso de herramientas |
| `agentic` | Modelo fundacional + uso autónomo de herramientas |
| `general` | Aproximándose a AGI |

### 3. Escala de Aplicación (`tc260_application_scale`)

| Escala | Definición |
|--------|-----------|
| `individual` | Usuario único / equipo pequeño |
| `organization` | Una sola organización |
| `societal` | Porción significativa del público (≥ 1M usuarios) |
| `cross_border` | Opera entre China continental y otras jurisdicciones |

---

## Composición del Nivel

No hay fórmula numérica publicada. Use la matriz como punto de partida y documente el razonamiento.

| Escenario \ Inteligencia | Narrow | Foundation | Agentic | General |
|--------------------------|--------|-----------|---------|---------|
| public_service | low → medium | medium | high | very_high |
| healthcare / finance | medium | high | high | very_high |
| safety_critical | high | very_high | very_high | extremely_severe |
| content_generation | low | medium | high | very_high |
| social | low | medium | high | very_high |
| industrial_control | high | very_high | very_high | extremely_severe |

**Modificador por escala**:
- `individual`, `organization` → sin cambio.
- `societal` → sube un nivel.
- `cross_border` → sube un nivel **y** requiere análisis explícito de transferencia transfronteriza (ver PIPL-PIPIA-GUIDE).

---

## Taxonomía de Riesgos

### Endógenos (`tc260_endogenous_risks`)

Inherentes al modelo: `hallucination`, `bias`, `robustness`, `data_leakage`, `prompt_injection`, `model_extraction`.

### De Aplicación (`tc260_application_risks`)

Del uso técnico: `misuse`, `scope_creep`, `dependency`, `availability`, `integration_flaw`.

### Derivados (`tc260_derivative_risks`)

Efectos sociales de segundo orden: `labor_displacement`, `opinion_shaping`, `ecosystem_disruption`, `monoculture`, `loss_of_skill`.

Para `very_high` y `extremely_severe`, v2.0 requiere explícitamente **monitoreo de riesgo catastrófico**: documente esto en la sección 5 (Medidas de Gobernanza) del TC260RA.

---

## Vinculación desde Otros Documentos

Cuando se setea `tc260_risk_level: high` (o superior) en un documento que no es TC260RA, la regla **CROSS-004** exige `review_required: true`. El propio TC260RA debe vincularse vía `related:`:

```yaml
related:
  - TC260RA-2026-04-25-001
  - MCARD-2026-04-25-001
  - PIPIA-2026-04-25-001
```

---

## Cadencia de Revisión

| Disparador | Acción |
|-----------|--------|
| Cambio de versión del modelo | Re-ejecutar la sección 4 (contramedidas) |
| Expansión de escenario | Re-clasificar escenario × inteligencia × escala |
| Cruce de tier de escala (ej. 1M usuarios) | Revisión del nivel |
| Actualización regulatoria de TC260 | Revisión completa |

<!-- StrayMark | https://strangedays.tech -->
