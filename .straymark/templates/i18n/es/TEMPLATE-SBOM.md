---
id: SBOM-YYYY-MM-DD-NNN
title: "[Sistema/Componente] SBOM de IA"
status: accepted
created: YYYY-MM-DD
agent: [agent-name]
confidence: high
review_required: false  # Inventario factual
risk_level: low
iso_42001_clause: [8]
sbom_format_reference: SPDX-3.0 | CycloneDX-1.6 | custom
system_name: ""
tags: [sbom, supply-chain]
related: []
---

# SBOM: [Sistema/Componente] Lista de Materiales de Software para IA

## Componentes de IA/ML

> Esta sección se mapea a CycloneDX `component` con `type: machine-learning-model`.

| Nombre del Componente | Versión | Proveedor | Tipo | Licencia | Nivel de Riesgo | Estado de Vulnerabilidades | Última Auditoría |
|-----------------------|---------|-----------|------|----------|-----------------|---------------------------|------------------|
| [Componente 1] | [x.y.z] | [Proveedor] | modelo | [Licencia] | [Bajo/Medio/Alto] | [Limpio/Vulnerable] | [YYYY-MM-DD] |
| [Componente 2] | [x.y.z] | [Proveedor] | librería | [Licencia] | [Bajo/Medio/Alto] | [Limpio/Vulnerable] | [YYYY-MM-DD] |
| [Componente 3] | [x.y.z] | [Proveedor] | servicio | [Licencia] | [Bajo/Medio/Alto] | [Limpio/Vulnerable] | [YYYY-MM-DD] |
| [Componente 4] | [x.y.z] | [Proveedor] | dataset | [Licencia] | [Bajo/Medio/Alto] | [Limpio/Vulnerable] | [YYYY-MM-DD] |

## Fuentes de Datos de Entrenamiento

> Alineado con ISO 42001 Anexo A.7 (Datos para Sistemas de IA).

| Dataset | Fuente | Licencia | PII Incluida | Resumen de Evaluación de Sesgo | Procedencia de Datos | Política de Retención |
|---------|--------|----------|--------------|-------------------------------|---------------------|----------------------|
| [Dataset 1] | [Fuente] | [Licencia] | [Sí/No] | [Resumen] | [Procedencia] | [Política] |
| [Dataset 2] | [Fuente] | [Licencia] | [Sí/No] | [Resumen] | [Procedencia] | [Política] |

## Servicios de IA de Terceros

| Servicio | Proveedor | Propósito | Datos Compartidos | DPA Vigente | SLA | Región | Certificaciones de Cumplimiento |
|----------|-----------|-----------|-------------------|-------------|-----|--------|--------------------------------|
| [Servicio 1] | [Proveedor] | [Propósito] | [Tipos de datos] | [Sí/No] | [Términos SLA] | [Región] | [SOC2, ISO 27001, etc.] |
| [Servicio 2] | [Proveedor] | [Propósito] | [Tipos de datos] | [Sí/No] | [Términos SLA] | [Región] | [SOC2, ISO 27001, etc.] |

## Dependencias de Software

> Considerar generar esta sección automáticamente con herramientas como `syft` o `trivy`.

| Paquete | Versión | Licencia | Vulnerabilidades Conocidas | Última Actualización |
|---------|---------|----------|---------------------------|---------------------|
| [Paquete 1] | [x.y.z] | [Licencia] | [CVE-YYYY-NNNNN, ...] | [YYYY-MM-DD] |
| [Paquete 2] | [x.y.z] | [Licencia] | [Ninguna] | [YYYY-MM-DD] |
| [Paquete 3] | [x.y.z] | [Licencia] | [CVE-YYYY-NNNNN] | [YYYY-MM-DD] |

## Evaluación de Riesgo de Cadena de Suministro

> Alineado con NIST AI 600-1 Categoría 12: Cadena de Valor e Integración de Componentes.

- **Nivel de Riesgo General**: [Bajo/Medio/Alto/Crítico]

- **Riesgos Clave Identificados**:
  - [Riesgo 1: Descripción]
  - [Riesgo 2: Descripción]
  - [Riesgo 3: Descripción]

- **Mitigaciones**:
  - [Mitigación 1: Descripción]
  - [Mitigación 2: Descripción]
  - [Mitigación 3: Descripción]

- **Plan de Monitoreo**:
  - [Actividad de monitoreo 1: Frecuencia y responsable]
  - [Actividad de monitoreo 2: Frecuencia y responsable]
  - [Actividad de monitoreo 3: Frecuencia y responsable]

<!-- Template: DevTrail | https://strangedays.tech -->
