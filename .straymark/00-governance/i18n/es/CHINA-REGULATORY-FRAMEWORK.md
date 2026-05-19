# Marco Regulatorio Chino — StrayMark

> Visión general de las seis regulaciones de IA / datos de China cubiertas cuando `regional_scope: china` está habilitado en `.straymark/config.yml`.

**Languages**: [English](../../CHINA-REGULATORY-FRAMEWORK.md) | Español | [简体中文](../zh-CN/CHINA-REGULATORY-FRAMEWORK.md)

---

## Activación

Las verificaciones regulatorias chinas son **opt-in**. Habilítelas en `.straymark/config.yml`:

```yaml
regional_scope:
  - global   # NIST + ISO 42001 (siempre disponibles)
  - eu       # EU AI Act + GDPR
  - china    # añade los 6 frameworks siguientes
```

Cuando `china` está en alcance:
- `straymark new` expone los 4 tipos de documento específicos de China (PIPIA, CACFILE, TC260RA, AILABEL).
- `straymark compliance --all` incluye los 6 checkers chinos.
- `straymark validate` aplica CROSS-004…CROSS-011 y TYPE-003…TYPE-006.

Un proyecto sin `china` en el alcance no se ve afectado.

---

## Matriz de Cobertura

| # | Regulación | Tipo | Estado | Evidencia StrayMark |
|---|-----------|------|--------|-------------------|
| 1 | **TC260 AI Safety Governance Framework v2.0** (15-sep-2025) | Recomendado (en redacción como GB) | Activa | TC260RA; campos `tc260_*` |
| 2 | **PIPL** + **PIPIA** (Art. 55-56) | Obligatorio | Vigente desde 2021-11-01 | PIPIA; campos `pipl_*`; retención ≥ 3 años |
| 3 | **GB 45438-2025** Etiquetado de contenido IA | **Obligatorio** | Vigente desde 2025-09-01 | AILABEL; campos `gb45438_*` en MCARD |
| 4 | **Registro de Algoritmos CAC** | Obligatorio para servicios en alcance | Vigente | CACFILE; `cac_filing_*` en MCARD |
| 5 | **GB/T 45652-2025** Seguridad de datos de entrenamiento | Recomendado | Vigente desde 2025-11-01 | `gb45652_training_data_compliance` en SBOM/MCARD |
| 6 | **CSL 2026** Reporte de incidentes | Obligatorio | Vigente desde 2026-01-01 | Sección "CSL 2026" en INC; `csl_severity_level`, `csl_report_deadline_hours` |

---

## Mapeo Tipo de Documento → Framework

| Framework | Plantilla principal | Cross-references |
|-----------|---------------------|------------------|
| TC260 v2.0 | TC260RA | ETH, MCARD |
| PIPL / PIPIA | PIPIA | DPIA |
| GB 45438 | AILABEL | MCARD (modelos generativos) |
| CAC | CACFILE | MCARD, SBOM |
| GB/T 45652 | (Secciones en SBOM y MCARD) | TC260RA |
| CSL 2026 | INC (extendido) | (ninguno) |

---

## Guías de Implementación

| Framework | Guía |
|-----------|------|
| Risk grading TC260 v2.0 | [TC260-IMPLEMENTATION-GUIDE.md](TC260-IMPLEMENTATION-GUIDE.md) |
| PIPL Art. 55 → PIPIA | [PIPL-PIPIA-GUIDE.md](PIPL-PIPIA-GUIDE.md) |
| Proceso de doble registro | [CAC-FILING-GUIDE.md](CAC-FILING-GUIDE.md) |
| Etiquetado explícito + implícito | [GB-45438-LABELING-GUIDE.md](GB-45438-LABELING-GUIDE.md) |

---

## Verificaciones de Compliance

| `--standard` | IDs | Valida |
|--------------|-----|--------|
| `china-tc260` | TC260-001/002/003 | Existe ≥1 TC260RA; niveles altos exigen review; tres criterios completos |
| `china-pipl` | PIPL-001…004 | PIPIA presente cuando `pipl_applicable`; cross-border documentado; retención ≥ 3 años |
| `china-gb45438` | GB45438-001/002/003 | AILABEL presente; estrategia explícita + implícita; metadatos completos |
| `china-cac` | CAC-001/002/003 | CACFILE presente; estado no estancado en `pending` > 90 días; filing_number cuando `*_approved` |
| `china-gb45652` | GB45652-001/002 | SBOM declara compliance; MCARD describe controles |
| `china-csl` | CSL-001/002/003 | INC con `csl_severity_level`; horas coherentes con severidad; post-mortem 30d |

`straymark compliance --region china` ejecuta los 6 a la vez.

---

## Fuentes

Para fuentes detalladas, consultar la versión inglesa.

<!-- StrayMark | https://strangedays.tech -->
