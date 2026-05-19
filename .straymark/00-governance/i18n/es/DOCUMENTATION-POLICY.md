# Política de Documentación - StrayMark

**Idiomas**: [English](../../DOCUMENTATION-POLICY.md) | Español | [简体中文](../zh-CN/DOCUMENTATION-POLICY.md)

## Por qué existe esta política

StrayMark externaliza la disciplina cognitiva del trabajo de ingeniería de software senior — alcance explícito, decisiones declaradas, riesgos nombrados, alternativas registradas, rastros auditables — en archivos versionados que viven junto al código. Esta política define los tipos de documento, metadatos y reglas de gobernanza que hacen que esa disciplina sea auditable.

Como efecto secundario de producir esos artefactos, el proyecto acumula evidencia que mapea limpiamente a los principales marcos de gobernanza de IA:

- **ISO/IEC 42001:2023** — estándar vertebral para Sistemas de Gestión de IA
- **EU AI Act** (efectivo agosto 2026) — clasificación de riesgo, transparencia, reporte de incidentes
- **NIST AI RMF 1.0 + AI 600-1** — funciones de gestión de riesgos de IA y perfiles de IA generativa
- **ISO/IEC 23894:2023** — marco de gestión de riesgos de IA
- **GDPR** — evaluaciones de impacto en protección de datos y privacidad

La política está escrita primero para el trabajo de ingeniería; el cumplimiento es lo que cae como subproducto cuando el trabajo se documenta con disciplina. Ver Sección 8 para la referencia completa de estándares y `Propuesta/straymark-design-principles.md` (en el repositorio upstream) para el racional a nivel de producto.

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
| `reviewed_by` | Identidad del revisor humano (email, usuario de GitHub o DID) | Lo establece el revisor al aprobar formalmente un documento `review_required: true` |
| `reviewed_at` | Fecha de la aprobación formal (`YYYY-MM-DD`, debe ser ≥ `created`) | Se establece junto con `reviewed_by` |
| `review_outcome` | Señal de cierre: `approved \| revisions_requested \| rejected` | Se establece junto con `reviewed_by`. Su presencia es la señal canónica de "un humano ya revisó" — ver §3.5 abajo |

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

Las referencias relacionadas vinculan documentos con otros **documentos StrayMark** dentro del mismo proyecto. Permiten navegación cruzada en herramientas como `straymark explore`.

**Reglas de formato:**
- Usar el **nombre del archivo** del documento (con extensión `.md`): `AILOG-2026-02-03-001-implementar-sincronizacion.md`
- Para documentos de gobernanza u otros sin tipo, usar el nombre tal cual: `AGENT-RULES.md`, `DOCUMENTATION-POLICY.md`
- Las rutas se resuelven relativas a `.straymark/` — si el documento está en un subdirectorio, incluir la ruta desde `.straymark/`: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implementar-sincronizacion.md`
- Cuando el archivo está en el mismo directorio que el documento que lo referencia, el nombre de archivo es suficiente
- **No usar** IDs de tareas externas (`T001`, `US3`), números de issues ni URLs — esos pertenecen al cuerpo del documento, no al frontmatter
- **No usar** IDs parciales sin descripción (preferir `AILOG-2026-02-03-001-implementar-sincronizacion.md` sobre `AILOG-2026-02-03-001`)

**Ejemplos:**
```yaml
# Mismo directorio o ubicación conocida — el nombre de archivo es suficiente
related:
  - AIDEC-2026-02-02-001-sqlite-bundled-vs-system.md
  - AGENT-RULES.md

# Documentos en subdirectorios específicos — incluir ruta desde .straymark/
related:
  - 07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-implementar-sincronizacion.md
  - 02-design/decisions/ADR-2026-01-15-001-usar-arquitectura-hexagonal.md
```

**Resolución:** El CLI resuelve referencias buscando: (1) coincidencia exacta de ID, (2) coincidencia de nombre de archivo en cualquier parte de `.straymark/`, (3) coincidencia de sufijo de ruta. Usar el nombre de archivo completo proporciona la resolución más confiable.

---

## 3. Estados de Documentos

```
identified ──┐
             ├──► draft ──────► accepted ──────► deprecated
             │                       │                   │
             │                       │                   ▼
             │                       └──────► superseded
             │
             └──► (estado de entrada TDE-only, ver §6)
                                      │
                                      ▼
                                  resolved
                                  (terminal sólo-TDE — deuda pagada; ver §6)
```

| Estado | Descripción |
|--------|-------------|
| `identified` | Estado de entrada para los tipos de descubrimiento dirigidos por agente (TDE hoy). Funcionalmente equivalente a `draft` para el lifecycle gating — se espera que un revisor humano lo priorice y lo promueva. Semánticamente distinto para que las analíticas del adopter puedan distinguir "el agente encontró esta deuda" de "un humano está redactando un documento deliberado". |
| `draft` | En borrador, pendiente de revisión |
| `accepted` | Aprobado y vigente |
| `resolved` | **Estado terminal sólo-TDE**: la deuda técnica descrita en este documento fue atendida; el archivo se mantiene en disco como historia de auditoría. Distinto de `accepted` ("aceptamos que esta deuda persista"), `superseded` ("otro TDE reemplazó a este") y `deprecated` ("el concepto de TDE mismo ya no es relevante"). La referencia canónica de cierre (el Charter, PR o commit que pagó la deuda) va en la sección body `## Resolución`. |
| `deprecated` | Obsoleto, pero se mantiene como referencia |
| `superseded` | Reemplazado por otro documento |

El mapeo de status por defecto por tipo vive en §6 — la mayoría de tipos entran en `draft` o `accepted`, pero TDE entra en `identified` por la frontera de autonomía del agente (el agente identifica, el humano prioriza). TDE es el único tipo hoy con un terminal personalizado (`resolved`); el validador acepta `resolved` globalmente como medida transitoria. Una futura tabla de vocabulario de lifecycle por-tipo (issue #149 Opción B) acotará `resolved` estrictamente a TDE; hasta entonces, usarlo en documentos no-TDE pasa la validación pero es semánticamente incorrecto.

---

## 3.5 Registro de Aprobación

`status` registra el estado del ciclo de vida del documento, y `review_required: true` registra que *se necesita revisión humana*. Ningún campo registra que la revisión humana *efectivamente ocurrió*. Esta sección define la señal canónica de cierre para documentos que requieren aprobación formal (AIDEC, ETH, MCARD, ADR, DPIA, INC, SEC y las variantes del scope China — ver AGENT-RULES.md §4 para los disparadores).

### Señal de cierre

Tres campos de frontmatter opcionales, establecidos por el revisor al momento de la aprobación:

```yaml
reviewed_by: pepe@ejemplo.com           # email | usuario-github | DID
reviewed_at: 2026-05-02
review_outcome: approved                # approved | revisions_requested | rejected
```

Semántica:

- **La presencia de `review_outcome` es la señal de cierre.** Un documento con `review_required: true` y sin `review_outcome` está *pendiente de revisión*.
- `review_required: true` **no** se cambia a `false` después de la aprobación — permanece como registro histórico de por qué hizo falta revisión en primer lugar.
- `reviewed_at` debe ser `>= created`. Si `reviewed_by` está presente, `reviewed_at` y `review_outcome` también deben estarlo (validado por `straymark validate`).
- `review_outcome: revisions_requested` permite ciclos iterativos de revisión: el documento se actualiza y el revisor eventualmente vuelve a aprobar. La convención es sobreescribir los tres campos con la aprobación más reciente (el frontmatter contiene solo el último estado); la sección body de abajo preserva la historia.

### Sección body (forma canónica en prosa)

Agregar en la posición terminal del cuerpo del documento (p. ej., antes de `## References` en AIDEC/ADR; después de `## Review Schedule` en DPIA; después de `## Post-Mortem Review` en INC). Para los templates que ya incluyen una tabla `## Approval` (ETH, MCARD, SEC, PIPIA, CACFILE, TC260RA, AILABEL), cualquiera de las dos formas es canónica; los campos del frontmatter son la fuente de verdad legible-por-máquina.

```markdown
## Approval

**Approved**: 2026-05-02 by `pepe@ejemplo.com`.

<Notas opcionales del revisor — observaciones, condiciones, alcance de la
aprobación. Omitir la sección entera si no hay nada que añadir más allá del
frontmatter.>
```

### Flujos multi-revisor (forward-looking)

Para documentos que requieren múltiples revisores (p. ej., ETH con sign-off de legal y de ingeniería), el canon de v1 es agregar bloques adicionales `## Approval` cronológicamente en el body, con el frontmatter reflejando la *última* aprobación. Una forma estructurada con array `review:` (una entrada por revisor) es forward-looking y no es parte de v1 — se añadirá cuando al menos un proyecto adoptante ejercite el flujo multi-revisor con datos reales.

### Tooling del CLI

`straymark approve <doc-id> --outcome approved --reviewer <id> [--notes "..."] [--at YYYY-MM-DD]` escribe los campos del frontmatter y la sección del body en una sola operación. `straymark validate --check-pending-reviews [--max-pending-days N]` lista documentos con `review_required: true` más antiguos que `N` días sin `review_outcome` (warn-only, no error). Ver CLI-REFERENCE.md.

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
.straymark/
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
| `TDE` | Deuda Técnica | `06-evolution/technical-debt/` | `identified` (entra aquí; terminal `resolved` cuando la deuda se paga — sólo-TDE) | No |
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

| Estándar | Versión | Alcance en StrayMark |
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

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
