# NIST AI RMF --- Guía de Implementación de la Función MANAGE

> **Marco de referencia**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Función**: MANAGE --- Respuesta a riesgos, mitigación y monitoreo
>
> La función MANAGE asigna recursos e implementa planes para responder, mitigar y monitorear los riesgos de IA de forma continua. Traduce las evaluaciones de riesgos en acciones concretas que reducen el daño y mantienen la confiabilidad a lo largo del ciclo de vida del sistema de IA.

---

## MG-1: Respuesta a Riesgos

Planificar y ejecutar respuestas a los riesgos identificados. Las respuestas a riesgos incluyen aceptación, evitación, transferencia o mitigación, cada una documentada con justificación clara y responsabilidad asignada.

> Cada riesgo identificado debe tener una estrategia de respuesta explícita. La aceptación de riesgos no documentada es negligencia implícita.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Decisiones de respuesta a riesgos | ADR | sección Decision and Rationale |
| Mitigaciones recomendadas | ETH | sección Recommendations |
| Mitigaciones arquitectónicas | ADR | sección Consequences |
| Priorización de respuesta | AI-RISK-CATALOG.md | columnas Priority, Status |
| Justificación de decisiones | AIDEC | secciones Alternatives Considered, Decision |

### Lista de Verificación de Implementación

- [ ] Para cada riesgo en AI-RISK-CATALOG.md, documentar la estrategia de respuesta elegida (aceptar, evitar, transferir, mitigar)
- [ ] Crear un ADR para las decisiones arquitectónicas tomadas para mitigar riesgos de alta severidad
- [ ] Registrar las recomendaciones de mitigación en la sección Recommendations del ETH
- [ ] Asignar responsables y plazos a cada acción de respuesta a riesgos

---

## MG-2: Mitigación de Riesgos

Implementar controles técnicos y organizacionales específicos para reducir la severidad o probabilidad de los riesgos. Verificar que las mitigaciones sean efectivas y no introduzcan nuevos riesgos.

> Las mitigaciones deben verificarse, no asumirse. Implementar controles, probarlos y documentar la evidencia de su efectividad.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Controles de seguridad | SEC | sección Controls Implemented |
| Mitigaciones de privacidad | DPIA | sección Mitigation Measures |
| Mitigaciones de sesgo | ETH | secciones Bias Evaluation, Recommendations |
| Salvaguardas técnicas | ADR | sección Implementation Details |
| Pruebas de mitigación | TES | pruebas de Mitigation Validation |

### Lista de Verificación de Implementación

- [ ] Documentar los controles de seguridad implementados en documentos SEC, referenciando los riesgos que abordan
- [ ] Completar la sección Mitigation Measures del DPIA para cada riesgo de privacidad identificado
- [ ] Crear documentos TES que validen específicamente la efectividad de las mitigaciones clave
- [ ] Actualizar los niveles de riesgo residual en AI-RISK-CATALOG.md después de que las mitigaciones se desplieguen y verifiquen

---

## MG-3: Gestión de Riesgos de Terceros

Gestionar los riesgos derivados de componentes de IA de terceros, modelos, fuentes de datos, APIs y servicios. Garantizar la transparencia de la cadena de suministro y establecer protecciones contractuales.

> Los componentes de terceros heredan y propagan riesgos. Las organizaciones siguen siendo responsables de los riesgos introducidos por sus proveedores.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Inventario de terceros | SBOM | secciones Third-Party Services, Components |
| Evaluación de riesgos de componentes | SBOM | sección Risk Assessment |
| Transparencia de la cadena de suministro | SBOM | columnas License, Version, Source |
| Evaluación de proveedores | ETH | sección Third-Party Risk |
| Seguimiento de dependencias | SBOM | sección Dependencies |

### Lista de Verificación de Implementación

- [ ] Mantener un SBOM actualizado que liste todos los componentes de IA de terceros, modelos y fuentes de datos
- [ ] Evaluar los riesgos de cada componente de terceros y documentarlos en la sección Risk Assessment del SBOM
- [ ] Evaluar a los proveedores críticos mediante documentos ETH dedicados cuando sus componentes afecten funciones de alto riesgo
- [ ] Establecer cadencias de revisión para componentes de terceros y registrar las fechas de revisión en el SBOM

---

## MG-4: Monitoreo Post-Despliegue

Monitorear los sistemas de IA desplegados para detectar degradación del rendimiento, riesgos emergentes, incidentes y cambios en el entorno operativo. Mantener la conciencia operativa durante todo el ciclo de vida del sistema.

> El despliegue no es el fin de la gestión de riesgos --- es donde los riesgos del mundo real comienzan a manifestarse. El monitoreo continuo es esencial.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Seguimiento de incidentes | INC | Documento completo |
| Estado del ciclo de vida | AI-LIFECYCLE-TRACKER.md | sección Monitoring Phase |
| Monitoreo operacional | AILOG | Entradas de monitoring and observability |
| Detección de deriva del rendimiento | TES | Pruebas de regresión y deriva |
| Salud del sistema | `devtrail status` | Métricas de salud de la documentación |

### Lista de Verificación de Implementación

- [ ] Crear documentos INC para cada incidente operacional, con clasificación de severidad y línea de tiempo
- [ ] Rastrear la fase del ciclo de vida del sistema en AI-LIFECYCLE-TRACKER.md y actualizar a medida que el sistema evoluciona
- [ ] Registrar cambios y observaciones relacionados con el monitoreo en entradas AILOG
- [ ] Ejecutar pruebas de regresión periódicas (TES) para detectar deriva del rendimiento en sistemas desplegados

---

## Resumen: Mapeo de la Función MANAGE a DevTrail

| Categoría | Descripción | Documento DevTrail Principal | Campos / Secciones Clave |
|----------|-------------|---------------------------|----------------------|
| MG-1 | Respuesta a Riesgos | ETH, ADR | Recommendations, Decision and Rationale |
| MG-2 | Mitigación de Riesgos | SEC, DPIA | Controls Implemented, Mitigation Measures |
| MG-3 | Riesgos de Terceros | SBOM | Third-Party Services, Risk Assessment |
| MG-4 | Monitoreo Post-Despliegue | INC, AI-LIFECYCLE-TRACKER.md | Documento completo, Monitoring Phase |

---

*Guía de Implementación de la Función MANAGE del NIST AI RMF --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
