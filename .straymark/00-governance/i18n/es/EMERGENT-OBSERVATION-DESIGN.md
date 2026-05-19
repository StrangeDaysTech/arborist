# Diseño de observación emergente — StrayMark

> Por qué los agentes que leen la documentación de StrayMark surfacean lo que no se les pidió: las propiedades estructurales y culturales que hacen detectable la disonancia entre fuentes canónicas, y la pirámide de patrones aplicados que ya instancian este meta.

**Idiomas**: [English](../../EMERGENT-OBSERVATION-DESIGN.md) | Español | [简体中文](../zh-CN/EMERGENT-OBSERVATION-DESIGN.md)

---

## Estado

**v0 — validado en N=1 dominio** (`StrangeDaysTech/sentinel`, Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) → Issue [#156](https://github.com/StrangeDaysTech/straymark/issues/156), 2026-04-21 hasta 2026-05-15).

Este documento nombra una *propiedad de diseño* del framework StrayMark que produjo un comportamiento emergente empíricamente observable. La propiedad **no es nueva** — ha estado presente desde que se canonizó `00-governance/` — pero no había sido *nombrada*, lo que la hacía invisible a la evolución del framework y por tanto en riesgo de erosión accidental. Nombrarla aquí la protege bajo la disciplina de validación de segundo dominio del Principio #12.

---

## Por qué existe este documento

Un agente trabajando en Sentinel surfaceó — **sin disparador explícito, sin solicitud del operador y sin un comando CLI diseñado para producir esa salida** — que `specs/002-commshub/plan.md` había acumulado doce aprendizajes empíricos no reflejados a lo largo de siete Charters consecutivos (CHARTER-07..17, ~1 mes), y que fillar CHARTER-18 contra el spec stale tenía ~50% de probabilidad de hallazgos críticos/altos en el próximo ciclo de auditoría. Esa observación produjo el ciclo upstream que cristalizó como `CHARTER-CHAIN-EVOLUTION.md` Pattern 1 en fw-4.16.0.

El comportamiento se reprodujo porque dos propiedades del aparato documental *coexistieron consistentemente*. Ninguna de las dos por sí sola habría sido suficiente. Nombrar ambas, y nombrar su composición, permite que la evolución futura del framework las preserve deliberadamente en lugar de por inercia.

El bridge `SPECKIT-CHARTER-BRIDGE.md` y el doc de chain-evolution `CHARTER-CHAIN-EVOLUTION.md` documentan *una aplicación* de este meta. Este documento nombra el meta en sí y enumera las otras aplicaciones ya shipped.

---

## Las dos propiedades de diseño

### Propiedad 1 — Cross-referencing estructural (linkeo formal)

El framework **no** delega el linkeo entre documentos a la intuición del agente ni a la prosa. Cada tipo de documento tiene campos de frontmatter *obligatorios* y secciones *canónicas* que declaran, en la propia estructura del documento, a qué otros documentos apunta y qué secciones de sí mismo están abiertas a tipos específicos de surfacing.

Instancias concretas que el agente encuentra durante lectura rutinaria:

- **Campos de linkeo en frontmatter** que resuelven a otros documentos StrayMark:
  - `originating_charter:` en frontmatter de AILOG / AIDEC ([`AGENT-RULES.md` §5](AGENT-RULES.md), [`SPECKIT-CHARTER-BRIDGE.md` sección Charter↔AILOG](SPECKIT-CHARTER-BRIDGE.md))
  - `originating_spec:` en frontmatter de Charter ([`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) §Frontmatter linkage)
  - `originating_ailogs:` en frontmatter de Charter (inverso de agregación)
  - `amends:` en frontmatter de AILOG de enmienda ([`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2)
  - `promoted_from_followup:` en frontmatter de TDE ([`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md))
  - `related:`, `supersedes:`, `superseded_by:` ([`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md))
- **Secciones canónicas dentro de plantillas** que contienen deltas en forma queryable:
  - `§Risk: R<N> (new, not in Charter)` en AILOGs ([`AGENT-RULES.md` §3](AGENT-RULES.md))
  - `## Follow-ups` por-AILOG ([`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md))
  - `## Batch Ledger` para AILOGs multi-batch
  - `## Historical correction (YYYY-MM-DD)` añadido al AILOG original en enmienda ([`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md))
- **Convenciones de IDs estables** que abaratan el linkeo:
  - `[TYPE]-[YYYY-MM-DD]-[NNN]-[descripción]` para documentos con fecha
  - `CHARTER-NN-slug` (sin timestamp, estable a través de renames)
  - `FU-NNN` monotónico global, nunca renumerado
- **Documentos bridge** que declaran relaciones canónicas entre capas:
  - `SPECKIT-CHARTER-BRIDGE.md` (Spec ↔ Charter ↔ AILOG)
  - `CHARTER-CHAIN-EVOLUTION.md` (Charter ↔ cadena de Charters ↔ Spec refresh)
- **Comandos CLI** que cruzan fuentes mecánicamente:
  - `straymark charter drift <id>` (scope declarado ↔ commits)
  - `straymark charter refresh-suggest <module>` (media móvil de telemetría ↔ necesidad de refresh)
  - `straymark validate` (frontmatter ↔ schema ↔ integridad de links)

**El punto de la Propiedad 1**: cuando el agente encuentra una divergencia entre dos fuentes, la divergencia es *estructuralmente visible* — no enterrada en prosa. El agente no necesita inventar la conexión; la conexión está declarada por el framework.

### Propiedad 2 — Permiso cultural sin gatekeeping

El framework le da al agente permiso explícito y reiterado para surfacear más allá de la tarea pedida — y empareja ese permiso con autonomía para *actuar* sobre el surfacing (crear el AILOG, fillar el TDE, abrir el AIDEC) sin pre-aprobación. El operador retiene priorización, no creación.

Pasajes concretos que el agente encuentra:

- **`AGENT-RULES.md` §6 "Be Proactive"** — *"Identify potential risks, Suggest improvements when evident, Alert about technical debt"*.
- **`AGENT-RULES.md` §6 "Be Transparent"** — *"Explain the reasoning behind decisions, Document considered alternatives, Admit uncertainty when it exists"*.
- **`AGENT-RULES.md` §12 Audit Checkpoint** — *"the agent proactively offers an external multi-model audit"* — institucionaliza el *acto* de surfacear como parte del workflow.
- **`PRINCIPLES.md` §2 "AI Agent Transparency"** — *"Not hide relevant information"*.
- **`AGENT-RULES.md` §3 tabla de autonomía "Create Freely"** — la creación de AILOG, AIDEC, TDE no requiere pre-aprobación; el agente fila y el operador prioriza.
- **`FOLLOW-UPS-BACKLOG-PATTERN.md` script auto-append** — `check-followups-drift.sh --apply` añade entradas FU-NNN al registro central sin intervención del operador.

**El punto de la Propiedad 2**: el agente externalizó *"¿debo decir algo?"* en *"¿existe una sección canónica donde esto encaja?"*. Si la respuesta es sí, surfacear no es un juicio de valor — es ejecución de una regla documentada. El costo de surfacear es bajo porque el destino está pre-construido.

### Por qué importa la composición

La Propiedad 1 *sola* — linkeo formal sin permiso cultural — produciría un corpus queryable que ningún agente se atreve a consultar proactivamente. La Propiedad 2 *sola* — permiso sin cross-referencing estructural — produciría surfacing vago ("creo que algo podría estar mal en algún lado") que los operadores no pueden accionar.

Compuestas, producen el comportamiento observado: un agente lee los AILOGs, cuenta entradas `R<N>(new, not in Charter)` que divergen materialmente del spec originador, ve que el spec no se ha refrescado en un mes, y — porque `§6 Be Proactive` le dijo que alerte y porque la divergencia tiene un nombre en el vocabulario del framework — surfacea *el delta específico y estructuralmente fundado* al operador antes de proceder con la tarea pedida.

Este es el meta-patrón.

---

## Caso empírico: detección de spec-drift en Sentinel

El caso está descrito en detalle en los Issues [#150](https://github.com/StrangeDaysTech/straymark/issues/150) y [#156](https://github.com/StrangeDaysTech/straymark/issues/156), y codificado como Pattern 1 en [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md). La secuencia comprimida:

1. Sentinel corre `specs/002-commshub/plan.md` (committed 2026-04-21) a través de CHARTER-07..17 durante ~1 mes. Doce aprendizajes empíricos se acumulan a lo largo de la cadena de AILOGs en secciones `§Risk: R<N>(new, not in Charter)` y `## Follow-ups`. La propagación de patrones (formas de handler, convenciones de reuso de tablas, helper RLS, etc.) cristaliza durante ejecución.
2. CHARTER-18 está por ser declarado. El agente — sin instrucción de hacerlo — triangula `plan.md` contra los AILOGs (donde las entradas `§Risk` nombran los gaps del spec) y contra el código (donde `straymark charter drift` habría detectado divergencia por-Charter de haberse corrido cross-Charter). El linkeo `originating_spec:` en cada Charter, `originating_charter:` en cada AILOG, y la convención `§Risk: R<N>` del framework hacen la triangulación mecánica, no heroica.
3. El agente surfacea *"si fillamos CHARTER-18 leyendo el plan stale, los hallazgos H1/M1 del próximo ciclo de auditoría serán remediación atómica pre-close de divergencias — ~50% de probabilidad de ≥1 hallazgo crítico/alto por herencia de premisa stale"* — citando AILOGs específicos por ID y referencias específicas de código.
4. El operador fila el Issue #150 como RFC. El AIDEC local de Sentinel documenta la disciplina propuesta de refresh con scope limitado + tres gates mecánicos.
5. El Issue #156 upstrea el patrón. `CHARTER-CHAIN-EVOLUTION.md` Pattern 1 aterriza en fw-4.16.0 con el slot de telemetría `pre_declare_refresh:`, el helper `straymark charter refresh-suggest`, y el contrato de tabla de aprendizajes categorizados.

La observación es empíricamente reproducible: cualquier spec que produzca ≥3 Charters separados por ≥1 semana exhibirá algún grado de drift plan-vs-código, y un agente leyendo la documentación del framework tiene el permiso estructural y cultural para detectarlo y surfacearlo.

---

## Pirámide de instancias — aplicaciones del meta-patrón

El meta-patrón se sitúa por encima de varios patrones ya canonicalizados. Cada uno es una *aplicación* de la misma composición subyacente (linkeo formal + permiso cultural) a un par de fuentes específico.

| Aplicación | Par de fuentes | Dónde canonicalizado |
|---|---|---|
| SpecKit refresh pre-declare (Pattern 1) | spec ↔ AILOGs ↔ código | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 1 |
| Enmienda post-close driven-by-audit (Pattern 2) | hallazgos de audit ↔ Charter cerrado | [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2 |
| Detección de drift de Charter | scope declarado ↔ commits | [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) + `straymark charter drift` |
| Drift de follow-ups backlog | `§Follow-ups` por-AILOG ↔ registro central | [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) + `check-followups-drift.sh` |
| Escalación TDE-vs-`R<N>` | `§Risk: R<N>` acumulado ↔ TDE backlog | [`AGENT-RULES.md`](AGENT-RULES.md) §3 |
| Audit checkpoint externo | estado implementation-complete ↔ revisión multi-modelo | [`AGENT-RULES.md`](AGENT-RULES.md) §12 |

Estas no son convenciones ad-hoc. Comparten la misma forma: *dos fuentes canónicas conectadas por linkeo de frontmatter o sección, con el agente permitido (a veces obligado) a surfacear el delta*. El próximo eje de aplicación — sea cual sea — se reconocerá en esta tabla.

---

## Anti-patrones: cómo se rompe el meta

El meta-patrón es frágil. Cada uno de los siguientes, si se introduce, regresa la capacidad del framework para producir observaciones emergentes.

- **Linkeo de frontmatter como opcional**. Si un nuevo tipo de documento ships con `related:` / `originating_*` como advisory en lugar de obligatorio, el grafo de cross-referencing desarrolla blind spots y el agente pierde la capacidad de triangular a través de ese tipo.
- **Secciones canónicas colapsadas en prosa**. Si `§Risk: R<N>` se reemplaza por *"discusión de riesgos"*, la queryability se evapora. El agente ya no puede contar entradas `R<N>` para detectar el umbral de saturación que dispara `refresh-suggest`. La prosa libre no es queryable; las secciones estructuradas sí.
- **Gatekeeping en docs creados por agente**. Requerir pre-aprobación para fillar AILOG / AIDEC / TDE mata la Propiedad 2. El agente regresa a surfacear sólo lo pedido, porque el costo de surfacear sube por encima del beneficio local.
- **Telemetría sin signals emergentes**. Si los schemas de `.telemetry.yaml` evolucionan sin preservar signals como `r_n_plus_one_emergent_count`, el operador pierde visibilidad de qué tan seguido el agente está surfaceando riesgos emergentes. El feedback loop se rompe; el meta se vuelve invisible a la evolución del framework.
- **Comandos CLI que bypasean la superficie**. Una CLI que emite decisiones directamente (sin AILOG escrito, sin sección `R<N>` poblada) bypasea la superficie estructural. La triangulación río abajo del agente se degrada porque el par de fuentes ya no está conectado vía documentos.

---

## Ejes de aplicación abiertos — dónde el meta podría replicarse

La auditoría subyacente a este documento identificó cuatro loci donde la infraestructura estructural existe *parcialmente* pero el permiso cultural o el patrón de aplicación no han sido nombrados. Son candidatos a futura aplicación del meta, no commitments de shipping.

- **MCARD ↔ código del modelo desplegado** — `TEMPLATE-MCARD.md` existe; no hay campo `model-version-at-close` en telemetría de Charter, no hay campo de linkeo AILOG `deployed_mcard:`, no hay patrón de detección de drift. Un despliegue de modelo que diverge de la MCARD on file es actualmente invisible.
- **SBOM ↔ lockfiles** — `AI-RISK-CATALOG.md` §RISK-004 menciona mantenimiento de SBOM para componentes AI; no hay campo canónico de AILOG enlazando a SBOM, no hay script de drift (análogo a `check-followups-drift.sh`) que compare SBOM declarada contra `package.lock` / `requirements.txt` real, no hay signal de telemetría para eventos de cambio de dependencias.
- **ADR vigente ↔ implementación que contradice** — el schema de `.telemetry.yaml` captura `decisions_contradicting_prior_adrs` pero ningún protocolo le dice al agente *cuándo* surfacear una contradicción que observa durante implementación. El signal existe; la convención de surfacing no.
- **Constitution Check ↔ bump de versión del framework** — `SPECKIT-CHARTER-BRIDGE.md §Constitution Check re-evaluation cadence` codifica la cadencia verbalmente; no se dispara alerta automática en `straymark update-framework`. Un bump del framework entre Charters puede cambiar los gates de Constitution silenciosamente.

Estos cuatro están trackeados en un único Issue upstream de RFC (filed después de que este documento aterrice). Cada uno requiere validación N=1 empírica de un adopter antes de cristalizar como patrón nombrado — aplica el Principio #12.

---

## Authority / flujo de aceptación para nombrar nuevas meta-aplicaciones

El mismo flujo de aceptación upstream que `CHARTER-CHAIN-EVOLUTION.md` documenta aplica recursivamente a este meta. Un nuevo eje de aplicación (uno de los cuatro arriba, o un quinto que emerja) aterriza así:

1. **RFC adopter-local** en `.straymark/06-evolution/<axis>-rfc.md` describiendo la conexión estructural que ya existe (o se está añadiendo) y la regla de permiso cultural que el agente debe seguir.
2. **Issue upstream** espejando el RFC, citando los AILOGs/Charters/telemetría donde ocurrió la observación empírica.
3. **Aceptación upstream** como: (a) actualización del template / schema / doc de governance relevante para añadir la pieza estructural faltante (campo de frontmatter, sección canónica, signal de telemetría); (b) adición del eje a la tabla "Pirámide de instancias" en este documento; (c) scaffolding CLI opcional para detección mecánica.
4. **Validación de segundo dominio** antes de que las adiciones de schema del eje gradúen de opcionales a recomendadas.

Este propio documento instancia el paso 3.b para el meta — el output de aceptación upstream de reconocer que las aplicaciones existentes comparten una propiedad subyacente única.

---

## Preguntas abiertas

- **Operacionalización de "divergencia material"**. El texto del Principio #8 ([`PRINCIPLES.md`](PRINCIPLES.md)) deja "material" al juicio del agente. Los umbrales por-aplicación (Pattern 1 usa media móvil `r_n_plus_one_emergent_count > 6`) se calibran empíricamente. Si un umbral cross-axis es alcanzable, o si cada eje debe calibrar el suyo, queda abierto.
- **Consolidación de telemetría**. Cada aplicación emite actualmente su propio slot de telemetría (`pre_declare_refresh:`, `post_close_amendment:`, `r_n_plus_one_emergent_count`). Un contador consolidado *"observaciones emergentes surfaceadas en este Charter"* podría hacer el meta visible a nivel de métricas. Diferido — agregación prematura arriesga perder granularidad de signal por-eje.
- **Onboarding de adopters**. Adopters nuevos leyendo `STRAYMARK.md` por primera vez deberían encontrar el meta lo suficientemente temprano como para reconocer el patrón cuando lo experimenten. Si eso vive en `QUICK-REFERENCE.md`, en `STRAYMARK.md` mismo, o en una nueva sección de onboarding, queda abierto.

---

## Relacionados

- [`PRINCIPLES.md`](PRINCIPLES.md) §8 — *Surfacing de disonancia entre fuentes* (la regla cultural, condensada).
- [`AGENT-RULES.md`](AGENT-RULES.md) §6 — *Be Proactive* (el mandato operativo); §3 — *TDE vs `R<N>`* (una superficie de aplicación); §12 — *Audit Checkpoint* (surfacing institucionalizado).
- [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) — Pattern 1, Pattern 2 (las dos aplicaciones de más alto nivel).
- [`SPECKIT-CHARTER-BRIDGE.md`](SPECKIT-CHARTER-BRIDGE.md) — Charter como la capa bridge donde el linkeo de la Propiedad 1 es más denso.
- [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md) — detección de drift en la superficie per-AILOG ↔ registro.
- [`DOCUMENTATION-POLICY.md`](DOCUMENTATION-POLICY.md) — canon de frontmatter y campo `related:`.
- [`../../STRAYMARK.md`](../../STRAYMARK.md) §15 — Charter como la unidad bounded donde las aplicaciones convergen.

---

*StrayMark fw-4.17.0 | [GitHub](https://github.com/StrangeDaysTech/straymark) | Issue [#150](https://github.com/StrangeDaysTech/straymark/issues/150) · [#156](https://github.com/StrangeDaysTech/straymark/issues/156)*
