# Guía PIPL / PIPIA — StrayMark

> Guía práctica para completar una Evaluación de Impacto de Protección de Información Personal bajo la **Personal Information Protection Law** (PIPL) de China, Artículos 55-56.

**Languages**: [English](../../PIPL-PIPIA-GUIDE.md) | Español | [简体中文](../zh-CN/PIPL-PIPIA-GUIDE.md)

---

## Cuándo Se Requiere un PIPIA

Por **PIPL Art. 55**, debe realizarse un PIPIA antes de:

1. Procesar **información personal sensible** (Art. 28: biometría, creencias religiosas, identidad específica, salud, cuentas financieras, geolocalización, datos de menores de 14 años).
2. Usar información personal para **toma de decisiones automatizada**.
3. **Encargar** procesamiento a terceros / proveer datos a otros responsables / **divulgar públicamente** información personal.
4. **Transferir información personal** fuera de China continental.
5. Otras actividades con **impacto significativo** sobre los individuos.

Si aplica cualquiera, setee `pipl_applicable: true` en la ETH / MCARD / AILOG correspondiente y cree un PIPIA. La regla **CROSS-005** aplica el enlace.

---

## PIPIA vs. DPIA

PIPIA (China) y DPIA (GDPR Art. 35) se solapan conceptualmente pero difieren en detalles:

| Aspecto | DPIA (GDPR) | PIPIA (PIPL) |
|---------|------------|--------------|
| Base estatutaria | GDPR Art. 35 | PIPL Art. 55-56 |
| Umbral de disparo | "Riesgo alto" | Cualquiera de los 5 escenarios del Art. 55 |
| Elementos requeridos | Necesidad, riesgos a titulares, mitigaciones, consulta DPO | Legalidad/legitimidad/necesidad, impacto a derechos personales, riesgos de seguridad, proporcionalidad |
| Retención | No especificada por GDPR | **Mínimo 3 años** (Art. 56) |
| Consulta a autoridad | Obligatoria si riesgo residual alto | CAC provincial consultada para transferencia transfronteriza |

Si su organización está sujeta a **ambos** GDPR y PIPL para el mismo procesamiento, lo más simple es:
- Mantener un **DPIA** como documento principal.
- Mantener un **PIPIA** que cross-referencie al DPIA vía `related: [DPIA-...]` y añada los elementos específicos PIPL.

La plantilla DPIA ahora incluye una sección *Cross-reference: PIPIA* que apunta al revés cuando `pipl_applicable: true`.

---

## Tres Elementos Obligatorios del Reporte PIPIA (Art. 56)

Un PIPIA debe abordar:

### 1. Legalidad / Legitimidad / Necesidad

- **Legal** — Identificar la base legal bajo PIPL Art. 13.
- **Legítimo** — El propósito es claro, razonable y declarado.
- **Necesario** — Se procesan los datos mínimos para el propósito.

### 2. Impacto a Derechos + Riesgos de Seguridad

- Mapear cada derecho PIPL (Arts. 44-47, más Art. 24 opt-out de decisión automatizada) a una fila de probabilidad/severidad/mitigación.
- Identificar riesgos de confidencialidad, integridad y disponibilidad. Use la misma escala (low/medium/high) que el campo `risk_level` de StrayMark.

### 3. Proporcionalidad de las Medidas Protectoras

- Demostrar que las medidas son **legales, efectivas y proporcionales**.
- Documentar el riesgo residual tras cada medida.

---

## Transferencia Transfronteriza (PIPL Art. 38-40)

La transferencia transfronteriza requiere **uno** de estos mecanismos:

| Mecanismo | Cuándo |
|-----------|--------|
| **Evaluación de Seguridad CAC** | Operadores de Infraestructura Crítica; procesadores que manejan información personal de ≥ 1M individuos; transferencia transfronteriza acumulada de ≥ 100.000 individuos o de información sensible de ≥ 10.000 individuos en cualquier período de 12 meses |
| **Certificación de Protección de Información Personal** | Alternativa voluntaria; otorgada por organismo acreditado |
| **Contrato Estándar** | Archivado con CAC provincial; adecuado para volúmenes menores |

StrayMark no modela actualmente estos como campos estructurados (más allá de `pipl_cross_border_transfer: true`) — documente el mecanismo elegido en la sección *Cross-Border Transfer Analysis* de la plantilla PIPIA.

---

## Retención

Por Art. 56, el PIPIA y los "registros de procesamiento" deben **conservarse al menos tres años**. Setee `pipl_retention_until: YYYY-MM-DD` a una fecha al menos 3 años después de `created`. **TYPE-003** lo aplica.

```yaml
created: 2026-04-25
pipl_retention_until: 2029-04-25  # 3 años exactos — mínimo permitido
```

---

## Activadores de Información Personal Sensible

Especial cuidado con estas categorías (Art. 28):

- Identificación biométrica (huella, rostro, voz, marcha)
- Creencias religiosas
- Identidad específica
- Salud médica
- Cuentas financieras
- Seguimiento de ubicación
- Información personal de menores de 14 años

El procesamiento requiere **consentimiento separado** y un PIPIA. Si su modelo usa cualquiera en entrenamiento o inferencia:
- Setee `pipl_sensitive_data: true`
- Documente la mitigación en *Protective Measures* del PIPIA
- Cross-referencie la sección `## Training Data` del MCARD relevante

---

## Oficial de Protección de Información Personal (PIPL Art. 52)

Un responsable que procese información personal de **más de 1.000.000 de individuos** debe designar un Oficial de Protección de Información Personal. Si aplica:
- Setee `dpo_consulted: true` en el PIPIA tras la consulta
- Documente la opinión del PIPO en *Consultation*

---

## Ejemplo: Chatbot de Atención al Cliente

Un chatbot SaaS desplegado en China continental que:
- Almacena historial de conversación (info personal)
- Usa un LLM para respuestas automatizadas (decisión automatizada)
- Envía algunas consultas a un endpoint de inferencia en el extranjero (cross-border)

**Activadores**: 1, 2, 4 → PIPIA requerido.

**Frontmatter**:
```yaml
id: PIPIA-2026-04-25-001
pipl_applicable: true
pipl_article_55_trigger: cross_border
pipl_sensitive_data: false
pipl_cross_border_transfer: true
pipl_retention_until: 2029-04-25
related:
  - MCARD-2026-04-25-001
  - TC260RA-2026-04-25-001
  - DPIA-2026-04-25-001
```

**Secciones a enfatizar**: análisis cross-border, decisión automatizada (Art. 24 opt-out), base legal (típicamente consentimiento), salvaguardas del receptor.

<!-- StrayMark | https://strangedays.tech -->
