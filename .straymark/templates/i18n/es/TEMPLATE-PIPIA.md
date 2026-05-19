---
id: PIPIA-YYYY-MM-DD-NNN
title: "[Sistema/Proceso] Evaluación de Impacto de Protección de Información Personal"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: low  # PIPIA requiere amplio juicio humano
review_required: true  # Obligatorio bajo PIPL Art. 55

# --- Workflow de aprobación (opcional, llenar al momento de la revisión) ---
# reviewed_by: <id-revisor>            # email | usuario-github | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
pipl_applicable: true
pipl_article_55_trigger: sensitive_data  # sensitive_data | automated_decision | third_party_disclosure | cross_border | public_disclosure | other
pipl_sensitive_data: true
pipl_cross_border_transfer: false
pipl_retention_until: YYYY-MM-DD  # Mínimo 3 años desde `created` por PIPL
iso_42001_clause: [6, 8]
dpo_consulted: false
tags: [privacy, pipl, china, pipia]
related: []
---

# PIPIA: [Sistema/Proceso] Evaluación de Impacto de Protección de Información Personal

> **IMPORTANTE**: Este documento es un BORRADOR creado por un agente de IA. Requiere revisión y aprobación humana antes de iniciar cualquier procesamiento de información personal.
> Por PIPL Art. 56, este informe y los registros de procesamiento relacionados deben conservarse **al menos 3 años**.

## Activador del Artículo 55

Marque los activadores que motivaron este PIPIA según PIPL Art. 55:

- [ ] Procesamiento de **información personal sensible** (Art. 28: biometría, creencias religiosas, identidad específica, salud, cuentas financieras, geolocalización, datos de menores de 14 años)
- [ ] Uso de información personal para **toma de decisiones automatizada**
- [ ] **Encargo** de procesamiento a terceros / provisión a otros responsables / divulgación pública
- [ ] **Transferencia transfronteriza** de información personal
- [ ] Otras actividades de procesamiento con **impacto significativo** sobre los individuos

## Descripción del Procesamiento

- **Naturaleza**: [Cómo se recopilan, almacenan, usan y eliminan los datos]
- **Alcance**: [Número de titulares, volumen, alcance geográfico dentro/fuera de China continental]
- **Contexto**: [Relación con los titulares, expectativas razonables]
- **Propósito**: [Propósito específico declarado]
- **Base legal (PIPL Art. 13)**: [consentimiento / ejecución contractual / deber legal / emergencia / informes periodísticos / interés público / información publicada / otra]
- **Categorías de titulares**: [Empleados, clientes, menores, pacientes, etc.]
- **Categorías de datos personales**: [Identificadores, contacto, salud, biometría, financieros, ubicación, etc.]
- **Información personal sensible involucrada**: [Sí/No — listar categorías Art. 28]
- **Destinatarios / encargados**: [Quién recibe los datos, salvaguardas contractuales]
- **Período de retención**: [Duración, criterios de eliminación, base legal]

## Necesidad y Proporcionalidad (PIPL Art. 56 §1)

- **Legalidad**: [Por qué el procesamiento es legal bajo PIPL Art. 13]
- **Legitimidad**: [Propósito claro y razonable]
- **Necesidad**: [Procesamiento mínimo necesario para el propósito]
- **Limitación de propósito**: [Cómo se limita al propósito declarado]
- **Minimización de datos**: [Cómo se recopila solo el mínimo necesario]

## Impacto sobre Derechos Personales (PIPL Art. 56 §2)

| Derecho impactado | Probabilidad | Severidad | Nivel de riesgo | Mitigación |
|-------------------|-------------|-----------|----------------|-----------|
| Derecho a saber / decidir (Art. 44) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |
| Derecho a acceder / copiar (Art. 45) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |
| Derecho a corregir / complementar (Art. 46) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |
| Derecho a eliminar (Art. 47) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |
| Derecho a portabilidad (Art. 45) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |
| Derecho a oponerse a decisión automatizada (Art. 24) | [B/M/A] | [B/M/A] | [B/M/A] | [Medidas] |

## Riesgos de Seguridad (PIPL Art. 56 §2)

| Riesgo | Probabilidad | Severidad | Nivel | Origen | Naturaleza del Impacto |
|--------|-------------|-----------|-------|--------|------------------------|
| [Riesgo 1] | [B/M/A] | [B/M/A] | [B/M/A] | [Origen] | [físico/material/reputacional] |

## Medidas Protectoras (PIPL Art. 56 §3)

Demuestre que las medidas son **legales, efectivas y proporcionales** al nivel de riesgo.

| Riesgo | Medida | Tipo | Riesgo residual | Responsable |
|--------|--------|------|-----------------|-------------|
| [Riesgo 1] | [Medida] | [técnica / organizativa / contractual] | [B/M/A] | [Rol/Persona] |

## Análisis de Transferencia Transfronteriza

> Complete esta sección sólo si `pipl_cross_border_transfer: true`.
>
> Por PIPL Art. 38-40, la transferencia transfronteriza requiere **uno** de: (a) Evaluación de seguridad CAC, (b) Certificación de protección de información personal, (c) Contrato estándar archivado con CAC provincial.

- **Destino(s)**: [Países/regiones]
- **Mecanismo**: [security_assessment / certification / standard_contract / other]
- **Referencia de revisión de seguridad CAC**: [Número o N/A]
- **Justificación de necesidad**: [Por qué los datos deben salir de China continental]
- **Salvaguardas del receptor**: [Obligaciones contractuales impuestas]
- **Notificación al titular**: [Cómo se informa y obtiene consentimiento]

## Toma de Decisiones Automatizada (PIPL Art. 24)

> Complete esta sección si el procesamiento involucra decisión automatizada.

- **Divulgación de la lógica**: [Cómo se explica al titular]
- **Equidad y transparencia**: [Cómo se audita el trato diferenciado injusto]
- **Derecho a oponerse / revisión humana**: [Cómo se solicita intervención humana]
- **Marketing / push**: [Cómo se proporciona el opt-out]

## Consultas

- **Opinión del Oficial de Protección de Información Personal**: [Requerido si > 1.000.000 individuos — PIPL Art. 52]
- **Titulares consultados**: [Sí/No] — [Metodología]
- **CAC provincial consultado**: [Sí/No / No aplicable] — [Referencia]

## Retención y Revisión

- **PIPIA retenido hasta**: [YYYY-MM-DD — al menos 3 años desde la creación]
- **Próxima revisión**: [YYYY-MM-DD]
- **Eventos disparadores**:
  - Cambios en alcance o propósito
  - Nuevas categorías de información personal
  - Cambios regulatorios (enmiendas PIPL, guía CAC)
  - Incidentes de seguridad
- **Responsable de revisión**: [Rol/Persona]

## Aprobación

| Aprobado por | Fecha | Decisión | Condiciones |
|--------------|-------|----------|-------------|
| [Revisor] | [YYYY-MM-DD] | [aprobado / condicional / rechazado] | [Si aplica] |

<!-- Template: StrayMark | https://strangedays.tech -->
