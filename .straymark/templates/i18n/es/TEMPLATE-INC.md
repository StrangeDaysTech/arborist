---
id: INC-YYYY-MM-DD-NNN
title: [Título del incidente]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true
risk_level: high | critical
severity: SEV1 | SEV2 | SEV3 | SEV4
eu_ai_act_applicable: false
incident_report_deadline: null  # YYYY-MM-DD — fecha límite regulatoria si aplica
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
observability_scope: none        # none | basic | full — activar cuando la instrumentación OTel sea relevante
tags: []
related: []
incident_date: YYYY-MM-DD
resolved_date: null
---

# INC: [Título del Incidente]

> **ANÁLISIS PARCIAL**: Este documento contiene análisis de un agente de IA.
> Las conclusiones finales y acciones correctivas requieren revisión humana.

## Resumen del Incidente

| Campo | Valor |
|-------|-------|
| Severidad | [SEV1/SEV2/SEV3/SEV4] |
| Fecha/hora de inicio | [YYYY-MM-DD HH:MM UTC] |
| Fecha/hora de resolución | [YYYY-MM-DD HH:MM UTC] |
| Duración | [X horas Y minutos] |
| Servicios afectados | [Lista de servicios] |
| Usuarios afectados | [Estimación] |
| Impacto en negocio | [Descripción] |

## Definiciones de Severidad

| Severidad | Definición |
|-----------|------------|
| SEV1 | Caída total del servicio, impacto crítico en el negocio |
| SEV2 | Degradación severa, funcionalidad principal afectada |
| SEV3 | Degradación parcial, existen soluciones alternativas |
| SEV4 | Impacto menor, pocos usuarios afectados |

## Cronología

> Si su sistema usa OpenTelemetry, incluir trace-id para evidencia correlacionada.

| Hora (UTC) | Evento | Trace ID | Span ID | Enlace a Dashboard |
|------------|--------|----------|---------|-------------------|
| HH:MM | [Primer síntoma detectado] | [trace-id si disponible] | [span-id] | [enlace] |
| HH:MM | [Alerta disparada] | | | |
| HH:MM | [Equipo notificado] | | | |
| HH:MM | [Diagnóstico inicial] | | | |
| HH:MM | [Mitigación aplicada] | | | |
| HH:MM | [Servicio restaurado] | | | |
| HH:MM | [Incidente cerrado] | | | |

## Análisis de Causa Raíz

### Causa Inmediata
[Qué falló directamente]

### Causas Contribuyentes
1. [Factor contribuyente 1]
2. [Factor contribuyente 2]

### Causa Raíz (Análisis del Agente)
[Análisis del agente sobre la causa fundamental]

> **Nota**: Este análisis requiere validación del equipo técnico.

## Impacto

### Técnico
- [Impacto técnico 1]
- [Impacto técnico 2]

### Negocio
- [Impacto en negocio 1]
- [Impacto en negocio 2]

### Usuarios
- [Impacto en usuarios 1]
- [Impacto en usuarios 2]

## Acciones de Mitigación Tomadas

1. [Acción tomada para resolver el incidente]
2. [Acción tomada para resolver el incidente]

## Acciones Correctivas Propuestas

> Estas propuestas requieren revisión y priorización humana.

| # | Acción | Tipo | Prioridad | Responsable | Fecha límite |
|---|--------|------|-----------|-------------|--------------|
| 1 | [Acción] | Prevención | [Alta/Media/Baja] | [TBD] | [TBD] |
| 2 | [Acción] | Detección | [Alta/Media/Baja] | [TBD] | [TBD] |
| 3 | [Acción] | Respuesta | [Alta/Media/Baja] | [TBD] | [TBD] |

## Lecciones Aprendidas

### Qué funcionó bien
- [Aspecto positivo 1]
- [Aspecto positivo 2]

### Qué no funcionó
- [Aspecto a mejorar 1]
- [Aspecto a mejorar 2]

### Donde tuvimos suerte
- [Aspecto que pudo haber sido peor]

## Reporte de Incidentes EU AI Act

> Para sistemas de IA de alto riesgo bajo EU AI Act, los incidentes deben reportarse a la autoridad de vigilancia del mercado dentro de:
> - **15 días** (incidentes estándar)
> - **10 días** (incidentes que resulten en muerte)
> - **2 días** (incidentes generalizados o muy graves)
>
> Referencia: Artículo 73, EU AI Act.
>
> Completar esta sección solo si `eu_ai_act_applicable` es `true`.

| Campo | Valor |
|-------|-------|
| Fecha Límite de Reporte | [YYYY-MM-DD] |
| Autoridad Notificada | [Sí/No/NA] |
| Referencia del Reporte | [Número de referencia si fue enviado] |

## Preguntas Abiertas

1. [Pregunta que requiere investigación adicional]
2. [Pregunta para el equipo]

---

## Revisión Post-Mortem

| Campo | Valor |
|-------|-------|
| Revisado por | [Nombre] |
| Fecha de revisión | [YYYY-MM-DD] |
| Estado | [Borrador/Revisado/Cerrado] |

<!-- Template: DevTrail | https://strangedays.tech -->
