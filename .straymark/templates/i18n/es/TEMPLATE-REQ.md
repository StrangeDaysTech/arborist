---
id: REQ-YYYY-MM-DD-NNN
title: [Título del requisito]
status: draft
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: medium | low
review_required: true
risk_level: low | medium | high
type: functional | non-functional | constraint
priority: must | should | could | wont
stakeholder_type: end_user | operator | acquirer | regulator | maintainer | developer
observability_scope: none        # none | basic | full — activar cuando la instrumentación OTel sea relevante
api_spec_path: ""               # Ruta al archivo de spec OpenAPI/AsyncAPI si este requisito involucra APIs
tags: []
related: []
validated_by: null
validated_date: null
---

# REQ: [Título del Requisito]

> **PROPUESTA**: Este requisito fue propuesto por un agente de IA.
> Requiere validación humana.
>
> **Alineación con estándares**: ISO/IEC/IEEE 29148:2018 (Ingeniería de Requisitos)

## Descripción

[Descripción clara y concisa del requisito]

## Tipo de Parte Interesada

Identificar quién origina este requisito (según ISO/IEC/IEEE 29148:2018 §6.2):

- [ ] **Usuario final** — Persona que interactúa con el sistema
- [ ] **Operador** — Persona que opera el sistema
- [ ] **Adquirente** — Organización que adquiere el sistema
- [ ] **Regulador** — Autoridad con supervisión regulatoria
- [ ] **Mantenedor** — Persona responsable del mantenimiento del sistema
- [ ] **Desarrollador** — Persona que construye el sistema

## Tipo de Requisito

- [ ] Funcional
- [ ] No funcional (ver características de calidad abajo)
- [ ] Restricción

## Prioridad (MoSCoW)

- [ ] **Must have** — Obligatorio para el MVP
- [ ] **Should have** — Importante pero no crítico
- [ ] **Could have** — Deseable si hay tiempo
- [ ] **Won't have** — Fuera del alcance actual

## Justificación

[Por qué este requisito es necesario. Qué problema resuelve o qué valor proporciona]

## Criterios de Aceptación

1. **Dado** [contexto inicial]
   **Cuando** [acción del usuario]
   **Entonces** [resultado esperado]

2. **Dado** [contexto inicial]
   **Cuando** [acción del usuario]
   **Entonces** [resultado esperado]

## Requisitos No Funcionales

> Categorías alineadas con ISO/IEC 25010:2023. Ver `00-governance/ISO-25010-2023-REFERENCE.md` para definiciones completas.
> Completar solo las secciones relevantes para este requisito.

### Idoneidad Funcional

- Completitud: [Grado en que las funciones cubren todas las tareas especificadas]
- Corrección: [Precisión requerida de los resultados]
- Pertinencia: [Qué tan bien las funciones facilitan el cumplimiento de tareas]

### Eficiencia de Desempeño

- Comportamiento Temporal: [Tiempos de respuesta, requisitos de throughput]
- Utilización de Recursos: [Restricciones de memoria, CPU, almacenamiento]
- Capacidad: [Límites máximos y requisitos de carga]

### Compatibilidad

- Coexistencia: [Restricciones de entorno compartido]
- Interoperabilidad: [Sistemas con los que integrar, formatos de intercambio]

### Capacidad de Interacción

> *Renombrada de "Usabilidad" en ISO 25010:2023*

- Reconocimiento de Pertinencia: [Facilidad para reconocer si el producto es apropiado]
- Aprendizaje: [Expectativas de curva de aprendizaje]
- Operabilidad: [Requisitos de facilidad de operación]
- Protección contra Errores de Usuario: [Mecanismos de prevención de errores]
- Compromiso del Usuario: [Requisitos de engagement y motivación]
- Inclusividad: [Rango de características de usuario a soportar]
- Asistencia al Usuario: [Requisitos de ayuda y guía]
- Auto-descriptividad: [Requisitos de capacidad auto-evidente]

### Fiabilidad

- Ausencia de Faltas: [Operación sin fallos esperada en condiciones normales]
- Disponibilidad: [Requisitos de tiempo activo, ej. 99.9%]
- Tolerancia a Fallos: [Comportamiento ante fallos de hardware/software]
- Capacidad de Recuperación: [Objetivo de tiempo de recuperación, requisitos de recuperación de datos]

### Seguridad

- Confidencialidad: [Requisitos de control de acceso a datos]
- Integridad: [Requisitos de protección de datos]
- No Repudio: [Requisitos de auditoría]
- Responsabilidad: [Requisitos de trazabilidad de acciones]
- Autenticidad: [Requisitos de verificación de identidad]
- Resistencia: [Requisitos de resistencia a ataques]

### Mantenibilidad

- Modularidad: [Requisitos de aislamiento de componentes]
- Reusabilidad: [Expectativas de reutilización]
- Analizabilidad: [Requisitos de evaluación de impacto]
- Modificabilidad: [Requisitos de facilidad de modificación]
- Capacidad de Prueba: [Requisitos de testing]

### Flexibilidad

> *Renombrada de "Portabilidad" en ISO 25010:2023*

- Adaptabilidad: [Requisitos de adaptación a entornos]
- Instalabilidad: [Requisitos de instalación]
- Capacidad de Reemplazo: [Requisitos de reemplazo]
- Escalabilidad: [Requisitos de manejo de carga creciente/decreciente]

### Seguridad Física (Safety)

> *Nueva en ISO 25010:2023 — especialmente relevante para sistemas de IA*

- Restricción Operacional: [Parámetros seguros de operación]
- Identificación de Riesgos: [Requisitos de detección de riesgos]
- Modo Seguro: [Requisitos de modo seguro/fallback]
- Advertencia de Peligros: [Requisitos de mecanismos de advertencia]
- Integración Segura: [Requisitos de seguridad durante la integración]

## Requisitos de Observabilidad

> Completar esta sección cuando el proyecto use OpenTelemetry o tenga requisitos de observabilidad.
> Activar con tag `observabilidad`.

| Requisito | Valor | Notas |
|-----------|-------|-------|
| Cobertura | [Endpoints/servicios que deben generar trazas] | [ej. "Todos los endpoints de API pública"] |
| Calidad de Trazas | [% mínimo de spans con atributos requeridos] | [ej. "95% de spans deben tener service.name, service.version"] |
| Latencia Máxima de Trazas | [Tiempo máximo aceptable para disponibilidad de trazas] | [ej. "< 30s desde emisión hasta backend"] |
| Política de Retención | [Periodo de retención por entorno] | [ej. "prod: 30 días, dev: 7 días"] |
| SLOs Ligados a Métricas Observables | [SLOs que dependen de métricas OTel] | [ej. "p99 latencia < 500ms medido via histograma OTel"] |

## Interfaces Externas

> Según ISO/IEC/IEEE 29148:2018 §9.4.2. Documentar interfaces con sistemas externos.

### Interfaces de Usuario

| ID Interfaz | Descripción | Origen | Protocolo/Formato | Elementos de Datos | Restricciones |
|-------------|-------------|--------|-------------------|-------------------|---------------|
| UI-001 | [Descripción] | [Origen] | [Protocolo] | [Datos] | [Restricciones] |

### Interfaces de Hardware

| ID Interfaz | Descripción | Origen | Protocolo/Formato | Elementos de Datos | Restricciones |
|-------------|-------------|--------|-------------------|-------------------|---------------|
| HW-001 | [Descripción] | [Origen] | [Protocolo] | [Datos] | [Restricciones] |

### Interfaces de Software

| ID Interfaz | Descripción | Origen | Protocolo/Formato | Elementos de Datos | Restricciones |
|-------------|-------------|--------|-------------------|-------------------|---------------|
| SW-001 | [Descripción] | [Origen] | [Protocolo] | [Datos] | [Restricciones] |

### Interfaces de Comunicaciones

| ID Interfaz | Descripción | Origen | Protocolo/Formato | Elementos de Datos | Restricciones |
|-------------|-------------|--------|-------------------|-------------------|---------------|
| COM-001 | [Descripción] | [Origen] | [Protocolo] | [Datos] | [Restricciones] |

## Dependencias

| Tipo | ID | Descripción |
|------|-----|-------------|
| Requiere | [REQ-XXX] | [Descripción] |
| Bloquea | [REQ-XXX] | [Descripción] |
| Relacionado | [ADR-XXX] | [Descripción] |

## Restricciones

- [Restricción técnica 1]
- [Restricción de negocio 1]
- [Restricción regulatoria si aplica]

## Supuestos

- [Supuesto 1]
- [Supuesto 2]

## Trazabilidad

> Según ISO/IEC/IEEE 29148:2018 §6.3. Establecer trazabilidad desde necesidades del interesado hasta requisitos del sistema y pruebas de aceptación.

| Necesidad del Interesado | Requisito del Sistema | Prueba de Aceptación |
|--------------------------|----------------------|---------------------|
| [Descripción de necesidad] | [REQ-YYYY-MM-DD-NNN] | [TES-YYYY-MM-DD-NNN / TC-NNN] |
| [Descripción de necesidad] | [REQ-YYYY-MM-DD-NNN] | [TES-YYYY-MM-DD-NNN / TC-NNN] |

## Verificación y Validación

> Según ISO/IEC/IEEE 29148:2018 §6.6. Definir cómo se verificará cada requisito.

| ID Requisito | Método de Verificación | Criterios de Aceptación | Responsable |
|-------------|----------------------|------------------------|-------------|
| [REQ-XXX] | inspección \| análisis \| demostración \| prueba | [Criterios medibles] | [Rol/Nombre] |
| [REQ-XXX] | inspección \| análisis \| demostración \| prueba | [Criterios medibles] | [Rol/Nombre] |

## Notas del Agente

[Contexto adicional, preguntas o sugerencias del agente que propone]

---

## Validación

| Campo | Valor |
|-------|-------|
| Validado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Estado | [Validado/Rechazado/Modificado] |
| Comentarios | [Notas del validador] |

<!-- Template: DevTrail | https://strangedays.tech -->
