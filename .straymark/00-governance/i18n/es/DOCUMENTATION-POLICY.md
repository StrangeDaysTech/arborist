# Política de Documentación - DevTrail

**Idiomas**: [English](../../DOCUMENTATION-POLICY.md) | Español

## Marco de Gobernanza

Esta política alinea la documentación de DevTrail con **ISO/IEC 42001:2023** (estándar vertebral para Sistemas de Gestión de IA) y operacionaliza:

- **EU AI Act** (efectivo agosto 2026) — clasificación de riesgo, transparencia, reporte de incidentes
- **NIST AI RMF 1.0 + AI 600-1** — funciones de gestión de riesgos de IA y perfiles de IA generativa
- **ISO/IEC 23894:2023** — marco de gestión de riesgos de IA
- **GDPR** — evaluaciones de impacto en protección de datos y privacidad

Todos los tipos de documentos, campos de metadatos y reglas de gobernanza contribuyen a evidencia que satisface estos marcos regulatorios. Ver Sección 8 para la referencia completa de estándares.

---

## 1. Convención de Nomenclatura de Archivos

### Formato Estándar

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

| Componente | Descripción | Ejemplo |
|------------|-------------|---------|
| `TIPO` | Prefijo del tipo de documento | `AILOG`, `AIDEC`, `ADR` |
| `YYYY-MM-DD` | Fecha de creación | `2025-01-27` |
| `NNN` | Número secuencial del día | `001`, `002` |
| `descripcion` | Breve descripción en kebab-case | `implementar-oauth` |

### Ejemplos

```
AILOG-2025-01-27-001-implementar-oauth.md
AIDEC-2025-01-27-001-seleccion-framework-testing.md
ADR-2025-01-27-001-arquitectura-microservicios.md
REQ-2025-01-27-001-autenticacion-usuarios.md
```

---

## 2. Metadatos Requeridos (Frontmatter)

Todos los documentos deben incluir metadatos YAML al principio:

```yaml
---
id: AILOG-2025-01-27-001
title: Implementación de Autenticación OAuth
status: draft | accepted | deprecated | superseded
created: 2025-01-27
updated: 2025-01-27
agent: claude-code-v1.0
confidence: high | medium | low
review_required: true | false
risk_level: low | medium | high | critical
tags:
  - auth
  - security
related:
  - ADR-2025-01-20-001
  - REQ-2025-01-15-003
---
```

### Campos Requeridos

| Campo | Descripción |
|-------|-------------|
| `id` | Identificador único (igual que el nombre del archivo sin .md) |
| `title` | Título descriptivo |
| `status` | Estado actual del documento |
| `created` | Fecha de creación |
| `agent` | Identificador del agente que creó el documento |
| `confidence` | Nivel de confianza del agente |
| `review_required` | Si se requiere revisión humana |
| `risk_level` | Nivel de riesgo del cambio |

### Campos Opcionales

| Campo | Descripción | Cuándo Usar |
|-------|-------------|-------------|
| `updated` | Fecha de última actualización | En cualquier actualización |
| `tags` | Etiquetas para categorización (ver convenciones abajo) | Siempre recomendado |
| `related` | Referencias a documentos relacionados (ver convenciones abajo) | Cuando existen referencias cruzadas |
| `supersedes` | ID del documento que este reemplaza | Al reemplazar un documento |
| `superseded_by` | ID del documento que reemplaza a este | Establecido por el documento que reemplaza |
| `eu_ai_act_risk` | Clasificación de riesgo EU AI Act: `unacceptable \| high \| limited \| minimal \| not_applicable` | Cuando el cambio involucra sistemas de IA bajo EU AI Act |
| `nist_genai_risks` | Categorías de riesgo NIST AI 600-1: `[privacy, bias, confabulation, ...]` | Cuando el cambio involucra componentes de IA generativa |
| `iso_42001_clause` | Cláusulas ISO 42001: `[4, 5, 6, 7, 8, 9, 10]` | Al mapear a controles ISO 42001 |
| `lines_changed` | Conteo de líneas cambiadas (auto-calculable) | En documentos AILOG |
| `files_modified` | Lista de archivos modificados (auto-calculable) | En documentos AILOG |
| `observability_scope` | Nivel de instrumentación OTel: `none \| basic \| full` | Cuando el cambio involucra instrumentación de observabilidad |
| `api_spec_path` | Ruta al archivo de especificación OpenAPI/AsyncAPI | En documentos REQ cuando el requisito involucra interfaces de API |
| `api_changes` | Lista de endpoints de API afectados | En documentos ADR cuando la decisión modifica APIs públicas |

### Convención de Tags

Los tags son **palabras clave de formato libre** usadas para categorización y búsqueda. Ayudan a descubrir documentos relacionados en todo el proyecto.

**Reglas de formato:**
- Usar **kebab-case** (minúsculas con guiones): `gnome-integration`, `sqlite`, `api-design`
- Un concepto por tag — evitar tags compuestos como `auth-y-seguridad`
- Rango recomendado: **3 a 8 tags** por documento
- Los tags deben describir el **tema**, **tecnología**, **componente** o **preocupación** del documento

**Ejemplo:**
```yaml
tags: [sqlite, persistencia, hexagonal-architecture, repository-pattern]
```

### Convención de Related

Las referencias relacionadas vinculan documentos con otros **documentos DevTrail** dentro del mismo proyecto. Permiten navegación cruzada en herramientas como `devtrail explore`.

**Reglas de formato:**
- Usar el **nombre del archivo** del documento (con extensión `.md`): `AILOG-2026-02-03-001-implementar-sincronizacion.md`
- Para documentos de gobernanza u otros sin tipo, usar el nombre tal cual: `AGENT-RULES.md`, `DOCUMENTATION-POLICY.md`
- Las rutas se resuelven relativas a `.devtrail/` — si el documento está en un subdirectorio, incluir la ruta desde `.devtrail/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implementar-sincronizacion.md`
- Cuando el archivo está en el mismo directorio que el documento que lo referencia, el nombre de archivo es suficiente
- **No usar** IDs de tareas externas (`T001`, `US3`), números de issues ni URLs — esos pertenecen al cuerpo del documento, no al frontmatter
- **No usar** IDs parciales sin descripción (preferir `AILOG-2026-02-03-001-implementar-sincronizacion.md` sobre `AILOG-2026-02-03-001`)

**Ejemplos:**
```yaml
# Mismo directorio o ubicación conocida — el nombre de archivo es suficiente
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# Documentos en subdirectorios específicos — incluir ruta desde .devtrail/
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implementar-sincronizacion.md
  - 02-design/decisions/ADR-2026-01-15-001-usar-arquitectura-hexagonal.md
```

**Resolución:** El CLI resuelve referencias buscando: (1) coincidencia exacta de ID, (2) coincidencia de nombre de archivo en cualquier parte de `.devtrail/`, (3) coincidencia de sufijo de ruta. Usar el nombre de archivo completo proporciona la resolución más confiable.

---

## 3. Estados de Documentos

```
draft ──────► accepted ──────► deprecated
                │                   │
                │                   ▼
                └──────► superseded
```

| Estado | Descripción |
|--------|-------------|
| `draft` | En borrador, pendiente de revisión |
| `accepted` | Aprobado y vigente |
| `deprecated` | Obsoleto, pero se mantiene como referencia |
| `superseded` | Reemplazado por otro documento |

---

## 4. Niveles de Riesgo

| Nivel | Cuándo usar | Requiere revisión |
|-------|-------------|-------------------|
| `low` | Cambios cosméticos, documentación | No |
| `medium` | Nueva funcionalidad, refactoring | Recomendado |
| `high` | Seguridad, datos sensibles, APIs públicas | Sí |
| `critical` | Cambios irreversibles, producción | Obligatorio |

---

## 5. Niveles de Confianza

| Nivel | Significado | Acción |
|-------|-------------|--------|
| `high` | El agente está seguro de la decisión | Proceder |
| `medium` | El agente tiene dudas menores | Documentar alternativas |
| `low` | El agente necesita validación | Marcar `review_required: true` |

---

## 6. Estructura de Carpetas

```
.devtrail/
├── 00-governance/          # Políticas y reglas
├── 01-requirements/        # Requisitos del sistema
├── 02-design/              # Diseño y arquitectura
│   └── decisions/          # ADRs
├── 03-implementation/      # Guías de implementación
├── 04-testing/             # Estrategias de prueba
├── 05-operations/          # Operaciones
│   └── incidents/          # Post-mortems
├── 06-evolution/           # Evolución del sistema
│   └── technical-debt/     # Deuda técnica
├── 07-ai-audit/            # Auditoría de agentes IA
│   ├── agent-logs/         # AILOG
│   ├── decisions/          # AIDEC
│   └── ethical-reviews/    # ETH
├── 08-security/            # SEC — Evaluaciones de seguridad
├── 09-ai-models/           # MCARD — Tarjetas de modelo/sistema
└── templates/              # Plantillas
```

### Tipos de Documentos

| Tipo | Nombre | Carpeta | Estado por Defecto | Requiere Revisión |
|------|--------|---------|-------------------|-------------------|
| `AILOG` | Log de Acción IA | `07-ai-audit/agent-logs/` | `accepted` | No |
| `AIDEC` | Decisión IA | `07-ai-audit/decisions/` | `accepted` | No |
| `ETH` | Revisión Ética | `07-ai-audit/ethical-reviews/` | `draft` | Sí |
| `ADR` | Registro de Decisión de Arquitectura | `02-design/decisions/` | `draft` | Sí |
| `REQ` | Requisito | `01-requirements/` | `draft` | Sí |
| `TES` | Plan de Pruebas | `04-testing/` | `draft` | Sí |
| `INC` | Post-mortem de Incidente | `05-operations/incidents/` | `draft` | Sí |
| `TDE` | Deuda Técnica | `06-evolution/technical-debt/` | `identified` | No |
| `SEC` | Evaluación de Seguridad | `08-security/` | `draft` | Sí (siempre) |
| `MCARD` | Tarjeta de Modelo/Sistema | `09-ai-models/` | `draft` | Sí (siempre) |
| `SBOM` | Lista de Materiales de Software | `07-ai-audit/` | `accepted` | No |
| `DPIA` | Evaluación de Impacto en Protección de Datos | `07-ai-audit/ethical-reviews/` | `draft` | Sí (siempre) |

---

## 7. Referencias Cruzadas

Usa el formato `[TIPO-ID]` para referencias:

```markdown
Esta decisión se basa en los requisitos definidos en [REQ-2025-01-15-003].
Ver también [ADR-2025-01-20-001] para contexto arquitectónico.
```

---

## 8. Estándares Referenciados

| Estándar | Versión | Alcance en DevTrail |
|----------|---------|---------------------|
| ISO/IEC/IEEE 29148:2018 | 2018 | Ingeniería de requisitos — TEMPLATE-REQ.md |
| ISO/IEC 25010:2023 | 2023 | Modelo de calidad de software — TEMPLATE-REQ.md, TEMPLATE-ADR.md |
| ISO/IEC/IEEE 29119-3:2021 | 2021 | Documentación de pruebas de software — TEMPLATE-TES.md |
| ISO/IEC 42001:2023 | 2023 | Sistema de Gestión de IA — AI-GOVERNANCE-POLICY.md (estándar vertebral) |
| EU AI Act | 2024 (vigente ago 2026) | Regulación de IA — ETH, INC, campos regulatorios de AILOG |
| NIST AI RMF 1.0 | 2023 | Gestión de riesgos de IA — ETH, categorías de riesgo de AILOG |
| NIST AI 600-1 | 2024 | Perfil de IA generativa — 12 categorías de riesgo en ETH/AILOG |
| ISO/IEC 23894:2023 | 2023 | Gestión de riesgos de IA — AI-RISK-CATALOG |
| GDPR | 2016/679 | Protección de datos — ETH (Privacidad de Datos), DPIA |
| OpenTelemetry | Actual | Observabilidad — OBSERVABILITY-GUIDE, opcional |

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
