---
id: ETH-YYYY-MM-DD-NNN
title: [Título de la revisión ética]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium | low
review_required: true
risk_level: high | critical
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
gdpr_legal_basis: none          # consent | contract | legal_obligation | vital_interests | public_task | legitimate_interests | none
fria_required: false            # Evaluación de Impacto en Derechos Fundamentales
tags: []
related: []
approved_by: null
approved_date: null
---

# ETH: [Título de la Revisión Ética]

> **IMPORTANTE**: Este documento es un BORRADOR creado por un agente de IA.
> Requiere revisión y aprobación humana antes de proceder.

## Resumen Ejecutivo

[Descripción breve del tema ético a considerar]

## Contexto

[Describir la situación que genera consideraciones éticas]

## Clasificación de Riesgo EU AI Act

> Referencia: EU AI Act, Anexo III.

| Nivel de Riesgo | Obligaciones | Aplica |
|-----------------|-------------|--------|
| **Inaceptable** | Prohibido — el sistema no puede desplegarse | [ ] |
| **Alto** | Cumplimiento total requerido — evaluación de conformidad, marcado CE, registro, monitoreo post-mercado | [ ] |
| **Limitado** | Obligaciones de transparencia — los usuarios deben ser informados de la interacción con IA | [ ] |
| **Mínimo** | Sin obligaciones específicas — se fomentan códigos de conducta voluntarios | [ ] |

- **Categoría Anexo III** (si alto riesgo): [Especificar: biometría, infraestructura crítica, educación, empleo, servicios esenciales, fuerzas del orden, migración, justicia, procesos democráticos]
- **Checklist de obligaciones** (si alto riesgo):
  - [ ] Sistema de gestión de riesgos establecido (Art. 9)
  - [ ] Requisitos de gobernanza de datos cumplidos (Art. 10)
  - [ ] Documentación técnica preparada (Art. 11, Anexo IV)
  - [ ] Registro habilitado (Art. 12)
  - [ ] Información de transparencia proporcionada (Art. 13)
  - [ ] Medidas de supervisión humana implementadas (Art. 14)
  - [ ] Precisión, robustez y ciberseguridad aseguradas (Art. 15)

## Áreas de Preocupación

### 1. Privacidad de Datos

- **Datos involucrados**: [Qué datos se procesan]
- **Sensibilidad**: [PII, datos de salud, financieros, etc.]
- **Jurisdicciones**: [GDPR, CCPA, etc.]
- **Preocupaciones**: [Listar preocupaciones específicas]

#### Base Legal GDPR

> Según GDPR Art. 6. Completar cuando se procesen datos personales.

| Actividad de Procesamiento | Base Legal (Art. 6) | Justificación | Periodo de Retención de Datos |
|---------------------------|--------------------|--------------|-----------------------------|
| [Actividad] | [consentimiento/contrato/obligación_legal/intereses_vitales/tarea_pública/intereses_legítimos] | [Por qué aplica esta base] | [Periodo] |

#### Referencia a Evaluación de Impacto en Protección de Datos

- **Existe DPIA**: [Sí/No]
- **Documento DPIA**: [DPIA-YYYY-MM-DD-NNN si aplica]

### 2. Sesgo y Equidad

- **Grupos afectados**: [Quiénes podrían verse afectados]
- **Riesgos de sesgo**: [Posibles sesgos identificados]
- **Mitigaciones propuestas**: [Cómo abordarlos]

#### Características Protegidas

| Característica | Potencialmente Afectada | Evaluación | Mitigación |
|---------------|------------------------|-----------|------------|
| Edad | [Sí/No] | [Evaluación] | [Mitigación] |
| Discapacidad | [Sí/No] | [Evaluación] | [Mitigación] |
| Género | [Sí/No] | [Evaluación] | [Mitigación] |
| Raza / Etnia | [Sí/No] | [Evaluación] | [Mitigación] |
| Religión / Creencia | [Sí/No] | [Evaluación] | [Mitigación] |
| Orientación Sexual | [Sí/No] | [Evaluación] | [Mitigación] |
| Estatus Socioeconómico | [Sí/No] | [Evaluación] | [Mitigación] |
| Otra | [Sí/No] | [Evaluación] | [Mitigación] |

### 3. Transparencia

- **Comunicación al usuario**: [Qué se comunica]
- **Consentimiento**: [Cómo se obtiene]
- **Derecho a explicación**: [Cómo se garantiza]

### 4. Seguridad

- **Riesgos identificados**: [Listar riesgos]
- **Impacto potencial**: [Consecuencias de una brecha]
- **Controles propuestos**: [Medidas de seguridad]

### 5. Impacto Social

- **Beneficios**: [Para quién y cómo]
- **Daños potenciales**: [A quién y cómo]
- **Balance**: [Análisis costo-beneficio]

## Impacto Ambiental

> Completar cuando el sistema involucre entrenamiento de modelos de IA o recursos de cómputo significativos.

| Métrica | Valor | Notas |
|---------|-------|-------|
| Estimación de Energía de Entrenamiento (kWh) | [Valor] | [Metodología] |
| Equivalente de CO2 (toneladas) | [Valor] | [Intensidad de carbono de la red eléctrica utilizada] |
| Hardware Utilizado | [GPUs/TPUs, cantidad] | [Proveedor cloud/región] |
| Costo de Inferencia por Solicitud | [Valor] | [Promedio/pico] |
| Medidas de Mitigación | [Descripción] | [Compensaciones de carbono, arquitecturas eficientes, etc.] |

## Potencial de Doble Uso

> Evaluar si el sistema podría ser reutilizado para aplicaciones dañinas.

- **Usos Beneficiosos**:
  - [Uso beneficioso previsto 1]
  - [Uso beneficioso previsto 2]
- **Usos Indebidos Potenciales**:
  - [Uso indebido potencial 1]
  - [Uso indebido potencial 2]
- **Salvaguardas Implementadas**:
  - [Salvaguarda 1]
  - [Salvaguarda 2]
- **Evaluación de Riesgo Residual**: [Bajo/Medio/Alto — describir riesgos remanentes tras salvaguardas]

## Evaluación de Impacto en Derechos Fundamentales (FRIA)

> Requerida por Art. 27 EU AI Act para implementadores de sistemas de IA de alto riesgo.
> Completar esta sección solo si `eu_ai_act_risk` es `high` y el sistema está siendo desplegado.

- **Categorías de Personas Afectadas**: [Quiénes son afectados por el uso del sistema de IA]
- **Riesgos Específicos a Derechos Fundamentales**: [Identificar riesgos a dignidad, no discriminación, privacidad, protección de datos, libertad de expresión, etc.]
- **Periodo y Frecuencia de Uso**: [Con qué frecuencia y durante cuánto tiempo se usará el sistema]
- **Medidas de Gobernanza**: [Medidas organizacionales para asegurar derechos fundamentales]
- **Procesos de Supervisión Humana**: [Cómo se implementa la supervisión humana según Art. 14]

## Recomendaciones del Agente

1. [Recomendación 1]
2. [Recomendación 2]
3. [Recomendación 3]

## Preguntas para el Revisor Humano

1. [Pregunta que el agente no puede resolver]
2. [Decisión que requiere juicio humano]
3. [Aspecto que necesita validación]

## Lista de Verificación de Revisión

### Para el Revisor Humano

- [ ] He leído y entendido las preocupaciones planteadas
- [ ] He evaluado los riesgos de privacidad
- [ ] He considerado el impacto en grupos vulnerables
- [ ] He verificado el cumplimiento regulatorio
- [ ] He evaluado las mitigaciones propuestas
- [ ] He revisado la clasificación de riesgo EU AI Act (si aplica)
- [ ] He revisado la FRIA (si despliegue de alto riesgo)
- [ ] He revisado la evaluación de impacto ambiental (si aplica)

### Decisión

- [ ] **APROBADO** — Proceder según lo propuesto
- [ ] **APROBADO CON CONDICIONES** — Proceder con modificaciones
- [ ] **RECHAZADO** — No proceder
- [ ] **REQUIERE MÁS ANÁLISIS** — Escalar o investigar más

## Notas del Revisor

[Espacio para que el revisor humano documente su análisis y decisión]

---

## Aprobación

| Campo | Valor |
|-------|-------|
| Aprobado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Decisión | [APROBADO/RECHAZADO/CONDICIONAL] |
| Condiciones | [Si aplica] |

<!-- Template: DevTrail | https://strangedays.tech -->
