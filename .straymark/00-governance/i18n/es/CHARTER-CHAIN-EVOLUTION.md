# Evolución de la cadena de Charters — StrayMark

> Dos patrones complementarios para mantener honesto un módulo multi-Charter: refrescar los artefactos SpecKit antes de declarar un Charter, y enmendar un Charter cerrado cuando llegan hallazgos de auditoría después de `status: closed`.

**Idiomas**: [English](../../CHARTER-CHAIN-EVOLUTION.md) | Español | [简体中文](../zh-CN/CHARTER-CHAIN-EVOLUTION.md)

---

## Estado

**v0 — validado en N=1 dominio** (`StrangeDaysTech/sentinel` CHARTER-18, 2026-05-15, Issue #156).

Ambos patrones son convenciones documentadas aquí como guía canónica del framework. La CLI envía helpers read-only y de scaffolding (`straymark charter refresh-suggest`, `straymark charter amend`); los patrones en sí los conduce el operador. Cualquiera de los dos patrones puede evolucionar cuando un segundo adopter los valide — hasta entonces, aplica la advertencia de dominio N=1 (Principio #12).

---

## Por qué existe este documento

El patrón Charter de StrayMark (`STRAYMARK.md` §15) asume que un único Charter es la unidad acotada de trabajo. Eso funciona para Charters aislados. También funciona para el *primer* Charter de una cadena. Pero cuando un módulo acumula muchos user-story Charters a lo largo de meses, emergen dos modos de falla que el patrón por-Charter no aborda:

1. **Deriva de spec a nivel de cadena** — los artefactos SpecKit (`plan.md`, `data-model.md`, `contracts/*`, `quickstart.md`, `research.md`) se escribieron contra la versión del framework y la realidad del módulo al inicio de la cadena. Tras 3+ Charters, los aprendizajes acumulados (patrones reusables extraídos, gaps de código encontrados, convenciones del framework evolucionadas, decisiones del operador ratificadas) han alejado la spec de la implementación. Pasar directamente del cierre del Charter-N al declare del Charter-(N+1) produce expansiones de scope sistemáticas en mid-flight y entradas emergentes `R<N+1> (new, not in Charter)`.
2. **Hallazgos de auditoría a nivel de ciclo** — los ciclos de auditoría externa corren post-close (los auditores ejecutan asincrónicamente después de la ceremonia de cierre). Hallazgos Critical o High pueden llegar después de que el Charter está marcado `status: closed`. Las opciones del framework son entonces: (a) abrir un Charter nuevo de remediación (pesado — declare + Tasks + ceremonia completos para ~5 ediciones de archivo), o (b) dejar los hallazgos en `review.md` y perder la propiedad "atómico con el Charter".

El Patrón 1 aborda (1). El Patrón 2 aborda (2). Los dos componen — un Charter que *recibió* el Patrón 1 tiene más probabilidad de *evitar* el Patrón 2, porque el refresh absorbe el riesgo pre-ejecución que la auditoría surfacearía post-close. Son complementarios, no sustituibles.

---

## Patrón 1 — Refresh de SpecKit pre-declare

### Cuándo aplica este patrón

Adopta este patrón cuando **todo** lo siguiente sostenga para un módulo dirigido por SpecKit:

- El módulo tiene **3 o más Charters cerrados** (longitud de cadena ≥ 3).
- La media móvil de `charter_telemetry.agent_quality.r_n_plus_one_emergent_count` sobre los últimos 3 Charters cerrados es **> 6**.
- Ningún PR de refresh ha aterrizado en los artefactos SpecKit desde el último branch point de la cadena.

Ejecuta `straymark charter refresh-suggest <module>` para evaluar la heurística contra tu historial `.telemetry.yaml`. La CLI lee los últimos Charters cerrados del módulo nombrado e imprime una recomendación; no muta nada.

Por debajo del umbral, el patrón por-Charter solo es suficiente — adoptar el refresh demasiado pronto añade overhead de un PR sin beneficio.

### Forma

Un **PR dedicado de refresh** aterriza entre el cierre del Charter-N y el declare del Charter-(N+1). Toca solo las **secciones no-locked** de los artefactos SpecKit:

- `specs/<module>/plan.md` — planes de fase, notas de dependencia, secuenciación.
- `specs/<module>/data-model.md` — entidades, campos, convenciones.
- `specs/<module>/contracts/*.md` — contratos de interfaz, formas de request/response.
- `specs/<module>/quickstart.md` — escenarios ejecutables.
- `specs/<module>/research.md` — conocimiento acumulado (ver "Tabla de aprendizajes categorizados" abajo).

`research.md` carga el artefacto load-bearing: una **tabla de aprendizajes categorizados** que consolida lo que la cadena aprendió. Buckets mínimos:

| Bucket | Qué va aquí |
|---|---|
| Patrones reusables | Idioms / utilities / wrappers que emergieron a través de Charters y deberían heredarse hacia adelante (p.ej. wrapper `withRLS`, LRU brand-cache, patrón de tabla de dedup). |
| Gaps de código | Trabajo identificado-pero-no-arreglado que la cadena descubrió pero no cerró (p.ej. tablas sin wirear, implementaciones stub, columnas faltantes). Cada gap es una entrada `Gn` con descripción + Charter dueño (actual o futuro). |
| Patrones de disciplina | Aprendizajes de proceso que la cadena ratificó (p.ej. pareja de auditoría cross-family, disciplina de batch-complete, cadencia de close per-batch). |
| Correcciones empíricas | Lugares donde la spec derivó de la implementación. Entradas `EC1...ECn`: la spec decía X, la realidad es Y, reconciliación elegida. |

Decisiones opcionales **del operador (Dn)** se ratifican pre-declare con: decisión, alternativas consideradas, camino elegido, racional. Los Charters posteriores heredan Dn como contratos.

### Mecánica

1. **PR de refresh** antes del próximo Charter declare. AIDEC opcional documentando la decisión del refresh + alternativas consideradas. El título del PR debería hacer explícito el scope (p.ej. `spec(<module>): US<n> plan refresh — LOCKED-aware Phase 7+8 redesign`).
2. **Tabla de aprendizajes categorizados** en `research.md` con los cuatro buckets arriba. Cada entrada tiene un id estable (Pn / Gn / DPn / ECn) para que Charters posteriores puedan citar por id.
3. **Decisiones del operador (Dn)** si aplican — listadas explícitamente con alternativas + camino elegido + racional.
4. **Sección `## Context` del próximo Charter** cita cada patrón, corrección y decisión por id. El scope del Charter se ancla en realidad refrescada, no en la spec del inicio de cadena.

### Telemetría

Popula `charter_telemetry.pre_declare_refresh:` en la telemetría del *próximo* Charter (el que consumió el refresh, no en el PR de refresh mismo):

```yaml
pre_declare_refresh:
  enabled: true
  refresh_pr: "owner/repo#76"
  refresh_aidec: "AIDEC-YYYY-MM-DD-NNN-speckit-refresh"
  reusable_patterns_integrated: 7
  code_gaps_integrated: 4
  discipline_patterns_integrated: 3
  empirical_corrections_integrated: 15
  operator_decisions_ratified: 3
```

Omite el bloque entero si no ocurrió un refresh — la ausencia significa "patrón no usado".

### Por qué funciona (empírico)

Sentinel CHARTER-18 fue el primer Charter en una cadena de 7 Charters en cerrar limpiamente sin un Charter de remediación mid-flight. `estimation_drift_factor: 1.0`, `pre_work.items_discovered_during_planning: 0`, `overall_satisfaction: 5/5`. Statement de drift del operador: *"el refresh de SpecKit del PR #76 ... eliminó la mayor parte de la ambigüedad que conducía la deriva en Charters previos. No se requirió Charter de remediación mid-flight — el inventario de correcciones empíricas EC1..EC15 en research.md absorbió lo que habría sido riesgo pre-ejecución en consciencia in-ejecución."*

---

## Patrón 2 — Enmienda post-close dirigida por auditoría (Batch N.4)

### Cuándo aplica este patrón

Adopta este patrón cuando **todo** lo siguiente sostenga después de que un Charter ha sido marcado `status: closed`:

- Uno o más hallazgos de auditoría externa emergen en el `review.md` post-close calificados **Critical** o **High**.
- El `closure_criterion` del Charter está materialmente incumplido por los hallazgos no remediados (es decir, enviar tal como está invalidaría el cierre).
- La superficie de fix cabe en **un PR cohesivo** (~< 25 archivos, sin reopen arquitectónico — sin abstracciones nuevas, sin migraciones, sin API breaks).

Si la superficie de fix es mayor o arquitectónica, abre un Charter nuevo en su lugar. El patrón de enmienda existe para el caso acotado; no es un mecanismo de evasión de Charter.

### Forma

La enmienda viaja en **la misma rama de execute** que el Charter original (la rama sigue mergeable a `main`; el commit de enmienda aterriza encima). Un **AILOG nuevo** documenta la enmienda — no una edición del AILOG original.

```
rama charter-<N>-execute
├── (commits originales — trabajo de execute del Charter)
├── commit X: charter close (status: closed, telemetry.yaml escrito)
└── commit Y: charter-<N>(batch-7.4): audit-driven remediation — <resumen corto>
    ↑
    AILOG-YYYY-MM-DD-MMM (NUEVO) documenta este commit
    AILOG-YYYY-MM-DD-NNN (ORIGINAL) recibe una subsección `## Historical correction`
                                    apuntando adelante a AILOG-...-MMM
```

### Mecánica

1. **Misma rama de execute** — no bifurques desde `main`. La rama de execute del Charter original sigue siendo la unidad; el commit de enmienda viaja con ella.
2. **AILOG nuevo** bajo `.straymark/07-ai-audit/agent-logs/` documenta la enmienda. Convención: `risk_level: high` y `review_required: true`. El AILOG nuevo carga un campo `amends:` apuntando de vuelta al id del AILOG original.
3. **Corrección histórica en el AILOG original** — anexa una subsección `## Historical correction (YYYY-MM-DD)` al final del AILOG original con el puntero adelante al AILOG nuevo. Las decisiones de auditoría son distintas de las de execute; el cuerpo del original permanece intacto como registro histórico.
4. **Comentario en el PR** — si el PR de execute aún no ha merged, agrega el commit de enmienda y actualiza la descripción del PR con una subsección "Batch N.4 amendment" listando los hallazgos cerrados. Si el PR ya merged, abre un PR de seguimiento referenciando el PR original y el AILOG.
5. **Telemetría** — popula `charter_telemetry.post_close_amendment:` (ver abajo). Usa `straymark charter audit <id> --merge-reports --merge-into <telemetry-yaml>` para mergear hallazgos de auditoría externa al mismo archivo; la CLI tolera reescrituras del placeholder `external_audit: []` en v0.2+.

`straymark charter amend <id>` hace scaffolding de los pasos 2, 3 y 5 (crea el stub del AILOG nuevo, edita el AILOG original con la subsección Historical correction, imprime el bloque YAML). No toca git — el operador decide cuándo committear.

### Telemetría

Popula `charter_telemetry.post_close_amendment:` en el `.telemetry.yaml` del Charter:

```yaml
post_close_amendment:
  applied: true
  trigger: "external_audit"           # external_audit | production_incident | deferred_implementation
  ailog_id: "AILOG-YYYY-MM-DD-MMM"    # el AILOG NUEVO, no el original
  findings_closed: 5
  files_modified: 19
  effort_hours: 6.0
```

Omite el bloque entero si no ocurrió enmienda.

### Por qué funciona (empírico)

Sentinel CHARTER-18 cerró el 2026-05-15 con `external-audit-pending.yaml`. Los reportes de auditoría aterrizaron 2026-05-15..05-17. Cinco hallazgos (4 Critical/High de `gpt-5.3-codex`, 1 Critical de `gemini-2.5-pro`, 1 Medium encontrado por el calibrador) fueron fixes a nivel de código — wiring DI, parsing de header de retry, filtro multi-tenant, default de timeout. La enmienda Batch 7.4 cerró los cinco en un commit cohesivo (19 archivos, +2257/-106 líneas). Un Charter nuevo habría creado overhead de gobernanza multi-semana para ~6h de ingeniería enfocada.

---

## Composición cross-pattern

Los dos patrones operan en distintos niveles de la cadena y componen:

| Patrón | Nivel | Frecuencia | Absorbe |
|---|---|---|---|
| Refresh de SpecKit pre-declare | Cadena / módulo | Una vez cada 3+ Charters | Deriva a nivel de spec (asunciones arquitectónicas, naming de tablas, evolución de versión de framework) |
| Enmienda post-close dirigida por auditoría | Ciclo / Charter | Por Charter cuando se dispara | Deriva a nivel de runtime (wiring DI, semántica de retry, filtros multi-tenant) |

Un Charter que *recibió* el Patrón 1 tiene más probabilidad de *evitar* el Patrón 2 — el refresh absorbe riesgo pre-ejecución que de otro modo surgiría como hallazgos post-close. Pero CHARTER-18 necesitó *ambos* — el refresh manejó deriva a nivel de spec; la enmienda manejó deriva a nivel de runtime que el refresh no alcanzaba. Fomenta el Patrón 1 al nivel de cadena; tolera el Patrón 2 al nivel de ciclo.

---

## Flujo de autoridad / aceptación para upstreaming de patrones nuevos

Este documento es por sí mismo el output del flujo de aceptación que Sentinel caminó para estos dos patrones (Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)). El flujo canónico para upstreamear un patrón nuevo de cadena-Charter es:

1. **RFC adopter-local** vive en `.straymark/06-evolution/<name>-rfc.md` en el árbol del adopter. El adopter envía el patrón ahí primero — la evidencia N=1 es necesaria pero no suficiente.
2. **Issue upstream** en `StrangeDaysTech/straymark` reflejando el cuerpo del RFC local, con citas de telemetría y enlaces a PRs.
3. **Aceptación upstream** aterriza como: (a) un doc aquí en `00-governance/` describiendo el patrón canónicamente, (b) adiciones al schema de telemetría (opt-in), (c) scaffolding CLI opcional para la mecánica orientada al operador. La advertencia de dominio N=1 se traslada a la estabilización v1.
4. **Validación en segundo dominio** antes de que los campos de schema del patrón gradúen de opcionales a recomendados.

`06-evolution/` es el hogar canónico adopter-local para RFCs en vuelo. Una vez aceptado upstream, el hogar canónico es `00-governance/<NAME>.md` — la convención que este documento instancia.

---

## Preguntas abiertas

- **Tuneo de umbral** — el umbral de media móvil de 6 para `r_n_plus_one_emergent_count` es derivado de Sentinel. Un segundo dominio puede moverlo. La CLI `straymark charter refresh-suggest` expone `--threshold N` para calibración del adopter.
- **Heurística de módulo** — `refresh-suggest <module>` actualmente matchea `<module>` contra el título y slug del Charter. Los módulos convencionales de SpecKit (`specs/<NNN>-<module>/`) podrían proveer un binding más estricto vía el campo `originating_spec` del Charter en un bump futuro de fw.
- **Cap de frecuencia de enmienda** — el Patrón 2 está acotado por "un PR cohesivo". Un Charter que recibe dos o más commits de enmienda a lo largo del tiempo debería re-evaluarse como señal de que el cierre original fue prematuro.

---

## Relacionado

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) — el meta-patrón del cual Pattern 1 y Pattern 2 son aplicaciones (cross-referencing formal + permiso cultural para surfacear).
- [STRAYMARK.md §15](../../../STRAYMARK.md) — Ciclo de vida del Charter y el patrón por-Charter que este documento extiende.
- [SPECKIT-CHARTER-BRIDGE.md](SPECKIT-CHARTER-BRIDGE.md) — cómo los artefactos SpecKit mapean a Charters; el Patrón 1 vive en esta costura.
- [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) — patrón hermano para `§Follow-ups` acumulados a lo largo de muchos AILOGs.
- [`.straymark/schemas/charter-telemetry.schema.v0.json`](../../schemas/charter-telemetry.schema.v0.json) — `pre_declare_refresh` y `post_close_amendment` están definidos aquí.

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
