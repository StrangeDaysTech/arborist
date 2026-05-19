---
id: TDE-YYYY-MM-DD-NNN
title: [Título de la deuda técnica]
status: identified
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high
type: code | architecture | infrastructure | documentation | testing
impact: low | medium | high
effort: low | medium | high
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
tags: []
related: []
priority: null
assigned_to: null
---

# TDE: [Título de la Deuda Técnica]

> **IDENTIFICADO POR AGENTE**: La priorización y asignación requieren decisión humana.

## Resumen

[Descripción breve de la deuda técnica identificada]

## Tipo de Deuda

- [ ] **Código**: Código difícil de mantener, duplicado o mal estructurado
- [ ] **Arquitectura**: Decisiones arquitectónicas subóptimas
- [ ] **Infraestructura**: Configuraciones o dependencias problemáticas
- [ ] **Documentación**: Documentación faltante o desactualizada
- [ ] **Testing**: Cobertura insuficiente o pruebas frágiles

## Ubicación

| Archivo/Componente | Descripción |
|--------------------|-------------|
| `path/to/file.ts` | [Cuál es el problema] |
| `path/to/component/` | [Cuál es el problema] |

## Descripción del Problema

[Descripción detallada de por qué esto es deuda técnica]

### Síntomas Observados
- [Síntoma 1: ej. "El archivo tiene más de 1000 líneas"]
- [Síntoma 2: ej. "Hay 5 funciones que hacen casi lo mismo"]

### Causa Original
[Por qué se generó esta deuda - si se conoce]

## Impacto

### En Desarrollo
- [Cómo afecta al equipo de desarrollo]

### En Mantenimiento
- [Cómo dificulta el mantenimiento]

### En Rendimiento (si aplica)
- [Impacto en rendimiento]

### En Seguridad (si aplica)
- [Riesgos de seguridad]

## Solución Propuesta

[Descripción de cómo podría resolverse]

### Enfoque Recomendado
1. [Paso 1]
2. [Paso 2]
3. [Paso 3]

### Alternativas
- [Alternativa 1]: [Breve descripción]
- [Alternativa 2]: [Breve descripción]

## Estimación

| Aspecto | Valor | Justificación |
|---------|-------|---------------|
| Esfuerzo | [Bajo/Medio/Alto] | [Por qué] |
| Impacto de resolver | [Bajo/Medio/Alto] | [Por qué] |
| Riesgo de no resolver | [Bajo/Medio/Alto] | [Por qué] |
| Urgencia | [Bajo/Medio/Alto] | [Por qué] |

## Matriz de Priorización (referencia para humanos)

```
         │ Bajo Esfuerzo │ Alto Esfuerzo │
─────────┼───────────────┼───────────────┤
Alto     │  HACER AHORA  │   PLANIFICAR  │
Impacto  │               │               │
─────────┼───────────────┼───────────────┤
Bajo     │  QUICK WIN    │   CONSIDERAR  │
Impacto  │               │               │
```

## Dependencias

- [Otras deudas que deberían resolverse primero]
- [Features que podrían verse afectadas]

## Notas del Agente

[Contexto adicional, observaciones o recomendaciones]

---

## Decisión de Priorización

| Campo | Valor |
|-------|-------|
| Priorizado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Prioridad asignada | [P1/P2/P3/Backlog/No se resolverá] |
| Sprint/Milestone | [Si aplica] |
| Asignado a | [Equipo/Persona] |
| Comentarios | [Notas] |

<!-- Template: DevTrail | https://strangedays.tech -->
