---
id: SEC-YYYY-MM-DD-NNN
title: "[Sistema/Componente] Evaluación de Seguridad"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true
risk_level: high
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
iso_42001_clause: [6, 8]
threat_model_methodology: STRIDE  # STRIDE | PASTA | LINDDUN | custom
owasp_asvs_level: 1  # 1 | 2 | 3
tags: [security]
related: []
---

# SEC: [Sistema/Componente] Evaluación de Seguridad

> **IMPORTANTE**: Este documento es un BORRADOR creado por un agente de IA.
> Requiere revisión y aprobación humana antes de proceder.

## Alcance y Objetivos

| Campo | Valor |
|-------|-------|
| Sistema Evaluado | [Nombre y versión del sistema o componente] |
| Tipo de Evaluación | [revisión de diseño / revisión de código / prueba de penetración / modelo de amenazas] |
| Fecha de Evaluación | [YYYY-MM-DD] |
| Evaluador | [Nombre del agente o evaluador humano] |

**Objetivos**:

- [Objetivo principal de esta evaluación de seguridad]
- [Objetivo secundario, si aplica]

**Dentro del Alcance**:

- [Componente, servicio o frontera incluido]
- [Componente, servicio o frontera incluido]

**Fuera del Alcance**:

- [Componente, servicio o frontera excluido]
- [Componente, servicio o frontera excluido]

## Modelo de Amenazas

> Metodología: **STRIDE** (Suplantación, Manipulación, Repudio, Divulgación de Información, Denegación de Servicio, Elevación de Privilegios).
> Reemplazar con PASTA, LINDDUN o personalizado según se indique en el frontmatter.

### Suplantación (Spoofing)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| S-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| S-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

### Manipulación (Tampering)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| T-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| T-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

### Repudio (Repudiation)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| R-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| R-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

### Divulgación de Información (Information Disclosure)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| I-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| I-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

### Denegación de Servicio (Denial of Service)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| D-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| D-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

### Elevación de Privilegios (Elevation of Privilege)

| ID Amenaza | Descripción | Probabilidad (1-5) | Impacto (1-5) | Puntuación de Riesgo | Mitigación |
|------------|-------------|:-------------------:|:-------------:|:--------------------:|------------|
| E-001 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |
| E-002 | [Descripción de la amenaza] | [1-5] | [1-5] | [P x I] | [Mitigación propuesta] |

## Cumplimiento OWASP ASVS

> Referencia: OWASP Application Security Verification Standard (ASVS) 5.0.
> Verificar controles al nivel indicado en el frontmatter (`owasp_asvs_level`).

| ID Control | Descripción | Nivel (L1/L2/L3) | Estado | Evidencia | Notas |
|------------|-------------|:-----------------:|:------:|-----------|-------|
| V1.1.1 | [Arquitectura — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V2.1.1 | [Autenticación — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V3.1.1 | [Gestión de Sesiones — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V4.1.1 | [Control de Acceso — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V5.1.1 | [Validación — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V6.1.1 | [Criptografía Almacenada — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V7.1.1 | [Manejo de Errores y Registro — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V8.1.1 | [Protección de Datos — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V9.1.1 | [Comunicaciones — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V10.1.1 | [Código Malicioso — descripción del control] | L2 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V11.1.1 | [Lógica de Negocio — descripción del control] | L2 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V12.1.1 | [Archivos y Recursos — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V13.1.1 | [API y Servicios Web — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V14.1.1 | [Configuración — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V50.1.1 | [OAuth y OIDC — descripción del control] | L1 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V51.1.1 | [Tokens Autocontenidos — descripción del control] | L2 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |
| V52.1.1 | [SAML — descripción del control] | L2 | [Pasa / Falla / NA] | [Enlace o referencia] | [Notas] |

> Agregar o eliminar filas según sea necesario. Enfocarse en los controles relevantes para el sistema evaluado.

## Vulnerabilidades Encontradas

| ID Vuln | CWE | Severidad (CVSS) | Descripción | Componente Afectado | Remediación | Estado |
|---------|-----|:----------------:|-------------|---------------------|-------------|:------:|
| VULN-001 | [CWE-XXX] | [0.0-10.0] | [Descripción de la vulnerabilidad] | [Componente o módulo] | [Pasos de remediación] | [abierta / mitigada / aceptada] |
| VULN-002 | [CWE-XXX] | [0.0-10.0] | [Descripción de la vulnerabilidad] | [Componente o módulo] | [Pasos de remediación] | [abierta / mitigada / aceptada] |
| VULN-003 | [CWE-XXX] | [0.0-10.0] | [Descripción de la vulnerabilidad] | [Componente o módulo] | [Pasos de remediación] | [abierta / mitigada / aceptada] |

## Controles de Seguridad

> Referencia: OWASP Software Assurance Maturity Model (SAMM).

| Función de Negocio | Práctica | Nivel de Madurez (1-3) | Estado Actual | Brechas |
|---------------------|----------|:----------------------:|---------------|---------|
| Gobernanza | Estrategia y Métricas | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Gobernanza | Política y Cumplimiento | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Gobernanza | Educación y Orientación | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Diseño | Evaluación de Amenazas | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Diseño | Requisitos de Seguridad | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Diseño | Arquitectura de Seguridad | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Implementación | Construcción Segura | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Implementación | Despliegue Seguro | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Implementación | Gestión de Defectos | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Verificación | Evaluación de Arquitectura | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Verificación | Pruebas Basadas en Requisitos | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Verificación | Pruebas de Seguridad | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Operaciones | Gestión de Incidentes | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Operaciones | Gestión de Entorno | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |
| Operaciones | Gestión Operativa | [1-3] | [Descripción del estado actual] | [Brechas identificadas] |

## Recomendaciones

| Prioridad | Descripción | Esfuerzo | Impacto |
|:---------:|-------------|:--------:|:-------:|
| Crítica | [Descripción de la recomendación] | [Bajo / Medio / Alto] | [Bajo / Medio / Alto] |
| Alta | [Descripción de la recomendación] | [Bajo / Medio / Alto] | [Bajo / Medio / Alto] |
| Media | [Descripción de la recomendación] | [Bajo / Medio / Alto] | [Bajo / Medio / Alto] |
| Baja | [Descripción de la recomendación] | [Bajo / Medio / Alto] | [Bajo / Medio / Alto] |

---

## Aprobación

| Campo | Valor |
|-------|-------|
| Aprobado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Decisión | [APROBADO / RECHAZADO / CONDICIONAL] |
| Condiciones | [Si aplica] |

<!-- Template: DevTrail | https://strangedays.tech -->
