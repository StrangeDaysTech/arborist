# Puente SpecKit ↔ Charter de StrayMark

> **Estado**: Patrón empírico (`v0`). Cristaliza tras validarse contra un segundo dominio (Principio #12). Refinar vía PRs cuando surjan nuevos casos de uso.

## El problema que este documento resuelve

[SpecKit](https://github.com/StrangeDaysTech/speckit) te da `spec.md`, `plan.md` y `tasks.md` para una feature. StrayMark te da Charters, AILOGs, AIDECs, ADRs. **Ningún documento canónico explicaba cuándo una feature de SpecKit debe producir un Charter, qué granularidad usar, quién dispara la creación, ni cuándo.** Reportado como el artefacto central del [issue #113](https://github.com/StrangeDaysTech/straymark/issues/113) — un *gap* de descubribilidad que llevaba a los agentes (Claude, Gemini, Copilot) a construir modelos mentales binarios (`SpecKit = planeación, StrayMark = audit-trail`) y a descartar silenciosamente la tercera capa (work-as-auditable-shippable-unit) donde viven los Charters.

Este archivo es la respuesta.

## Modelo mental

Tres capas, con *handoffs*:

| Capa | Vive en | Propósito | Dueño |
|------|---------|-----------|-------|
| **1. Especificación** | `specs/NNN-feature/{spec,plan,tasks,research,quickstart}.md` | Qué es la feature, por qué existe, cómo se implementará a nivel técnico. SpecKit produce esto vía `/speckit-specify` → `/speckit-plan` → `/speckit-tasks`. | Operador (con asistencia del agente). |
| **2. Unidad acotada de ejecución** | `.straymark/charters/NN-slug.md` | El contrato de un único corte enviable de la feature. Empareja el alcance ex-ante (archivos, riesgos, subset de tareas) con la telemetría ex-post (drift, audit, lecciones). | Operador declara el Charter; el agente ejecuta dentro del mismo. |
| **3. Traza de implementación** | `.straymark/07-ai-audit/agent-logs/AILOG-*.md` (más AIDECs y ADRs cuando aplique) | Registro día-a-día de qué se hizo, por qué, con qué nivel de confianza. Cada AILOG referencia al Charter vía `originating_charter:` (o el Charter agrega los AILOGs vía `originating_ailogs:`). | El agente los crea mientras trabaja; el operador revisa. |

**El puente es el Charter.** Las specs son demasiado de alto nivel para hacer drift-check ("¿enviaste la spec?" no se puede contestar en un horizonte útil). Los AILOGs son demasiado de bajo nivel para enviar contra ellos ("¿enviaste este AILOG?" es la unidad equivocada). Los Charters están en la granularidad correcta: un contrato de alcance estable contra el que puedes auditar en días, no en meses.

## ¿Cuándo una feature de SpecKit produce un Charter?

Una feature de SpecKit debe producir **al menos un Charter** cuando *cualquiera* de las siguientes se cumple:

1. El `tasks.md` de la feature tiene **5 o más tareas** que no puedes completar en una sola sesión.
2. La feature abarca **2 o más fases de SpecKit** (Setup, Foundation, User Stories, Polish, etc.) que pretendes enviar juntas como una unidad.
3. El trabajo amerita una **auditoría externa** (revisión cross-modelo, cross-equipo) al cierre.
4. Quieres **telemetría medible** al cierre (effort estimate vs. real, conteo de drift, lecciones).

**No** debe producir un Charter cuando:

- La feature es lo suficientemente pequeña para enviarse en una sola sesión (<1 día, <5 tareas). Usa AILOGs solamente — el overhead del Charter excede la ganancia de auditabilidad.
- La feature es **puramente planeación** (sin código todavía). Espera hasta que exista `tasks.md`; el contrato del Charter necesita tareas concretas que enumerar.
- La feature es **mantenimiento** sin alcance planeado (ej. "arreglar bugs según aparezcan"). Para mantenimiento ad-hoc, los AILOGs son suficientes.

## Heurísticas de granularidad

Cuando una feature amerita Charters, elige granularidad por **unidad enviable**, no por unidad estructural. Concretamente:

### Heurística 1 — Un Charter por corte enviable

Si la feature tiene Fases (ej. el típico Foundation → US1 → US2 → US3 → Polish de SpecKit), el **primer Charter envuelve el corte de fundación** (todo lo que envía junto como `v0.1`). Charters subsecuentes envuelven cortes subsecuentes. *Effort estimate* **M** es el bucket mediano para un corte enviable; **L** para un corte de feature completa.

```
specs/001-peek-mvp-foundation/
├── spec.md
├── plan.md
└── tasks.md  →  CHARTER-01 (Foundation: T001-T012, effort M)
                  CHARTER-02 (peek MVP: T013-T044, effort L)
```

### Heurística 2 — NO por User Story

Las User Stories son demasiado granulares. Una US que toma 2-3 tareas pertenece *dentro* de un Charter, no como su propio Charter. Telemetría por US es ruido; telemetría por corte enviable es señal.

### Heurística 3 — NO por feature

Una feature que se envía en dos cortes (ej. MVP → polish) merece dos Charters, no uno. El contrato del Charter contra el que puedes hacer drift-check es "lo que envió este corte", no "lo que eventualmente construimos".

### Heurística 4 — Caso borde: ≥10 tareas en 4+ fases

Cuando una feature es excepcionalmente grande, un tercer Charter (o partir el corte de fundación en "scaffolding" + "core") puede estar justificado. Usa effort estimate **L** como tope; si estimarías **XL**, esa es señal de que la feature debe re-especificarse.

## Cronología de creación

```
/speckit-specify  → spec.md
/speckit-plan     → plan.md
/speckit-tasks    → tasks.md
                    ↓
                ┌────────────────────────────────────────┐
                │  ★ PUNTO DE DECLARACIÓN DEL CHARTER ★  │
                │                                        │
                │  Operador corre `straymark charter new`│
                │   --from-spec specs/NNN-feature/spec.md│
                │   --type <M|L>                         │
                │                                        │
                │  Status del Charter: declared          │
                │  → Operador llena scope, files, tasks  │
                │  → status: in-progress al ejecutar     │
                └────────────────────────────────────────┘
                    ↓
/speckit-implement  → tareas ejecutadas
                    → AILOGs creados (`originating_charter:` → Charter)
                    ↓
straymark charter drift CHARTER-NN  → check archivos-vs-commit
straymark charter audit CHARTER-NN  → auditoría externa (opcional)
straymark charter close CHARTER-NN  → telemetría, status: closed
```

**Invariante clave**: declara el Charter *antes* de que `/speckit-implement` arranque. El Charter es un contrato; declararlo después de la ejecución vacía el drift check.

## Vinculación en frontmatter

El frontmatter del Charter cita explícitamente la feature de SpecKit:

```yaml
charter_id: CHARTER-01-workspace-foundation
status: declared
effort_estimate: M
trigger: tasks.md tiene 12 tareas ordenadas en 2 fases; envíar como v0.1.
originating_spec: specs/001-peek-mvp-foundation/spec.md
```

La dirección inversa (spec → Charter) es por convención — lista el Charter activo en la sección "Phase 5: Implementation Tracking" de la spec si tu template de `plan.md` la tiene. SpecKit actualmente no tiene un slot de schema para esto; convención emergente.

Los AILOGs creados durante la ejecución deben citar al Charter:

```yaml
id: AILOG-2026-05-08-005
title: T013, T016-T026 — US1 P1 MVP core + TUI + peek bin
agent: claude-code-v4.7
confidence: high
risk_level: medium
review_required: false
originating_charter: CHARTER-02-peek-mvp-foundation
```

## Mapa del ciclo de vida

| Fase de SpecKit | Evento del Charter | CLI de StrayMark |
|-----------------|-------------------|------------------|
| `/speckit-tasks` completo | **Declarar Charter** | Skill `/straymark-charter-new` o `straymark charter new --from-spec …` |
| Primera tarea inicia | Operador cambia `declared` → `in-progress` | (edición manual de frontmatter) |
| Cada tarea ejecutada | AILOG producido (cuando lo amerite §6 de STRAYMARK.md) | `/straymark-ailog` |
| Decisión mayor encontrada | AIDEC producido | `/straymark-aidec` |
| Cambio arquitectónico | ADR producido | `/straymark-adr` |
| Última tarea hecha, antes de cerrar | Drift check | `straymark charter drift CHARTER-NN` |
| Revisión externa opcional | Auditoría multi-modelo | `straymark charter audit CHARTER-NN` + `/straymark-audit-prompt` + `/straymark-audit-execute` + `/straymark-audit-review` |
| Corte enviado | Cerrar Charter | `straymark charter close CHARTER-NN` (status: `closed`, telemetry yaml emitido) |

## Mantención del spec durante ejecución multi-Charter

> **Anclaje empírico**: surfaceado por [issue #150](https://github.com/StrangeDaysTech/straymark/issues/150) después de que Sentinel corriera un único `specs/002-commshub/plan.md` (committed 2026-04-21) a través de **siete Charters consecutivos** (CHARTER-07 a CHARTER-17, ~1 mes). Doce aprendizajes empíricos que impactan materialmente el scope del siguiente Charter **no** quedaron reflejados en el plan. El patrón siguiente codifica lo que Sentinel descubrió antes de llenar CHARTER-18.

El mapa de ciclo de vida arriba asume **una sola pasada**: los artefactos de SpecKit se generan una vez, luego los Charters se declaran y ejecutan. Esto escala bien para features que producen un solo Charter. Cuando un solo spec conduce varios Charters espaciados por semanas, **los artefactos de planeación driftean respecto al código shippeado** — y re-correr `/speckit-plan` ingenuamente es *peor*, no mejor: la regeneración afirma cosas sobre user stories ya shippeadas que el código real no implementa, y los lectores futuros (auditores, agentes, nuevos operadores) confían en esos artefactos regenerados como ground truth.

Esta sección responde **cómo**, no **si**: qué disciplina mantiene al spec sincronizado con el código durante ejecución multi-Charter **sin** que el paso de regeneración mienta sobre las partes que ya shippearon.

### Cuándo refrescar

Un spec-refresh está justificado cuando *cualquiera* de las siguientes condiciones se cumple:

1. **≥3 Charters cerrados contra el mismo spec** — el volumen de detalle de ejecución no reflejado es lo suficientemente alto para que las decisiones de scope del siguiente Charter arriesguen heredar premisas obsoletas.
2. **≥4 semanas calendario** desde el último refresh del spec (o desde la generación inicial) y ≥2 Charters cerrados en esa ventana.
3. **Conteo de AILOG `## Risk: R<N>(new, not in Charter)` sobre el spec excede ~6 a través de los Charters cerrados** — la anticipación de riesgo del spec subdescribe medibly el territorio.
4. **La user story del siguiente Charter toca infraestructura que los Charters previos refinaron empíricamente** (nuevas tablas/migraciones creadas, helpers extraídos, contratos cristalizados) y el spec describe el estado pre-refinamiento.

Si ninguna se cumple y el siguiente Charter apunta a un sub-sistema fresco que los Charters previos no tocaron, **omite el refresh**. La estabilidad del spec tiene valor; refrescar en cada Charter genera churn sin ganancia proporcional de claridad.

### Cómo refrescar: prompt scope-limited

**No** re-corras `/speckit-plan` con pizarra en blanco. El `plan.md` + `research.md` + `data-model.md` + `contracts/` + `quickstart.md` regenerados afirmarán cosas sobre user stories ya shippeadas que el código real no implementa.

En su lugar, invoca `/speckit-plan` con un **prompt scope-limited** que:

1. **Nombre la fase target explícitamente** (p.ej., "refresca planeación sólo para US5 — failover + tracking").
2. **Liste secciones bloqueadas que no deben cambiar** (p.ej., "las secciones Foundation, US1, US2, US3, US4 son inmutables — el código shippeado contra ellas es la ground truth, no el plan").
3. **Cite los AILOGs que documentan refinamientos** (p.ej., "ver AILOG-2026-05-11-043 §R5 para el patrón de reuso de `processed_events`; reflejar esto en el data model refrescado").
4. **Prohíba regenerar `tasks.md`** — ver subsección siguiente.

El output es un `plan.md` (y posiblemente `research.md` / `data-model.md` / `contracts/`) donde el contenido de la fase target es fresco y las secciones bloqueadas cargan hacia adelante el estado realmente shippeado, no el estado aspiracional original.

### Tres gates mecánicos post-refresh

Antes de mergear el PR del spec-refresh, tres gates corren secuencialmente:

**Gate (a) — Validación contra realidad del código.**
Para cada entidad no-target-phase en `data-model.md`, diff contra los `db/migrations/*.sql` reales (o fuente equivalente de schema). Para cada endpoint no-target-phase en `contracts/*.md`, diff contra signatures de handlers reales. Cualquier divergencia en una sección *bloqueada* bloquea el merge — eso es la regeneración mintiendo. Los adopters pueden scriptarlo contra su stack; un helper CLI (`straymark spec-drift`) está en el roadmap (ver #150 Ask 3).

**Gate (b) — Revisión granular hunk-por-hunk del diff.**
Corre `git diff specs/NNN-feature/` y revisa archivo-por-archivo, hunk-por-hunk. Ningún cambio a secciones bloqueadas se acepta sin un comentario de justificación explícito en el PR. El diff es lo suficientemente pequeño cuando es scope-limited para que esto sea factible en una sentada.

**Gate (c) — Split en dos PRs.**
Aterriza el spec-refresh como su propio PR. Revísalo contra el *código*, no contra el output plan-only. Luego llena el Charter target contra el spec refrescado en un PR *separado*. Mezclar los dos colapsa superficies de review: los reviewers ya no pueden distinguir si un hunk refleja nuevo estado shippeado o nuevo estado planeado.

### Por qué NO re-correr `/speckit-tasks` mid-ejecución

El archivo `tasks.md` acumula estado de trace de implementación durante ejecución: checkboxes `[X]` en tareas completadas, anotaciones `*CHARTER-NN: <commit-sha>*` citando qué Charter shippeó qué tarea, posiblemente marcadores `^skipped` con rationale. **Regenerar `tasks.md` destruye este estado.** El archivo se vuelve una lista fresca de tareas sin registro de lo que ya shippeó.

Disciplina: **nunca** re-corras `/speckit-tasks` mientras un spec está en medio de ejecución multi-Charter. En su lugar, **edita manualmente `tasks.md`** sólo para la fase target — añade tareas nuevas para el scope refrescado, deja las secciones ya-shippeadas (`[X]` + anotaciones `*CHARTER-NN:*`) intactas.

Si descubres que el `tasks.md` original tenía errores en secciones shippeadas (p.ej., una tarea fue incorrectamente marcada `[X]` cuando su trabajo se partió en dos Charters), corrígelo manualmente con un commit Git. Trata `tasks.md` como un registro histórico desde el punto de primera ejecución en adelante; ya no es un artefacto regenerable.

### Cadencia de re-evaluación de Constitution Check

El Constitution Check de SpecKit típicamente se corre una vez en tiempo `/speckit-plan`. En ejecución multi-Charter contra un solo spec, la pregunta de *cuándo* re-evaluar queda implícita. Para hacerla explícita:

- **Por-Charter (recomendado)** — re-evalúa Constitution Check al inicio de cada Charter nuevo declarado contra el spec. El check es barato (leer la constitución; comparar contra el scope declarado del Charter) y atrapa drift temprano, antes de que la ejecución se commitee.
- **Por-spec-refresh (obligatorio cuando el refresh sucede)** — cuando un refresh de `/speckit-plan` scope-limited aterriza, el PR de refresh debe re-correr Constitution Check contra el plan refrescado. Si la versión del framework se movió (p.ej., `fw-4.10.x → fw-4.14.x`), Constitution Check puede arrojar resultados distintos porque existen gates nuevos.
- **NO por-framework-bump aislado** — un `straymark update-framework` entre Charters *no* requiere un re-run inmediato de Constitution Check sobre el spec abierto. El check aplica en el siguiente boundary natural (siguiente declaración de Charter o spec-refresh).

Codificar esto como cadencia explícita (en lugar de "lo decide quien quiera") cierra una ambigüedad recurrente reportada por Sentinel post-CHARTER-17.

### Roadmap: `straymark spec-drift`

Un comando CLI análogo a `straymark charter drift`, pero operando a granularidad de spec — parsear `data-model.md` → entidades → diff contra `db/migrations/*.sql`; parsear `contracts/*.md` → endpoints → diff contra signatures de handlers. Mecanizaría el Gate (a) arriba.

Diferido deliberadamente a un Charter post-anuncio (tracked por separado). La superficie de CLI es significativa sólo para adopters cuyo formato de spec sigue las convenciones de SpecKit; la capa de detección de lenguaje (handlers Go vs Rust vs TypeScript vs Python; schemas SQL vs ORM-defined) es no-trivial y amerita su propio ciclo de diseño informado por stacks reales de adopters. La disciplina arriba (Gates a/b/c ejecutados manualmente) es el v0; el CLI es el v1 que mecaniza el gate más costoso.

## Anti-patrones

**No abras un Charter "por si acaso".** Un Charter sin un corte enviable claro se convierte en una wishlist. El operador termina cerrándolo como `closed: aborted` y la telemetría no significa nada.

**No abras un Charter por User Story.** Telemetría por US es demasiado ruidosa para informar estimaciones futuras. Agrega.

**No omitas el campo `originating_spec`.** Aunque el Charter envuelva trabajo que no tiene una spec de SpecKit, define `originating_ailogs:` en su lugar. Charters sin origen son un anti-patrón (señalan motivación no documentada).

**No corras `straymark charter audit` sin las CLIs auditoras disponibles.** La auditoría es orchestration-only — `straymark` no llama a APIs de LLM. Si no tienes N CLIs auditoras listas, salta el paso; cierra el Charter sin auditoría externa.

**No cambies status a `closed` antes del drift check + yaml de telemetría.** `straymark charter close` hace ambos atómicamente; el cierre manual salta invariantes.

**No re-corras `/speckit-tasks` mid-ejecución del spec.** Regenerar `tasks.md` destruye los marcadores `[X]` de completitud y las anotaciones `*CHARTER-NN:* …` que forman el rastro histórico. Ver "Mantención del spec durante ejecución multi-Charter" arriba para la ruta segura (edición manual sólo para la fase target).

## Cuándo este patrón no aplica

Este puente asume un flujo de feature manejado por SpecKit con implementación multi-tarea y multi-sesión. No aplica a:

- **Features de una sola sesión** — usa AILOGs solamente.
- **Trabajo solo de arquitectura, sin implementación** (ej. "diseñar el siguiente schema") — usa ADRs.
- **Refactors puros sin nuevo comportamiento** — usa AILOGs + etiqueta con `refactor:`.
- **Respuesta a incidentes y hotfixes** — usa INC + AILOG.
- **Entregables sólo de cumplimiento** (ej. refresh trimestral del DPIA) — usa el doc type relevante directamente.

Si tu trabajo encaja en alguno de los anteriores, *no declares Charter*. El costo del Charter excede el valor cuando no hay corte enviable que envolver.

## Ver también

- [`EMERGENT-OBSERVATION-DESIGN.md`](EMERGENT-OBSERVATION-DESIGN.md) — el meta-patrón que explica *por qué* el linkeo multi-fuente en este bridge produce observaciones emergentes durante ejecución multi-Charter.
- `STRAYMARK.md` §6 (Cuándo Documentar) y §15 (Charters como unidades acotadas de trabajo)
- `.straymark/templates/charter/charter-template.md` — template declarativo
- `.straymark/templates/charter/charter-telemetry-template.yaml` — template de telemetría
- `.straymark/schemas/charter.schema.v0.json` — JSON Schema del frontmatter declarativo
- `.straymark/schemas/charter-telemetry.schema.v0.json` — JSON Schema de telemetría
- `.claude/skills/straymark-charter-new/SKILL.md` (y equivalentes Gemini / agnóstico)

> **Contexto empírico citado** (issue #113): Suite Rust CLI/TUI greenfield, onboarding de Claude Opus 4.7 vía los puntos de entrada canónicos (`STRAYMARK.md`, constitución del proyecto, checklist de `CLAUDE.md`, skills `/straymark-*` disponibles, `/straymark-status`). Los Charters fueron *eventualmente* adoptados (2 Charters: foundation + MVP) sólo tras prompt explícito del usuario — confirmando que el gap era sistémico, no específico de la sesión. Este documento elimina ese gap.

---

*Idiomas*: [English](../../SPECKIT-CHARTER-BRIDGE.md) | Español | [简体中文](../zh-CN/SPECKIT-CHARTER-BRIDGE.md)
