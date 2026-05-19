# Guía de Registro CAC — StrayMark

> Guía práctica para completar un documento CACFILE y navegar el proceso de doble registro administrado por la Cyberspace Administration of China (CAC).

**Languages**: [English](../../CAC-FILING-GUIDE.md) | Español | [简体中文](../zh-CN/CAC-FILING-GUIDE.md)

---

## Cuándo se Requiere Registro CAC

El registro es obligatorio para servicios con **atributos de opinión pública** o **capacidad de movilización social** bajo:

- *Provisions on the Administration of Algorithmic Recommendations of Internet Information Services* (en vigor desde 2022-03-01)
- *Provisions on the Administration of Deep Synthesis* (en vigor desde 2023-01-10)
- *Interim Measures for the Management of Generative AI Services* (en vigor desde 2023-08-15)

Indicadores de aplicabilidad:

- El servicio genera, sintetiza o disemina contenido visible al público.
- El servicio usa ranking, recomendación, búsqueda o selección de contenido que influye en la opinión.
- El servicio opera en noticias, redes sociales, audio/vídeo, streaming en vivo, o IA generativa.
- El servicio se proporciona a usuarios en China continental.

Si aplica, setee `cac_filing_required: true` en el MCARD y cree un CACFILE. La regla **CROSS-007** aplica el enlace.

---

## Registro Único vs. Doble

Coexisten dos tracks regulatorios:

| Track | Autoridad | Qué | Cuándo |
|-------|-----------|-----|--------|
| **Algorithm Filing** | CAC nacional (tras submission online) | Divulgación de mecanismo, audiencia objetivo, reporte de responsabilidad de seguridad | Todos los algoritmos en alcance |
| **Generative AI Large Model Filing** | CAC provincial → CAC nacional | Pruebas técnicas en dos etapas + divulgación algorítmica | Servicios IA generativos con atributos de opinión pública / movilización |

Servicios IA generativos típicamente requieren **doble registro**: evaluación de seguridad provincial primero, luego algorithm filing nacional. Setee `cac_filing_type: dual`.

---

## Documentos e Información Requeridos

La plantilla CACFILE captura los inputs canónicos. Los artefactos más comúnmente solicitados:

| Artefacto | Dónde en CACFILE | Notas |
|-----------|------------------|-------|
| Resumen del servicio | § Service Overview | Nombre, proveedor, dominio, audiencia |
| Reporte de auto-evaluación | § Self-Assessment | Riesgos + mitigaciones en compliance, privacidad, sesgo, IP, menores |
| Política interna de gestión | § Internal Algorithm Management Policy | Responsable designado, manejo de quejas, logging |
| Reporte de Responsabilidad de Seguridad | § Algorithm Security Responsibility Report | Per Art. 24 |
| Descripción de datos de entrenamiento | § Training Data Description | Fuentes, volumen, legalidad, categorías sensibles filtradas |
| Lista de palabras clave bloqueadas | § Blocked Keywords Strategy | Prevenir contenido subversivo, separatista, etc. |
| Conjunto de preguntas de prueba | § Testing Question Set | Prompts de red-team + umbrales |
| Acuerdos de servicio + canales de queja | (texto libre) | Términos al usuario y SLA |

---

## Ciclo de Vida del Estado

| Estado | Significado |
|--------|-------------|
| `pending` | CACFILE redactado pero no presentado |
| `provincial_submitted` | Presentado a CAC provincial; esperando decisión |
| `provincial_approved` | Decisión provincial favorable; puede continuar a nacional (doble registro) |
| `national_submitted` | Presentado a CAC nacional |
| `national_approved` | Filing number emitido (`cac_filing_number` poblado) |
| `rejected` | Registro rechazado; documentar razones en *Filing Trail* |
| `not_required` | Evaluación inicial concluyó que el servicio está fuera de alcance |

La verificación de compliance **CAC-002** marca filings que llevan más de 90 días en `pending` como aprobación parcial. **CROSS-006** requiere que `cac_filing_number` esté poblado cuando el estado es cualquier `*_approved`.

---

## Divulgación Pública del Filing Number

Una vez emitido `cac_filing_number`, el proveedor debe mostrarlo a usuarios finales. Patrones comunes:
- Footer de la home: "**算法备案号: 网信算备XXXXXXXXXXXX**".
- Página *Settings / About* dentro de la app.
- Términos de servicio.

Documente la superficie de divulgación en la última fila de *Filing Trail*.

---

## Categorías de Riesgo en la Auto-evaluación

La auto-evaluación debe abordar al menos:

- **Compliance de contenido**: prevención de contenido ilegal (subversión, separatismo, terrorismo, obscenidad, violencia, odio étnico).
- **Protección de información personal**: alineamiento con PIPL (cross-link a PIPIA).
- **Sesgo / trato diferenciado injusto**: discriminación de precios, manipulación de ranking, sesgo demográfico.
- **Propiedad intelectual**: licencias de datos de entrenamiento, similitud de outputs con obras protegidas.
- **Protección de menores**: manejo especial para menores de 18 — filtros, límites de tiempo, anti-adicción.

Para servicios que usan modelos fundacionales de terceros, la auto-evaluación debe incluir una evaluación del estado de filing del proveedor upstream.

---

## Vinculación con Otros Documentos StrayMark

Un set completo de filing típicamente referencia:

```yaml
# Frontmatter de CACFILE-2026-04-25-001
related:
  - MCARD-2026-04-25-001    # el modelo registrado
  - TC260RA-2026-04-25-001  # risk assessment TC260
  - PIPIA-2026-04-25-001    # si se procesa información personal
  - AILABEL-2026-04-25-001  # plan GB 45438
  - SBOM-2026-04-25-001     # inventario de datos (compliance GB/T 45652)
```

`straymark compliance --standard china-cac` recorre este vinculado para verificar.

<!-- StrayMark | https://strangedays.tech -->
