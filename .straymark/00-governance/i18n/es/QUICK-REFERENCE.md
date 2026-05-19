# StrayMark - Referencia Rápida

> Referencia de una página para agentes IA y desarrolladores.
>
> **Este es un documento derivado** — DOCUMENTATION-POLICY.md es la fuente autoritativa.

**Idiomas**: [English](../../QUICK-REFERENCE.md) | Español | [简体中文](../zh-CN/QUICK-REFERENCE.md)

---

## Configuración de Idioma

**Archivo**: `.straymark/config.yml`

```yaml
language: en  # Opciones: en, es (por defecto: en)
```

| Idioma | Ruta de Templates |
|--------|-------------------|
| `en` | `.straymark/templates/TEMPLATE-*.md` |
| `es` | `.straymark/templates/i18n/es/TEMPLATE-*.md` |

---

## Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

**Ejemplo**: `AILOG-2026-03-25-001-implementar-oauth.md`

---

## Tipos de Documentos (12)

### Tipos Base (8)

| Tipo | Nombre | Carpeta | Autonomía del Agente |
|------|--------|---------|---------------------|
| `AILOG` | Log de Acción IA | `07-ai-audit/agent-logs/` | Crear libremente |
| `AIDEC` | Decisión IA | `07-ai-audit/decisions/` | Crear libremente |
| `ETH` | Revisión Ética | `07-ai-audit/ethical-reviews/` | Solo borrador |
| `ADR` | Decisión de Arquitectura | `02-design/decisions/` | Requiere revisión |
| `REQ` | Requisito | `01-requirements/` | Proponer |
| `TES` | Plan de Pruebas | `04-testing/` | Proponer |
| `INC` | Post-mortem de Incidente | `05-operations/incidents/` | Contribuir |
| `TDE` | Deuda Técnica | `06-evolution/technical-debt/` | Identificar |

### Tipos Extendidos (4)

| Tipo | Nombre | Carpeta | Autonomía del Agente |
|------|--------|---------|---------------------|
| `SEC` | Evaluación de Seguridad | `08-security/` | Borrador → aprobación (siempre) |
| `MCARD` | Tarjeta de Modelo/Sistema | `09-ai-models/` | Borrador → aprobación (siempre) |
| `SBOM` | Lista de Materiales de Software | `07-ai-audit/` | Crear libremente |
| `DPIA` | Evaluación de Impacto en Protección de Datos | `07-ai-audit/ethical-reviews/` | Borrador → aprobación (siempre) |

### Unidades de Trabajo Acotadas — Charter

Los Charters **no** son tipos de documento — envuelven un bloque de implementación multi-sesión. El nombre de archivo usa prefijo secuencial (`NN-slug.md`), no prefijo de fecha. Ciclo de vida: `declared` → `in-progress` → `closed`.

| Concepto | Carpeta | Autonomía del Agente |
|----------|---------|---------------------|
| `Charter` | `.straymark/charters/` (declarativo `NN-slug.md` + telemetría `NN-slug.telemetry.yaml`) | Andamiar vía `charter new`; el operador es dueño del trigger y de las transiciones de ciclo de vida |

> Ver sección 15 de `STRAYMARK.md` y `.straymark/00-governance/SPECKIT-CHARTER-BRIDGE.md` para heurísticas de granularidad, ciclo de vida y el puente SpecKit ↔ Charter.

---

## Cuándo Documentar

| Situación | Acción |
|-----------|--------|
| Código complejo (`straymark analyze`; alternativa: >20 líneas) | AILOG |
| Decisión entre alternativas | AIDEC |
| Cambios en auth/autorización/PII | AILOG + `risk_level: high` + ETH |
| Cambios en API pública o esquema de BD | AILOG + considerar ADR |
| Cambios en modelos ML/prompts | AILOG + revisión humana |
| Cambios en dependencias críticas de seguridad | AILOG + revisión humana |
| Cambios en instrumentación OTel | AILOG + tag `observabilidad` |
| Bloque de implementación multi-sesión (>1 día, >5 tareas en varias fases) | Declarar un **Charter** (`straymark charter new`) |
| Deuda técnica transversal (herencia de Charter previo, aplica a múltiples módulos, requiere Charter dedicado, necesita priorización humana) | **TDE** — distinto del `R<N>` por Charter; ver AGENT-RULES.md §3 |

**NO documentar**: credenciales, tokens, PII, secretos.

---

## Metadatos Mínimos

```yaml
---
id: AILOG-2026-03-25-001
title: Descripción breve
status: accepted
created: 2026-03-25
agent: agent-name-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
# Campos regulatorios opcionales (activar por contexto):
# eu_ai_act_risk: not_applicable
# nist_genai_risks: []
# iso_42001_clause: []
# observability_scope: none
---
```

---

## Revisión Humana Requerida

Marcar `review_required: true` cuando:

- `confidence: low`
- `risk_level: high | critical`
- Decisiones de seguridad
- Cambios irreversibles
- Cambios en modelos ML o prompts
- Cambios en dependencias críticas de seguridad
- Documentos: ETH, ADR, REQ, SEC, MCARD, DPIA

---

## Estructura de Carpetas

```
.straymark/
├── 00-governance/               ← Políticas, AI-GOVERNANCE-POLICY.md
├── 01-requirements/             ← REQ
├── 02-design/decisions/         ← ADR
├── 03-implementation/           ← Guías
├── 04-testing/                  ← TES
├── 05-operations/incidents/     ← INC
├── 06-evolution/technical-debt/ ← TDE
├── 07-ai-audit/
│   ├── agent-logs/              ← AILOG
│   ├── decisions/               ← AIDEC
│   └── ethical-reviews/         ← ETH, DPIA
├── 08-security/                 ← SEC
├── 09-ai-models/                ← MCARD
├── charters/                    ← Charter (NN-slug.md + NN-slug.telemetry.yaml)
└── templates/                   ← Plantillas (incl. subdir charter/)
```

---

## Flujo de Trabajo

```
1. EVALUAR → ¿Requiere documentación?
       ↓
2. CARGAR → Plantilla correspondiente
       ↓
3. CREAR  → Con nomenclatura correcta
       ↓
4. MARCAR → review_required si aplica
```

---

## Niveles

### Confianza
| Nivel | Acción |
|-------|--------|
| `high` | Proceder |
| `medium` | Documentar alternativas |
| `low` | `review_required: true` |

### Riesgo
| Nivel | Ejemplos |
|-------|----------|
| `low` | Docs, formato |
| `medium` | Nueva funcionalidad |
| `high` | Seguridad, APIs |
| `critical` | Producción, irreversible |

---

## Alineamiento Regulatorio

| Estándar | Documentos Clave |
|----------|-----------------|
| ISO/IEC 42001:2023 | AI-GOVERNANCE-POLICY.md (vertebral) |
| EU AI Act | ETH (clasificación de riesgo), INC (reporte de incidentes) |
| NIST AI RMF / 600-1 | ETH (12 categorías de riesgo GenAI), AILOG |
| GDPR | ETH (Privacidad de Datos), DPIA |
| ISO/IEC 25010:2023 | REQ (calidad), ADR (impacto en calidad) |
| OpenTelemetry | Opcional — ver OBSERVABILITY-GUIDE |
| Modelo C4 | Diagramas en ADR — ver C4-DIAGRAM-GUIDE |

---

## Skills (Claude Code)

| Comando | Propósito |
|---------|-----------|
| `/straymark-status` | Verificar estado y cumplimiento de documentación |
| `/straymark-new` | Crear cualquier tipo de documento (interactivo) |
| `/straymark-ailog` / `/straymark-aidec` / `/straymark-adr` | Atajos rápidos para AILOG / AIDEC / ADR |
| `/straymark-mcard` / `/straymark-sec` | Flujos interactivos para Model Card / SEC assessment |
| `/straymark-charter-new` | Andamiar un Charter (unidad de trabajo declarativa ex-ante) |
| `/straymark-audit-prompt CHARTER-XX` *(fw-4.9.0+, refactorizada en fw-4.9.0)* | Auditoría externa multi-modelo — escribe prompt unificado en path canónico |
| `/straymark-audit-execute [CHARTER-XX]` *(fw-4.9.0+)* | Corre en una CLI auditora — lee prompt, audita con tool use, escribe report |
| `/straymark-audit-review CHARTER-XX` *(fw-4.9.0+, expandida en fw-4.9.0)* | Consolida N reports en review.md (6 secciones) + mergea YAML en telemetría |

---

## Patrones

| Patrón | Documento |
|--------|-----------|
| Backlog de follow-ups (registro central + detección de drift) *(fw-4.10.0+)* | [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) |

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
