# DevTrail - Referencia Rápida

> Referencia de una página para agentes IA y desarrolladores.
>
> **Este es un documento derivado** — DOCUMENTATION-POLICY.md es la fuente autoritativa.

**Idiomas**: [English](../../QUICK-REFERENCE.md) | Español

---

## Configuración de Idioma

**Archivo**: `.devtrail/config.yml`

```yaml
language: en  # Opciones: en, es (por defecto: en)
```

| Idioma | Ruta de Templates |
|--------|-------------------|
| `en` | `.devtrail/templates/TEMPLATE-*.md` |
| `es` | `.devtrail/templates/i18n/es/TEMPLATE-*.md` |

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

---

## Cuándo Documentar

| Situación | Acción |
|-----------|--------|
| Código complejo (`devtrail analyze`; alternativa: >20 líneas) | AILOG |
| Decisión entre alternativas | AIDEC |
| Cambios en auth/autorización/PII | AILOG + `risk_level: high` + ETH |
| Cambios en API pública o esquema de BD | AILOG + considerar ADR |
| Cambios en modelos ML/prompts | AILOG + revisión humana |
| Cambios en dependencias críticas de seguridad | AILOG + revisión humana |
| Cambios en instrumentación OTel | AILOG + tag `observabilidad` |

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
.devtrail/
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
└── templates/                   ← Plantillas
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
| `/devtrail-status` | Verificar estado y cumplimiento de documentación |

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
