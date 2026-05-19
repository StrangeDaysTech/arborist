# Patrón de Backlog de Follow-ups - StrayMark

> Convención reproducible para gestionar entradas acumuladas de `§Follow-ups` y `R<N> (new, not in Charter)` a lo largo de muchos AILOGs y Charters.

**Idiomas**: [English](../../FOLLOW-UPS-BACKLOG-PATTERN.md) | Español | [简体中文](../zh-CN/FOLLOW-UPS-BACKLOG-PATTERN.md)

---

## Estado

**v0 — validado en N=1 dominio** (`StrangeDaysTech/sentinel` CHARTER-12, 2026-05-06).

Esto es una **convención**, no una funcionalidad del CLI. Los adopters la reproducen localmente con markdown + un script bash portable. El patrón puede evolucionar a un subcomando `straymark followups` una vez que un segundo adopter lo valide (ver [Preguntas abiertas](#preguntas-abiertas)).

---

## Cuándo aplica este patrón

La convención per-AILOG `§Follow-ups` de StrayMark funciona en tiempo de escritura — cuando se crea un AILOG, el implementador documenta lo que se difiere a Charters subsiguientes o triggers operativos. Eso funciona bien hasta que la lista acumulada crece más allá de lo que un operador puede escanear de memoria.

Adopta este patrón cuando se cumpla **cualquiera** de estas condiciones:

- El proyecto ha acumulado **~20 o más AILOGs** con secciones `§Follow-ups` no triviales.
- Los operadores piden repetidamente a los agentes "lista qué está pendiente en el proyecto" y la respuesta requiere un escaneo multi-archivo.
- Un follow-up de tipo "haz esto cuando llegue X" estuvo a punto de perderse porque el AILOG originador nunca fue releído después de que llegó X.
- Una retrospectiva de Charter aflora follow-ups que deberían haber sido clasificados como `closed` semanas antes pero nunca fueron indexados.

Por debajo de ese volumen, la convención per-AILOG por sí sola es suficiente — adoptar este patrón temprano agrega overhead de mantenimiento sin retorno.

---

## Forma

### Archivo de registro

Único archivo markdown en la ruta canónica:

```
.straymark/follow-ups-backlog.md
```

### Frontmatter (YAML)

```yaml
---
last_scan: 2026-05-06
last_scan_range: AILOG-NNNN-NN-NN-NNN..AILOG-NNNN-NN-NN-NNN  # opcional — primer..último AILOG cubierto
schema_version: v0
total_open: 0           # cuenta de entradas actualmente `open`
total_promoted: 0       # cuenta de entradas actualmente `promoted` (agregado en schema v0.1 — ver "Promoción a TDE")
total_closed_in_session: 0   # cuenta de entradas `closed` desde la última sesión (opcional, operator-maintained)
total_phase_blocked: 0  # cuenta de entradas `phase-blocked` (opcional)
buckets:
  - ready
  - time-triggered
  - charter-triggered
  - phase-blocked
  - operational
fully_extracted_ailogs:
  - AILOG-2026-04-11-001
  - AILOG-2026-04-12-001
  # ... una entrada por cada AILOG cuyos follow-ups han sido procesados
---
```

Los contadores `total_*` son **metadatos operator-maintained**. El script de drift no los actualiza automáticamente — viven en el header para que un vistazo a inicio de sesión muestre el pulso del registro sin scrollear por buckets. `total_promoted` se canonicalizó en schema v0.1 (señal empírica del adopter Sentinel, fw-4.13.1) para reflejar el patrón existente de `total_open` / `total_closed_in_session` / `total_phase_blocked`.

La lista `fully_extracted_ailogs` es el **metadato cargante** para la detección de drift. Todo AILOG cuyas entradas de `§Follow-ups` y `R<N>` han sido transferidas al registro (o explícitamente clasificadas como superseded) pertenece a esta lista. La detección de drift compara esta lista contra los AILOGs que tienen contenido de follow-ups en el repo.

### Buckets

Cinco buckets organizan las entradas por tipo de trigger:

- `ready` — accionable ahora, sin dependencia de trigger externo.
- `time-triggered` — trigger basado en calendario (ciclo de auditoría, revisión periódica).
- `charter-triggered` — bloqueado por un Charter futuro que toque el área relevante.
- `phase-blocked` — bloqueado por un componente o fase futura que aún no existe.
- `operational` — decisión manual del operador o acción de sistema externo.

### Esquema de entrada

Cada entrada dentro de un bucket sigue esta forma:

```markdown
### FU-NNN — <descripción corta>
- **Origin**: AILOG-NNNN-NN-NN-NNN <pointer a la sección fuente>
- **Status**: open | in-progress | closed | superseded | promoted
- **Trigger**: ready | <fecha calendario> | when <X> | <otro>
- **Destination**: <id de Charter, "operations", fase futura, o TDE-YYYY-MM-DD-NNN>
- **Cost**: <estimación de esfuerzo>
- **Notes**: <contexto libre>
- **Promoted to**: <id de TDE, cuando Status: promoted — ver "Promoción a TDE" abajo>
```

`FU-NNN` es monotónicamente creciente a lo largo de la vida del registro; no se renumera cuando las entradas se cierran.

### Vocabulario de status

- `open` — pendiente, sin acción aún.
- `in-progress` — un Charter ha sido declarado o está en ejecución para atender esta entrada.
- `closed` — entrada resuelta (Charter mergeado, tarea operativa hecha, tiempo transcurrido y revisado).
- `superseded` — atendida por otro trabajo que no referenció esta entrada directamente.
- `promoted` — la entrada fue elevada a un documento TDE porque cumple los criterios de deuda transversal (ver "Promoción a TDE" abajo). El campo `Promoted to:` carga el id del TDE.

Las entradas closed, superseded y promoted permanecen en el archivo (historia auditable). Los operadores pueden moverlas a una sección `## Bucket: closed` al final para limpieza visual, pero nunca se eliminan.

---

## Promoción a TDE

Algunas entradas FU no son solo tareas diferidas — describen **deuda técnica transversal** que merece su propio documento de gobernanza (TDE). Los criterios para promoción reflejan la desambiguación TDE-vs-`R<N>` en `AGENT-RULES.md §3`:

- La entrada es *herencia de un Charter previo* (ya vivió ≥1 cierre de Charter sin remediación).
- La entrada *aplica a múltiples módulos o múltiples Charters* — el registro central la ha fragmentado en bullets que comparten una causa raíz.
- La entrada *requiere un Charter dedicado fuera del envelope de scope actual* para remediarse.
- La entrada *requiere priorización o asignación humana* que la revisión periódica del operador no puede decidir desde el bullet solo (matriz impact × effort, ownership).

Cuando cualquiera de estos se cumple, promueve la entrada FU a un documento TDE bajo `.straymark/06-evolution/technical-debt/`:

1. Crea el TDE vía `/straymark-new tde` (o `straymark new --type tde`). Llena `impact`, `effort`, `type`, y las secciones del body desde el contexto de la entrada FU.
2. Agrega `promoted_from_followup: FU-NNN` al frontmatter del TDE para trazabilidad.
3. En la entrada FU, establece `Status: promoted`, `Destination: TDE-YYYY-MM-DD-NNN`, y agrega `Promoted to: TDE-YYYY-MM-DD-NNN`. Mueve la entrada a la sección `## Bucket: closed` si mantienes una; si no, déjala en lugar con el nuevo status.

La entrada FU **no se elimina** tras la promoción — su presencia en el registro es el rastro auditable que muestra de dónde vino el TDE.

### Dos formas de promoción — promoción-de-existente vs retroactiva-en-la-creación

El workflow anterior cubre el **caso estándar**: una entrada FU `open` ya existe en el registro y se eleva a un TDE durante revisión periódica. Existe un segundo caso igualmente válido que emergió empíricamente del retrospectivo de Sentinel CHARTER-13:

- **Promoción de entrada existente** — un FU fue registrado (típicamente vía `--apply`) como `open` semanas o Charters atrás, vivió ≥1 cierre de Charter sin resolución, y cumple los cuatro criterios. Flujo estándar.
- **Promoción retroactiva en la creación** — la deuda se reconoce como TDE-worthy *durante* un retrospectivo (ceremonia de cierre de Charter, ciclo de auditoría, redacción de RFC) y nunca existió como FU `open`. Se crea primero el TDE; se agrega una entrada FU al registro *con `Status: promoted`* desde el nacimiento, proporcionando el rastro auditable desde el TDE hacia el contexto originador (un `R<N>` en un AILOG, un finding del calibrador, una clasificación diferida).

Ambas formas producen el mismo estado final en el registro: una entrada con `Status: promoted` y un puntero `Promoted to: TDE-YYYY-MM-DD-NNN`. La diferencia es si la entrada pre-existía como `open` o nació `promoted`. El script de drift las trata idénticamente (no diferencia por status de nacimiento), y las analíticas del adopter que cuentan `total_promoted` obtienen el mismo número en ambos casos.

Ante la duda, prefiere crear la entrada FU — aunque sea retroactivamente — porque cross-referencia el TDE de vuelta al AILOG / número-R / contexto fuente que disparó el reconocimiento. Un TDE con `promoted_from_followup: FU-NNN` apuntando a una entrada que existe en el backlog es más navegable que uno apuntando a un FU ficticio.

### Cuándo promover

- **Revisión periódica** — cuando el operador hace el pase manual de reclasificación, promueve cualquier entrada que haya vivido ≥2 cierres de Charter sin resolución y cumpla los criterios anteriores.
- **Cierre de Charter** — al revisar entradas que el Charter recién cerrado resolvió, si encuentras entradas que *no* fueron resueltas y cumplen los criterios anteriores, promuévelas en vez de dejarlas como `open`.
- **Pre-declaración de Charter** — si estás a punto de declarar un Charter y notas que el registro contiene entradas que este Charter *parcialmente* atendería, la porción no atendida puede pertenecer como TDE en vez de como otro FU diferido.

El script de drift (`scripts/check-followups-drift.sh`) **no se extiende** para candidatos a promoción en v0 — la promoción es operator-driven. Una mejora futura v1 podría flagear entradas que cumplan la heurística "vivió ≥2 Charters", pero eso cristaliza tras un segundo adopter que valide el patrón (misma puerta que el resto de v0 → v1).

---

## Detección de drift

Un pequeño script bash es la capa de verificación que mantiene el registro sincronizado con nuevos AILOGs. El script vive en el repo del adopter (ruta sugerida: `scripts/check-followups-drift.sh`) y tiene tres modos.

### Modos

- **Default** — escanea AILOGs modificados en `git diff origin/main..HEAD` (con fallback a `HEAD~1..HEAD`). Avisa sobre cualquier AILOG con contenido `§Follow-ups` / `R<N> (new)` que no esté en `fully_extracted_ailogs`. Sale con 1 si hay drift.
- **`--apply`** — mismo escaneo, pero auto-agrega nuevas entradas bajo `## Bucket: ready` con ids `FU-NNN` auto-generados y agrega el id del AILOG a `fully_extracted_ailogs`. El operador reclasifica al bucket correcto después.
- **`--scan-all`** — escanea cada AILOG en el proyecto (barrido completo periódico).

### Granularidad per-AILOG vs per-bullet

El tracking es **per-AILOG**, no per-bullet. Un AILOG está totalmente extraído (su id está en `fully_extracted_ailogs` — confiar en el registro) o no lo está (extraer todo). El matching per-bullet requeriría fingerprinting (hashing de texto o comparación fuzzy), que produce falsos positivos cada vez que una entrada del registro parafrasea el bullet del AILOG — y las entradas curadas siempre parafrasean.

El costo de la granularidad per-AILOG: cuando se agrega un follow-up a un AILOG ya extraído tras el cierre del Charter, la detección de drift no lo detecta. La remediación es manual del operador — quitar el AILOG de `fully_extracted_ailogs` y re-correr con `--apply`. Este trade-off es intencional para v0 porque la mayoría de AILOGs son write-once tras el cierre del Charter.

### Plantilla de script

Una implementación de referencia (~290 líneas de bash POSIX) está en `StrangeDaysTech/sentinel` en `scripts/check-followups-drift.sh`. Cópiala a tu repo y ajusta las constantes al inicio:

```bash
BACKLOG_FILE=".straymark/follow-ups-backlog.md"
AILOG_DIR=".straymark/07-ai-audit/agent-logs"
```

El script usa solo `awk` y `grep` — sin jq, sin yq, sin dependencias pesadas. Portable entre Linux y macOS.

---

## Integración con agentes

El agente (Claude / Gemini / etc.) se vuelve el mantenedor primario del registro. Agrega a tu `CLAUDE.md` / `AGENT.md`:

```markdown
## Backlog de follow-ups

- **Inicio de sesión**: revisar `.straymark/follow-ups-backlog.md` para saber qué está pendiente en el proyecto.
- **Checklist pre-commit**: ¿creaste o modificaste algún AILOG con entradas `## Follow-ups` o `R<N> (new, not in Charter)`? → ejecuta `scripts/check-followups-drift.sh --apply` para extender el backlog en el mismo commit.
- **Post-cierre de Charter**: revisar entradas que el Charter resolvió; marcarlas `closed` (con el id del Charter de cierre en `Notes`) o `superseded`. Para entradas no resueltas que cumplen los criterios de TDE (herencia de Charter previo, transversal, requiere Charter dedicado, necesita priorización humana), promuévelas a un documento TDE (ver "Promoción a TDE" en este patrón + `AGENT-RULES.md §3`).
```

Esto hace al agente el mantenedor, al script la capa de verificación, y al operador el revisor periódico (re-bucketing, marcar closed, podar superseded, promover a TDE cuando los criterios aplican).

---

## Walkthrough de adopción

Para un adopter empezando desde cero:

1. Crear `.straymark/follow-ups-backlog.md` con el frontmatter de arriba (lista `fully_extracted_ailogs:` vacía inicialmente) y los cinco headers `## Bucket: <name>`.
2. Copiar el script de referencia desde `StrangeDaysTech/sentinel` a `scripts/check-followups-drift.sh`. Ajustar `AILOG_DIR` si tus AILOGs viven en otro lado.
3. Ejecutar `scripts/check-followups-drift.sh --scan-all --apply` para sembrar el registro desde AILOGs existentes.
4. Reclasificar manualmente las entradas auto-generadas en `## Bucket: ready` a los buckets correctos. Esto es triage one-time, típicamente 30-60 min para un backlog de ~50 entradas.
5. Agregar el bloque de integración con agentes a `CLAUDE.md` / `AGENT.md`.
6. Opcionalmente conectar `scripts/check-followups-drift.sh` a un pre-commit hook para enforcement duro.

Para un adopter migrando desde tracking ad-hoc: el mismo flujo, pero el paso 4 puede requerir decidir qué entradas ya están `closed` o `superseded` — esa clasificación es lo que hace al registro útil.

---

## Implementación de referencia

`StrangeDaysTech/sentinel` CHARTER-12, mergeado 2026-05-06:

- PR de implementación: [sentinel#53](https://github.com/StrangeDaysTech/sentinel/pull/53) (registro + script + adiciones a CLAUDE.md).
- PR de cierre: [sentinel#54](https://github.com/StrangeDaysTech/sentinel/pull/54) (verificación post-merge + cierre del Charter).

Contexto empírico: 47 entradas sembradas desde la retrospectiva CHARTER-08 → CHARTER-11 (~30 min de triage multi-agente). La cadena demostró el gap que motivó el patrón — sin el registro, el operador no podía ver "qué está pendiente en el proyecto" sin reclasificar los follow-ups de cada Charter en aislamiento. Con el registro, la revisión de inicio de sesión es una sola lectura de archivo.

---

## Preguntas abiertas

Estas no se resuelven en v0. Revisiones futuras de este patrón, o un helper CLI, pueden atenderlas:

- **Heurística de clasificación de bucket**. Hoy `--apply` vuelca cada entrada nueva a `## Bucket: ready`; el operador reclasifica manualmente. Una heurística usando `tags` del AILOG y `effort_estimate` del Charter podría sugerir un bucket automáticamente.
- **Validación de schema**. El registro sigue un esquema tácito; aún no existe `.straymark/schemas/follow-ups-backlog.schema.v0.json`. La validación hoy es revisión humana.
- **Integración con el ciclo de auditoría**. Cuando `straymark charter audit --merge-reports` produce findings de deuda real que no son remediados atómicamente pre-cierre, esos findings viven solo en `.straymark/audits/<id>/review.md`. No fluyen automáticamente al registro central. Aflorarlos automáticamente cerraría un gap conocido.
- **Semántica `closed` vs `superseded`**. Hoy la diferencia es si el trabajo de resolución referenció explícitamente la entrada. Una convención más estricta puede emerger.
- **Cristalización como CLI `straymark followups`**. Una vez que un segundo adopter valide el patrón, el framework puede shippear un subcomando que espeje al trío existente `straymark charter`: `list` / `close` / `drift`. Los adopters en v0 (este patrón) migran borrando su script local y cambiando la instrucción del agente.

---

## Créditos

Contribuido vía [issue #111](https://github.com/StrangeDaysTech/straymark/issues/111) por el adopter Sentinel. Fundamento empírico: cadena CHARTER-08 → CHARTER-11 en `StrangeDaysTech/sentinel`. Autor: Claude Opus 4.7 a nombre del operador José Villaseñor Montfort.

---

## Relacionado

- [EMERGENT-OBSERVATION-DESIGN.md](EMERGENT-OBSERVATION-DESIGN.md) — el meta-patrón que esta convención de detección de drift instancia en la superficie per-AILOG ↔ registro.
- [CHARTER-CHAIN-EVOLUTION.md](CHARTER-CHAIN-EVOLUTION.md) — patrón hermano que opera a nivel de cadena (Pattern 1) y a nivel de ciclo (Pattern 2).
- [AGENT-RULES.md §3](AGENT-RULES.md) — criterios de escalación TDE-vs-`R<N>` que pueden promover follow-ups a entradas de deuda dedicadas.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
