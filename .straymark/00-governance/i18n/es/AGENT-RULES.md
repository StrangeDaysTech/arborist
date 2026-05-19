# Reglas para Agentes IA - StrayMark

> Este documento define las reglas que todos los agentes de IA deben seguir cuando trabajan en proyectos bajo StrayMark.

**Idiomas**: [English](../../AGENT-RULES.md) | Español | [简体中文](../zh-CN/AGENT-RULES.md)

---

## 1. Identificación Obligatoria

### Al Iniciar una Sesión

Cada agente debe identificarse con:
- Nombre del agente (ej.: `claude-code-v1.0`, `cursor-v1.0`, `gemini-cli-v1.0`)
- Versión del agente si está disponible

### En Cada Documento

Incluir en el frontmatter:
```yaml
agent: agent-name-v1.0
confidence: high | medium | low
```

---

## 2. Cuándo Documentar

### OBLIGATORIO - Crear documento

| Situación | Tipo | Notas |
|-----------|------|-------|
| Complejidad de código sobre el umbral | AILOG | Ejecutar `straymark analyze <archivos-modificados> --output json`. Si `summary.above_threshold > 0`, crear AILOG (umbral por defecto: 8). **Alternativa**: si el CLI no está disponible, usar heurística de >20 líneas de lógica de negocio |
| Decisión entre 2+ alternativas técnicas | AIDEC | Documentar alternativas |
| Cambios en auth/autorización/PII | AILOG + ETH | `risk_level: high`, ETH requiere aprobación |
| Cambios en API pública o esquema de BD | AILOG | `risk_level: medium+`, considerar ADR |
| Cambios en modelos ML o prompts de IA | AILOG | `risk_level: medium+`, revisión humana requerida |
| Integración con servicio externo | AILOG | - |
| Adición/eliminación/actualización de dependencias críticas de seguridad | AILOG | Revisión humana requerida |
| Cambios que afectan el ciclo de vida del sistema de IA (despliegue, retirada) | AILOG + ADR | Revisión humana requerida |
| Cambios en instrumentación OTel (spans, atributos, pipeline) | AILOG | Tag `observabilidad`, ver §9 |
| Deuda técnica transversal detectada durante la implementación | TDE | Ver §3 "TDE vs `R<N>` (new, not in Charter)" para el criterio de desambiguación |

### PROHIBIDO - No documentar

- Credenciales, tokens, API keys
- Información personal identificable
- Secretos de cualquier tipo

### OPCIONAL - No requiere documento

- Cambios de formato (espacios, indentación)
- Correcciones de erratas
- Comentarios de código
- Cambios menores de estilo

---

## 3. Límites de Autonomía

### Crear Libremente

| Tipo | Descripción |
|------|-------------|
| AILOG | Logs de acciones realizadas |
| AIDEC | Decisiones técnicas tomadas |

### Crear Borrador → Requiere Aprobación Humana

| Tipo | Descripción |
|------|-------------|
| ETH | Revisiones éticas |
| ADR | Decisiones arquitectónicas |

### Proponer → Requiere Validación Humana

| Tipo | Descripción |
|------|-------------|
| REQ | Requisitos del sistema |
| TES | Planes de prueba |

### Crear Borrador → Requiere Aprobación Humana (tipos nuevos)

| Tipo | Descripción |
|------|-------------|
| SEC | Evaluaciones de seguridad (`review_required: true` siempre) |
| MCARD | Tarjetas de modelo/sistema (`review_required: true` siempre) |
| DPIA | Evaluaciones de impacto en protección de datos (`review_required: true` siempre) |

### Crear Libremente (tipos nuevos)

| Tipo | Descripción |
|------|-------------|
| SBOM | Lista de materiales de software (inventario factual) |

### Solo Identificar → Humano Prioriza

| Tipo | Descripción |
|------|-------------|
| TDE | Deuda técnica |
| INC | Conclusiones de incidentes |

### TDE vs `R<N> (new, not in Charter)`

Existen dos superficies para la deuda emergente. No son intercambiables — elige la que coincida con el ciclo de vida del trabajo, no la que tengas más a mano.

**Registra un `R<N> (new, not in Charter)` en la sección `§Risk` del AILOG** cuando la deuda:

- Está *acotada al Charter en ejecución* o al siguiente Charter en la secuencia.
- Se resuelve como un diferimiento documentado, un fix atómico pequeño, o un puntero a un Charter que ya existe.
- Tiene impacto bajo-a-medio y el agente puede describir la remediación en una sola viñeta.

**Crea un documento TDE** cuando la deuda:

- Es *herencia de un Charter previo*. Dos formas distintas califican (ambas son TDE-worthy):
  - **Herencia estricta** — un Charter previo introdujo la deuda; los Charters subsecuentes solo la propagan sin re-introducir la decisión subyacente (p. ej., una elección legacy de schema de BD; un atajo temprano de auth; una decisión de config diferida). El Charter actual hereda la deuda por contacto transitivo.
  - **Propagación de patrón** — un Charter previo estableció un patrón que los Charters subsecuentes *re-introducen* siguiéndolo. El Charter actual no solo propaga; recrea la misma deuda replicando el patrón (p. ej., forma de handler que omite `RequireScope`; scaffolding de tests que bypasea middleware HTTP). El fix está al nivel del patrón, no de ningún Charter individual.
- *Aplica a múltiples módulos **o a fronteras de ejecución de Charter*** — fragmentarla en entradas `R<N>` por Charter pierde la forma arquitectónica. "Fronteras de ejecución de Charter" captura deuda de rastro de gobernanza que atraviesa sesiones sin atravesar módulos de código: p. ej., una clasificación diferida en CHARTER-04 que pasa en silencio por CHARTER-08 → CHARTER-13 y solo aflora bajo una gate de CI nueva.
- *Requiere un Charter dedicado fuera del envelope de scope actual* para remediarse (no el Charter actual, no el siguiente).
- *Requiere priorización o asignación humana* que el agente no puede decidir solo (matriz impact × effort, ownership, sprint placement).

Los cuatro triggers anteriores son los criterios de activación para TDE bajo §2. Cuando el AILOG que vas a escribir cargaría un `R<N>` que coincida con cualquiera de ellos, escribe el TDE en su lugar y referencialo desde la fila `§Risk` del AILOG.

---

## 4. Cuándo Solicitar Revisión Humana

Marcar `review_required: true` cuando:

1. **Baja confianza**: `confidence: low`
2. **Alto riesgo**: `risk_level: high | critical`
3. **Decisiones de seguridad**: Cualquier cambio en auth/authz
4. **Cambios irreversibles**: Migraciones, eliminaciones
5. **Impacto en usuarios**: Cambios que afectan UX
6. **Preocupaciones éticas**: Privacidad, sesgo, accesibilidad
7. **Cambios en modelos ML**: Cambios en parámetros del modelo, arquitectura o datos de entrenamiento
8. **Cambios en prompts de IA**: Modificaciones a prompts o instrucciones de agentes
9. **Dependencias críticas de seguridad**: Adición, eliminación o actualización de paquetes sensibles a la seguridad
10. **Cambios en ciclo de vida de IA**: Despliegue, retirada o cambios de versión mayor de sistemas de IA

---

## 5. Formato de Documentos

### Usar Plantillas

Antes de crear un documento, cargar la plantilla correspondiente:

```
.straymark/templates/TEMPLATE-[TIPO].md
```

### Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

### Ubicación

| Tipo | Carpeta |
|------|---------|
| AILOG | `.straymark/07-ai-audit/agent-logs/` |
| AIDEC | `.straymark/07-ai-audit/decisions/` |
| ETH | `.straymark/07-ai-audit/ethical-reviews/` |
| ADR | `.straymark/02-design/decisions/` |
| REQ | `.straymark/01-requirements/` |
| TES | `.straymark/04-testing/` |
| INC | `.straymark/05-operations/incidents/` |
| TDE | `.straymark/06-evolution/technical-debt/` |
| SEC | `.straymark/08-security/` |
| MCARD | `.straymark/09-ai-models/` |
| SBOM | `.straymark/07-ai-audit/` |
| DPIA | `.straymark/07-ai-audit/ethical-reviews/` |

### Tags y Related

Al poblar los campos `tags` y `related` en el frontmatter:

**Tags:**
- Usar palabras clave en kebab-case: `sqlite`, `api-design`, `gnome-integration`
- 3 a 8 tags por documento describiendo tema, tecnología o componente
- Los tags habilitan búsqueda y categorización en `straymark explore`

**Related:**
- Referenciar únicamente otros **documentos StrayMark** — usar el nombre de archivo con extensión `.md`
- Si el documento está en un subdirectorio dentro de `.straymark/`, incluir la ruta relativa: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-archivo.md`
- Si el documento está en el mismo directorio, el nombre de archivo es suficiente
- **No** colocar IDs de tareas (T001, US3), números de issues ni URLs externas en `related` — esos van en el cuerpo del documento

---

## 6. Comunicación con Humanos

### Ser Transparente

- Explicar el razonamiento detrás de las decisiones
- Documentar alternativas consideradas
- Admitir incertidumbre cuando existe

### Ser Conciso

- Ir al grano
- Evitar jerga innecesaria
- Usar listas y tablas cuando sea apropiado

### Ser Proactivo

- Identificar riesgos potenciales
- Sugerir mejoras cuando sean evidentes
- Alertar sobre deuda técnica
- **Surfacear disonancia entre fuentes canónicas** (Principio #8 — ver [`PRINCIPLES.md`](PRINCIPLES.md)). Cuando el agente detecte divergencia material entre dos fuentes canónicas de la documentación StrayMark, debe surfacearla antes de proceder con la tarea pedida. Ejemplos a observar durante trabajo rutinario:
  - Spec stale relativo al código shipped en cadenas multi-Charter de larga duración (ver [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 1)
  - Entradas `R<N> (new, not in Charter)` acumuladas que cumplen criterios de TDE pero no fueron escaladas (ver §3 arriba)
  - ADR vigente contradicho por la implementación actual
  - Conteo de `§Follow-ups` cruzando el umbral del backlog-pattern (ver [`FOLLOW-UPS-BACKLOG-PATTERN.md`](FOLLOW-UPS-BACKLOG-PATTERN.md))
  - Hallazgos de audit emergiendo post-close que ameriten enmienda (ver [`CHARTER-CHAIN-EVOLUTION.md`](CHARTER-CHAIN-EVOLUTION.md) Pattern 2)

  Ver [`EMERGENT-OBSERVATION-DESIGN.md`](EMERGENT-OBSERVATION-DESIGN.md) para el meta-patrón que conecta estas superficies.

---

## 7. Manejo de Errores

Si el agente comete un error:

1. **Documentar** el error en un AILOG
2. **Explicar** qué salió mal
3. **Proponer** corrección
4. **Marcar** `review_required: true`

---

## 8. Actualización de Documentos

### Crear Nuevo vs Actualizar

| Situación | Acción |
|-----------|--------|
| Corrección menor | Actualizar documento existente |
| Cambio significativo | Crear nuevo documento |
| Documento obsoleto | Marcar como `deprecated` |
| Reemplazo completo | Crear nuevo + marcar anterior como `superseded` |

### Al Actualizar

- Actualizar el campo `updated` en frontmatter
- Agregar nota en la sección de historial si existe
- Mantener consistencia con documentos relacionados

---

## 9. Observabilidad (OpenTelemetry)

Cuando se trabaja en proyectos que usan OpenTelemetry:

### Reglas

- **No** capturar PII, tokens o secretos en atributos o logs de OTel
- **Registrar** cambios en el pipeline de instrumentación (nuevos spans, atributos modificados, configuración del Collector) en AILOG con tag `observabilidad`
- **Crear** AIDEC o ADR al adoptar OTel en proyectos distribuidos — documentar la decisión de adopción y selección de backend
- **Establecer** `observability_scope` en el frontmatter cuando el cambio involucra instrumentación OTel

### Disparadores de Documentación

| Cambio | Documento | Adicional |
|--------|----------|-----------|
| Nuevos spans o atributos modificados | AILOG | Tag `observabilidad` |
| Selección de backend OTel | AIDEC o ADR | Si sistema distribuido |
| Configuración del pipeline del Collector | AILOG | Tag `observabilidad` |
| Cambios en estrategia de muestreo | AIDEC | Documentar justificación |
| Requisitos de observabilidad | REQ | Usar sección de Requisitos de Observabilidad |
| Pruebas de propagación de trazas | TES | Usar sección de Pruebas de Observabilidad |
| Incidente con evidencia de trazas | INC | Incluir trace_id/span_id en la línea temporal |
| Deuda de instrumentación | TDE | Tag `observabilidad` |

---

## 10. Diagramas de Arquitectura (Modelo C4)

Al crear documentos ADR que involucren cambios arquitectónicos:

- **Incluir** un diagrama C4 con Mermaid al nivel apropiado
- **Usar** `C4Context` para decisiones a nivel de sistema (quién usa el sistema, dependencias externas)
- **Usar** `C4Container` para decisiones a nivel de servicio/contenedor (aplicaciones, bases de datos, colas de mensajes)
- **Usar** `C4Component` para decisiones de módulos internos (componentes dentro de un servicio)
- **Ver** `00-governance/C4-DIAGRAM-GUIDE.md` para referencia de sintaxis y ejemplos

> Los diagramas son opcionales para decisiones menores. Usarlos cuando la decisión cambie fronteras del sistema, introduzca nuevos servicios o modifique comunicación entre servicios.

---

## 11. Seguimiento de Especificaciones de API

Cuando un cambio modifica endpoints de API:

- **Verificar** que la especificación OpenAPI o AsyncAPI correspondiente esté actualizada
- **Referenciar** la ruta del spec en el AILOG o ADR usando el campo `api_spec_path` (en REQ) o `api_changes` (en ADR)
- **Documentar** cambios de API que rompen compatibilidad en un ADR con `risk_level: high`

---

## 12. Checkpoint de Auditoría (workflow de Charter)

Cuando estés co-implementando un Charter, el agente **proactivamente ofrece** una auditoría externa multi-modelo en un momento específico del workflow. El checkpoint es **soft** — nunca bloquea `charter close` y nunca escala a enforcement. La auditoría externa es opt-in por diseño (costo, confianza en la disciplina primaria del operador).

### Cuándo emitir el checkpoint

Emite el checkpoint **una sola vez por Charter** cuando los **cuatro** triggers se cumplen simultáneamente:

1. El Charter está en status `in-progress` o `declared` (no `closed`).
2. Todas las tasks de la sección `## Tasks` del Charter están marcadas `[x]` completadas (o el agente acaba de completar la última).
3. `straymark charter drift <CHARTER-ID>` retorna exit 0 (sin drift no contabilizado).
4. El developer **no** ha invocado `straymark charter close <CHARTER-ID>` aún, ni ha mencionado intención de cerrar.

Si el developer rechazó la auditoría en un turno previo para el mismo Charter, **no re-emitir** en turnos subsiguientes de la misma conversación.

### Forma del mensaje del checkpoint

Renderiza el mensaje así (sustituye `<CHARTER-ID>` y la justificación de la recomendación):

```
Llegamos al checkpoint del Charter <CHARTER-ID>. Está implementado,
drift check OK, pendiente solo `straymark charter close`.

En este punto puedes correr una auditoría externa (típicamente 2 LLMs
de familias distintas + un calibrador) que arroje findings cross-modelo
sobre la implementación.

Mi recomendación: [SÍ / NO], porque:
  - <razón concreta basada en el Charter, AILOGs o diff>

Si decides auditar:
  Ejecuta /straymark-audit-prompt <CHARTER-ID> y yo escribo el prompt
  unificado de auditoría en .straymark/audits/<CHARTER-ID>/audit-prompt.md.
  Después abre una o más CLIs auditoras (gemini-cli, claude-cli,
  copilot-cli, codex-cli) en este repo e invoca
  /straymark-audit-execute <CHARTER-ID> en cada una — recomendación: al
  menos 2 auditores de familias de modelo distintas. Cuando y solo
  cuando TODAS las auditorías que encargaste hayan terminado, regresa
  aquí y ejecuta /straymark-audit-review <CHARTER-ID>. Yo consolido los
  N reports en un documento review.md con veredictos, plan de
  remediación y calificación de auditores, y mergeo el bloque YAML en
  la telemetría del Charter.

Si decides no auditar:
  Continúa con `straymark charter close <CHARTER-ID>` cuando estés listo.
  La auditoría externa es completamente opcional — la disciplina del
  Charter declarativo + drift check + AILOG da suficiente rigor para
  un cierre confiable sin auditoría.
```

### Heurísticas para la recomendación SÍ/NO

Son heurísticas, no reglas rígidas — estás cerca del contexto, afínalas con el adoptante.

**Recomienda SÍ cuando** (cualquiera basta):

- El Charter tocó superficie crítica de seguridad (auth, RLS, manejo de secrets, IAM).
- El Charter introdujo un componente nuevo (no refactor) que el developer no había co-implementado antes.
- Algún AILOG asociado documenta un `R<N>` con `confidence: low | medium` y `risk_level: medium` o mayor.
- El developer marcó el Charter como `effort_estimate: L` y este es el primer Charter del adoptante.
- El developer **explícitamente** pidió validación cross-modelo en el trigger del Charter.
- **Señal estructural de complejidad** *(disponible solo cuando el CLI fue compilado con el feature `analyze` activo, true en los binarios oficiales)*: el diff del `range` introduce o modifica al menos una función cuya complejidad cognitiva supera **2× el threshold configurado** en `.straymark/config.yml` (`complexity.threshold`, default `8` → ≥ `17`). Una función densa nueva es exactamente el caso donde dos auditores cross-familia capturan implementation gaps que un solo modelo deja pasar. **Graceful-degradation:** si el binario no tiene el feature `analyze`, omite silenciosamente esta señal — no avises, no menciones la ausencia.

**Recomienda NO cuando** (todas juntas):

- El Charter es refactor o cambio de documentación (sin comportamiento nuevo).
- `effort_estimate` es `XS` o `S`.
- Los AILOGs asociados tienen todos `confidence: high` y sin riesgos `R<N+1>` emergentes.
- `risk_level` del Charter es `low` (o no está marcado).

**Caso default (ninguna señal clara):** recomienda **NO** con framing neutro ("no veo señal específica que justifique el costo de dos modelos adicionales; cierra cuando estés listo"). El costo de la auditoría externa es real — no infles adopción recomendando SÍ por inercia.

### Reglas de engagement

- El checkpoint **nunca** se repite dentro del mismo Charter una vez que el developer responde.
- El checkpoint **no** bloquea ninguna acción posterior. Si el developer lo ignora y corre `charter close`, close procede normalmente — no hay enforcement y no lo habrá (decisión de diseño v0+v1 permanente; ver `Propuesta/straymark-audit-skills.md` §2.2).
- El checkpoint **no** se cuenta en ninguna métrica de calidad. No hay KPI "% Charters auditados" en `straymark metrics` — por diseño, para evitar incentivos a inflar el conteo.
- Si el developer acepta la auditoría, el workflow procede vía tres skills en secuencia: `/straymark-audit-prompt` (escribe el prompt unificado en el path canónico) → `/straymark-audit-execute` × N (una por CLI auditora que abra el operador — estas corren en esas CLIs, no en el agente principal) → `/straymark-audit-review` (consolida N reports inline en `.straymark/audits/<id>/review.md` y mergea el YAML en la telemetría). Los operadores nunca copian/pegan prompts ni reports — el intercambio de archivos sucede vía paths canónicos bajo `.straymark/audits/`.

---

## Patrones

Cuando un proyecto acumula un volumen alto de AILOGs a lo largo de múltiples Charters y los follow-ups se vuelven difíciles de rastrear, ver [FOLLOW-UPS-BACKLOG-PATTERN.md](FOLLOW-UPS-BACKLOG-PATTERN.md) para una convención reproducible (registro central + script de detección de drift + integración con agentes). Los adopters con ~20+ AILOGs se benefician; por debajo de ese umbral la convención per-AILOG `§Follow-ups` por sí sola es suficiente.

---

*StrayMark fw-4.17.0 | [Strange Days Tech](https://strangedays.tech)*
