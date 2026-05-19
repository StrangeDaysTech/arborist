---
charter_id: CHARTER-NN
status: declared
effort_estimate: M
trigger: "[1 línea: qué señal concreta — evento observable, decisión declarada, umbral métrico, o hito de infraestructura — justifica ejecutar este Charter ahora]"
# Establece exactamente uno de los siguientes dos cuando el Charter tenga un origen conocido.
# Ambos ausentes es válido para un Charter creado sin origen explícito (debe llenarse antes
# de que el status pase a in-progress).
# originating_ailogs: [AILOG-YYYY-MM-DD-NNN]
# originating_spec: specs/001-feature/spec.md
---

# Charter: [TÍTULO BREVE]

> **Status (espejado del frontmatter — la fuente de verdad está arriba):** declared. Esfuerzo: [XS | S | M | L] (~[N] min).
>
> **Origen:** [resumen humano; la forma machine-readable es `originating_ailogs` u `originating_spec` en el frontmatter].

<!-- Charter template — 6 convenciones de formato destiladas del experimento
     Sentinel /plan-audit (6 ciclos, 2026-04-28). Ver el bloque de comentario al final
     de este archivo para cada convención con su justificación empírica, y
     straymark-cli-roadmap.md §3 + straymark-thesis-validation.md §3-§5 para la
     evidencia de origen. -->

## Context

[1-2 párrafos. Qué problema resuelve este Charter, qué motivación operacional o
regulatoria lo hace urgente, qué se ha intentado antes (si algo). Cita los AILOGs
origen aquí también si ayuda al lector a entender por qué la deuda quedó abierta.]

## Scope

**In scope:**

[Lista numerada de los cambios concretos a aplicar. Cada item debe ser verificable:
"X archivo gana Y método", "Z test cubre W caso". Evitar items vagos como "mejorar
performance" — esos son objetivos, no scope.]

1. [Item 1]
2. [Item 2]
3. [...]

**Out of scope:**

[Lista de cosas explícitamente NO cubiertas por este Charter. Importante para que
auditores no las clasifiquen como gaps. Idealmente cita el Charter o iniciativa
donde sí van.]

- [Item 1] — diferido a [Charter/iniciativa].
- [Item 2] — fuera del alcance porque [razón].

## Archivos a modificar

| Archivo | Cambio |
|---|---|
| `path/al/archivo.ext` | [Descripción concreta del cambio] |
| `...` | `...` |
| `.straymark/07-ai-audit/agent-logs/AILOG-...md` | Nuevo, `risk_level: [low|medium|high]` |

## Verification

### Local checks

Comandos ejecutables literal en clean shell — incluyen setup explícito de dependencias.
Cualquier fallo de estos comandos indica deuda real.

```bash
# Build & test (adapta a tu stack)
<comando-build>
<comando-test>

# Setup explícito de scanners de seguridad/vulnerabilidades
# (Patrón validado en Sentinel PLAN-01..05: lookups implícitos en PATH generaban
# clasificaciones falso-positivas como 'real_debt' por auditores externos.)
<install-and-run-security-scanner>
<install-and-run-vulnerability-scanner>

# Otros comandos locales aquí. Si requieren infra de integración, indicar:
<comando-integration-test>
```

### Production smoke (after deploy)

Comandos que **solo aplican después de deploy a un ambiente real**. NO ejecutables
en clean shell sin infraestructura. Auditores externos deben saltar esta sección —
fallos aquí no son `real_debt`.

```bash
# Ejemplo: verificar que un endpoint nuevo está vivo en producción.
TOKEN="$(<auth-cli> print-identity-token)"
curl -X PUT "https://${SERVICE_HOST}/api/v1/.../..." \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"...": "..."}'

# Ejemplo: query SQL en DB productiva para verificar evento persistido.
<production-db-cli> connect <service-db> -- \
  -c "SELECT context FROM audit_records WHERE action='...' \
      ORDER BY timestamp DESC LIMIT 1"
```

## Riesgos

[Lista de riesgos R1, R2, ... que la implementación se compromete a mitigar. Cada uno
con su mitigación documentada. Convención: si durante la ejecución emerge un riesgo
NUEVO no anticipado, documentarlo en el AILOG bajo `## Risk` como
`R<N+1> (nuevo, no en Charter)` — Gemini y otros auditores externos validan estos
cross-document.

Cada mitigación debe especificar: (a) trigger o threshold concreto (no "eventualmente"),
(b) acción comprometida, (c) qué pasa si la mitigación misma falla, (d) dónde se
captura el follow-up si el riesgo destapa lecciones para un ciclo posterior.]

- **R1 — [descripción del riesgo]**: [probabilidad/severidad].
  Mitigación: [acción concreta tomada en la implementación].
- **R2 — ...**: ...
- [...]

## Tasks

1. Sync main, branch `<prefijo>/[slug]`.
2. [Tarea de implementación 1].
3. [Tarea de implementación 2].
4. [...]
5. AILOG (`risk_level: [low|medium|high]`, `review_required: [true|false]`).
6. **Para ejecución multi-batch (3+ lotes o >1 día)**: mantener una
   `## Batch Ledger` en el AILOG. Después de cada commit de lote, correr
   `straymark charter batch-complete <CHARTER-ID> <N>` para actualizar
   la bitácora antes de pushear. El drift gate al cierre rechazará
   cualquier `### Batch N` que quede como `(pending)`. Saltar este paso
   en Charters de un solo lote — `## Acciones Realizadas` en el AILOG basta.
7. Verification local pasa limpio.
8. **Auto-checklist drift** (cuando entregue Fase 2 del CLI roadmap):
   `straymark charter drift CHARTER-NN <range>` para detectar drifts entre lo declarado
   y lo modificado **antes** del commit. Si reporta omisiones, completar el trabajo
   o documentar en AILOG bajo `## Risk` como `R<N+1> (nuevo, no en Charter)`. Si
   reporta scope expansion, documentar en AILOG el motivo (mock updates, generated
   files, drift fix pre-existente, etc.). Hasta que Fase 2 entregue, correr el
   `check-plan-drift.sh` de Sentinel manualmente para el mismo efecto.
9. Commit + push + abrir PR.

## Cierre del Charter

Al cerrar este Charter:

1. **Atomic update (format v4)**: si el drift check (Tasks #7) reportó cualquier drift
   no capturado ya en el AILOG, editar `## Archivos a modificar` y/o añadir un bloque
   `## Closing notes` en **este mismo commit/PR**, antes de submitir. No diferir a un
   housekeeping PR post-merge. El patrón atomic-update es la forma canónica de mantener
   el Charter coherente con la ejecución; diferirlo deja el Charter stale y confunde a
   lectores futuros (PLAN-07 de Sentinel demostró el failure mode que este step previene).

2. **Post-merge drift check** (automatizado cuando entregue Fase 2 + revisión manual):
   - Correr `straymark charter drift CHARTER-NN origin/main..HEAD` (Fase 2) o el
     script de Sentinel equivalente, y validar que el output esté limpio o que
     todos los drifts estén documentados en el AILOG.
   - Esto atrapa el caso raro donde drift se introduce post-merge (squash mangling,
     amendments admin, etc.) y el step atomic en #1 no pudo aplicar.

3. **Mover la fila** en `.straymark/charters/README.md` a `## Cerrados` y referenciar el PR.

4. **Status del frontmatter** pasa de `in-progress` a `closed` (y opcionalmente
   se añade `closed_at: YYYY-MM-DD` — el schema permite campos adicionales arbitrarios).

5. **No borrar** este archivo — el historial de planning importa tanto como el AILOG
   de ejecución.

## Closing notes

> Añadir esta sección SOLO cuando el drift check (Tasks #7) reportó drift que el
> implementor eligió remediar atómicamente (en lugar de rehacer la implementación
> para coincidir con `## Archivos a modificar` exactamente). Cada bullet: qué cambió
> respecto a la declaración, por qué, referencia al AILOG que documentó la decisión.
> Omitir la sección entera si no hubo drift — un `## Closing notes` vacío es ruido.
>
> Ejemplos históricos en Sentinel: PLAN-05 (`docs/plans/05-per-service-anomaly-thresholds.md`)
> §Notas de cierre — archivos removidos porque la implementación eligió otro punto
> de inyección; PLAN-07 (`docs/plans/07-fix-distribution-aligner.md`) §Notas de
> cierre — archivo removido porque el live test resultó agnóstico al cambio. Ambos
> demuestran el patrón en uso productivo.

- `[path/archivo-de-declaración.ext]` [removido | reubicado a X | repurposed]:
  [1-2 líneas explicando qué hizo la implementación en lugar de eso y por qué la
  declaración original ya no es precisa]. Referencia: AILOG-YYYY-MM-DD-NNN §[sección].

---

<!--
Convenciones de formato — 6 patrones embebidos en este template, destilados del
experimento Sentinel /plan-audit de 6 ciclos (2026-04-28). La provenance forma parte
del registro histórico (en términos de StrayMark estas son simplemente "las
convenciones", no "v2 + adición v3" — la partición era el log de iteración de
Sentinel, no estructural).

1. Verification se divide en `### Local checks` (ejecutables literal en clean shell)
   y `### Production smoke (after deploy)` (no ejecutables sin infraestructura).
   Razón: los auditores externos clasificaban como `real_debt` los fallos de comandos
   prod-only — ruido evitable. Validado 5/5 ciclos tras nombrar la convención.

2. Esfuerzo se mide en TIEMPO (XS/S/M/L), no en `~N líneas`. Razón: el tiempo cumplió
   la estimación (1.0x) en 4/5 ciclos; las líneas se inflaron 1.0x → 3.1x → 8.1x por
   AILOG/tests/mocks. Las líneas no son predictivas del esfuerzo cognitivo.

3. Modificadores como `(opcional)` o `(después de deploy)` viven como sub-secciones
   estructuradas, nunca como comentarios in-line entre paréntesis. Razón: el auditor
   Gemini ignoró consistentemente los modificadores parentizados y clasificó comandos
   marcados como opcionales como `real_debt`. Validado 2/2 ciclos donde aplicaba.

4. Riesgos R<N> se enumeran en el Charter; los nuevos riesgos que emergen durante
   ejecución se documentan en el AILOG como `R<N+1> (nuevo, no en Charter)`. Razón:
   señal cross-validable por auditores externos — triangulan declaraciones del
   Charter contra emergencia en el AILOG. Validado 4/4 ciclos donde emergieron
   riesgos nuevos.

5. La sección `## Cierre del Charter` requiere que el implementor actualice el
   Charter doc atómicamente (mismo PR del fix) cuando Tasks #7 detecta drift, no en
   un housekeeping PR separado post-merge. El bloque `## Closing notes` es el lugar
   canónico para documentar cada edición atomic (qué cambió respecto a `## Archivos
   a modificar`, por qué, referencia al AILOG). Razón: PLAN-07 de Sentinel demostró
   que sin un step atomic-update explícito, la remediación de drift puede demorar
   días respecto al PR principal, dejando el Charter stale y confundiendo a lectores
   futuros — AIDEC-2026-05-02-001 de Sentinel formalizó el gap y propuso format v4
   (este template lo encarna).

6. Auto-checklist drift (`straymark charter drift`, Fase 2 del CLI roadmap; Sentinel
   tenía `scripts/check-plan-drift.sh`) corre en pre-commit (Tasks #7) y al cierre
   del Charter. Detecta drifts de OMISIÓN (archivo declarado, no tocado) y de SCOPE
   EXPANSION (archivo tocado, no declarado). Razón: los auditores externos capturaron
   drifts de implementation-gap y hallucination que el implementador no documentó
   en su AILOG. El script atrapa los mismos drifts ANTES del commit, separando
   "conocidos y documentados" de "olvidados". Cero falsos positivos en 2/2 tests
   empíricos contra los Plans canónicos de Sentinel.
-->
