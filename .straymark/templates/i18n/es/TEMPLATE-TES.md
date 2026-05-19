---
id: TES-YYYY-MM-DD-NNN
title: [Título del plan de pruebas]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium
review_required: true
risk_level: low | medium
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
observability_scope: none        # none | basic | full — activar cuando la instrumentación OTel sea relevante
tags: []
related: []
---

# TES: [Título del Plan de Pruebas]

> **PROPUESTA**: Este plan fue creado por un agente de IA y requiere validación.
>
> **Alineación con estándares**: ISO/IEC/IEEE 29119-3:2021 (Pruebas de Software — Documentación de Pruebas)
>
> Este template corresponde al nivel de **Plan de Pruebas** en la jerarquía de ISO 29119-3:
> - **Política de Pruebas Organizacional** — Principios de pruebas a nivel organización (fuera del alcance de DevTrail)
> - **Estrategia de Pruebas** — Estrategia de pruebas a nivel proyecto (puede referenciarse en gobernanza del proyecto)
> - **Plan de Pruebas** — Este documento: planificación de pruebas específica para una funcionalidad, componente o cambio

## Alcance

### Dentro del Alcance
- [Funcionalidad/componente a probar 1]
- [Funcionalidad/componente a probar 2]

### Fuera del Alcance
- [Qué no se probará y por qué]

## Enfoque de Pruebas

> *Según ISO/IEC/IEEE 29119-3:2021. Describe el enfoque general de pruebas para este plan.*

### Técnicas de Diseño de Pruebas

| Técnica | Aplicación | Justificación |
|---------|-----------|---------------|
| Partición de Equivalencia | [Dónde se aplica] | [Por qué se eligió] |
| Análisis de Valores Límite | [Dónde se aplica] | [Por qué se eligió] |
| Tabla de Decisión | [Dónde se aplica] | [Por qué se eligió] |
| Transición de Estados | [Dónde se aplica] | [Por qué se eligió] |
| Exploratoria | [Dónde se aplica] | [Por qué se eligió] |

### Tipos de Pruebas y Cobertura

| Tipo | Cobertura | Herramienta | Justificación |
|------|-----------|-------------|---------------|
| Unitarias | [%] | [Jest/Vitest/etc.] | [Por qué este nivel] |
| Integración | [%] | [Herramienta] | [Por qué este nivel] |
| E2E | [Casos críticos] | [Cypress/Playwright/etc.] | [Por qué estos casos] |
| Rendimiento | [Si aplica] | [Herramienta] | [Por qué es necesario] |

### Criterios de Completitud de Pruebas

- [ ] [Criterio 1, ej. "Todos los casos de prueba críticos pasan"]
- [ ] [Criterio 2, ej. "Cobertura de código >= X%"]
- [ ] [Criterio 3, ej. "Sin defectos abiertos de severidad 1"]
- [ ] [Criterio 4, ej. "Rendimiento dentro de umbrales de SLA"]

### Criterios de Suspensión y Reanudación

**Criterios de suspensión** (condiciones para detener las pruebas):
- [ej. "El build falla al desplegarse en el entorno de pruebas"]
- [ej. "Se encuentra un defecto bloqueante en la ruta crítica"]

**Criterios de reanudación** (condiciones para reanudar las pruebas):
- [ej. "Defecto bloqueante resuelto y verificado"]
- [ej. "Entorno de pruebas restaurado y estable"]

## Casos de Prueba

### Funcionalidad: [Nombre]

| ID | Caso | Precondiciones | Pasos | Resultado Esperado | Prioridad |
|----|------|----------------|-------|-------------------|-----------|
| TC-001 | [Nombre] | [Setup] | 1. [Paso] | [Esperado] | Alta |
| TC-002 | [Nombre] | [Setup] | 1. [Paso] | [Esperado] | Media |

### Casos Negativos

| ID | Caso | Entrada Inválida | Resultado Esperado |
|----|------|------------------|-------------------|
| TC-N01 | [Nombre] | [Entrada] | [Error esperado] |

### Casos de Borde

| ID | Caso | Condición | Resultado Esperado |
|----|------|-----------|-------------------|
| TC-E01 | [Nombre] | [Condición límite] | [Esperado] |

## Requisitos de Datos de Prueba

> Según ISO/IEC/IEEE 29119-3:2021.

| ID Conjunto de Datos | Origen | Pasos de Preparación | Clasificación de Sensibilidad | Política de Retención |
|---------------------|--------|---------------------|-------------------------------|----------------------|
| TD-001 | [Origen] | [Cómo preparar] | [Público/Interno/Confidencial/Restringido] | [Retener/Eliminar tras prueba] |
| TD-002 | [Origen] | [Cómo preparar] | [Clasificación] | [Política] |

### Fixtures Requeridos
- [Fixture 1]: [Descripción]
- [Fixture 2]: [Descripción]

### Mocks Requeridos
- [Mock 1]: [Qué simula]
- [Mock 2]: [Qué simula]

## Requisitos del Entorno de Pruebas

> Según ISO/IEC/IEEE 29119-3:2021.

| Componente | Versión | Configuración | Dependencias |
|-----------|---------|---------------|-------------|
| [SO/Runtime] | [Versión] | [Config específica] | [Servicios requeridos] |
| [Base de datos] | [Versión] | [Schema/seed] | [Conectividad] |
| [Servicio externo] | [Versión/API] | [Stub/live] | [Auth/red] |

- **Entorno**: [Local/CI/Staging]
- **Configuración especial**: [Si aplica]
- **Dependencias externas**: [Lista]

## Pruebas de Observabilidad

> Completar esta sección cuando el proyecto use OpenTelemetry o tenga requisitos de observabilidad.
> Activar con tag `observabilidad`.

| ID Prueba | Descripción | Resultado Esperado | Estado |
|-----------|------------|-------------------|--------|
| OBS-01 | Verificar propagación de W3C Trace Context en llamadas internas | Header `traceparent` presente en todas las solicitudes descendentes | [ ] |
| OBS-02 | Verificar propagación de W3C Trace Context en llamadas externas/asíncronas | `traceparent` propagado a través de colas de mensajes y procesos asíncronos | [ ] |
| OBS-03 | Validar correlación log-trace | `trace_id` y `span_id` presentes en entradas de log estructurado | [ ] |
| OBS-04 | Probar muestreo head bajo carga | La tasa de muestreo coincide con el porcentaje configurado bajo carga sostenida | [ ] |
| OBS-05 | Probar muestreo tail para casos de error (si aplica) | Las trazas de error se capturan independientemente de la tasa de muestreo head | [ ] |
| OBS-06 | Verificar redacción de datos sensibles en Collector | PII, tokens y secretos son redactados antes de llegar al backend | [ ] |

## Criterios de Aceptación

- [ ] Cobertura mínima de [X]%
- [ ] Todos los casos críticos pasan
- [ ] Sin regresiones en funcionalidad existente
- [ ] Rendimiento dentro de umbrales aceptables

## Riesgos y Mitigaciones

| Riesgo | Probabilidad | Impacto | Mitigación |
|--------|--------------|---------|------------|
| [Riesgo] | [Alta/Media/Baja] | [Alto/Medio/Bajo] | [Acción] |

## Resultados

> Usar terminología de ISO/IEC/IEEE 29119-3:2021 para artefactos de documentación de pruebas.

### Registro de Ejecución de Pruebas

| ID TC | Fecha | Tester | Resultado | Salida Real | Notas |
|-------|-------|--------|-----------|-------------|-------|
| TC-001 | [Fecha] | [Agente/Humano] | [Pasa/Falla/Bloqueado] | [Real] | [Notas] |

### Reportes de Incidentes de Prueba

> Documentar cualquier defecto o comportamiento inesperado encontrado durante la ejecución.

| ID Incidente | ID TC | Severidad | Descripción | Estado |
|-------------|-------|-----------|-------------|--------|
| TI-001 | [TC-XXX] | [Crítica/Mayor/Menor] | [Descripción] | [Abierto/Resuelto] |

### Reporte de Estado de Pruebas

| Métrica | Valor |
|---------|-------|
| Total casos de prueba | [N] |
| Pasaron | [N] |
| Fallaron | [N] |
| Bloqueados | [N] |
| No ejecutados | [N] |
| Tasa de éxito | [%] |

### Reporte de Completitud de Pruebas

- **Fecha de completitud**: [YYYY-MM-DD]
- **Criterios de completitud cumplidos**: [Sí/No — listar criterios no cumplidos]
- **Riesgos pendientes**: [Riesgos residuales de incidentes no resueltos]
- **Recomendación**: [Proceder al release / Detener para correcciones / Pruebas adicionales necesarias]

---

## Validación

| Campo | Valor |
|-------|-------|
| Validado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Comentarios | [Notas] |

<!-- Template: DevTrail | https://strangedays.tech -->
