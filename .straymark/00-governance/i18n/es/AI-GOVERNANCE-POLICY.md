# Política de Gobernanza de IA

> **Alineado con**: ISO/IEC 42001:2023 — Sistema de Gestión de Inteligencia Artificial (AIMS)
>
> Este documento proporciona una plantilla de gobernanza para organizaciones que usan DevTrail. Mapea las cláusulas de ISO 42001 a los tipos de documentos de DevTrail, permitiendo a los equipos construir documentación lista para cumplimiento como parte de su flujo de desarrollo.
>
> **Esto es una plantilla** — adapta cada sección al contexto de tu organización.

---

## 1. Alcance y Contexto (ISO 42001 Cláusula 4)

> Define los límites y el contexto de tu sistema de gestión de IA.

### 1.1 Contexto Organizacional

- **Organización**: [Nombre de la organización]
- **Industria / Sector**: [Sector]
- **Sistemas de IA en Alcance**: [Lista de sistemas de IA cubiertos por esta política]
- **Exclusiones**: [Sistemas o actividades explícitamente excluidos]

### 1.2 Partes Interesadas

| Parte | Necesidades y Expectativas | Requisitos Relevantes |
|-------|---------------------------|----------------------|
| Usuarios Finales | [Expectativas] | [Requisitos] |
| Reguladores | [Expectativas de cumplimiento] | [EU AI Act, GDPR, etc.] |
| Equipo de Desarrollo | [Expectativas] | [Requisitos] |
| Dirección | [Expectativas] | [Requisitos] |
| Titulares de Datos | [Expectativas de privacidad] | [Derechos GDPR] |

### 1.3 Requisitos Legales y Regulatorios

| Regulación | Aplicable | Estado | Evidencia DevTrail |
|-----------|-----------|--------|-------------------|
| EU AI Act | [Sí/No] | [Cumple/En progreso/Brecha] | ETH, MCARD |
| GDPR | [Sí/No] | [Estado] | ETH (Privacidad de Datos), DPIA |
| NIST AI RMF | [Sí/No] | [Estado] | AI-RISK-CATALOG |
| ISO/IEC 42001 | [Sí/No] | [Estado] | Este documento |
| [Otro] | [Sí/No] | [Estado] | [Documentos] |

> **Mapeo DevTrail**: Los documentos REQ capturan requisitos regulatorios. Los documentos ETH evalúan brechas de cumplimiento.

---

## 2. Liderazgo y Compromiso (ISO 42001 Cláusula 5)

> Define la política de IA, roles y compromiso del liderazgo.

### 2.1 Declaración de Política de IA

[Nombre de la organización] se compromete a:

- Desarrollar y desplegar sistemas de IA de manera responsable y ética
- Garantizar transparencia y explicabilidad en las decisiones asistidas por IA
- Proteger la privacidad y los derechos fundamentales de las personas afectadas
- Mantener supervisión humana de los sistemas de IA
- Mejorar continuamente las prácticas de gobernanza de IA

### 2.2 Roles y Responsabilidades

| Rol | Responsabilidades | Mapeo DevTrail |
|-----|-------------------|----------------|
| Líder de Gobernanza de IA | Gestión general del AIMS, mantenimiento de políticas | Este documento, MANAGEMENT-REVIEW-TEMPLATE |
| Equipo de Desarrollo | Documentación, implementación, pruebas | AILOG, AIDEC, TES |
| Revisor de Ética de IA | Revisión y aprobación ética | Aprobación ETH |
| Gestor de Riesgos | Identificación y evaluación de riesgos | AI-RISK-CATALOG, ETH |
| Delegado de Protección de Datos | Cumplimiento de privacidad, supervisión de DPIA | DPIA, ETH (Privacidad de Datos) |
| Agentes de IA | Documentación autónoma dentro de límites definidos | Según tabla de autonomía de AGENT-RULES.md |

### 2.3 Compromiso de la Dirección

- [ ] Política de IA aprobada y comunicada
- [ ] Roles y responsabilidades asignados
- [ ] Recursos asignados para gobernanza de IA
- [ ] Revisiones periódicas de la dirección programadas

> **Mapeo DevTrail**: Los documentos ADR registran decisiones de gobernanza. AGENT-RULES.md define los límites de autonomía de los agentes.

---

## 3. Planificación de Riesgos (ISO 42001 Cláusula 6)

> Identifica riesgos, establece objetivos y planifica cambios.

### 3.1 Identificación de Riesgos

| Categoría de Riesgo | Descripción | Probabilidad | Impacto | Controles Actuales | Evidencia DevTrail |
|---------------------|-------------|-------------|---------|-------------------|-------------------|
| Sesgo / Equidad | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | ETH (Sección de Sesgo) |
| Privacidad | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | ETH (Privacidad), DPIA |
| Seguridad | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | SEC |
| Seguridad Funcional | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | ETH, REQ (Seguridad Funcional) |
| Transparencia | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | ETH (Transparencia) |
| Ambiental | [Descripción] | [A/M/B] | [A/M/B] | [Controles] | ETH (Impacto Ambiental) |

> **Referencia**: Ver AI-RISK-CATALOG.md para el catálogo completo de riesgos mapeado a las categorías de NIST AI 600-1.
>
> **Mapeo DevTrail**: Los documentos ETH evalúan riesgos individuales. AI-RISK-CATALOG.md consolida el registro de riesgos organizacional según ISO 42001 Anexo A.5.

### 3.2 Objetivos de IA

| Objetivo | Meta | Medición | Plazo | Responsable |
|----------|------|----------|-------|-------------|
| Cobertura de documentación | [ej., 100% de cambios significativos documentados] | `devtrail metrics` | [Fecha] | [Responsable] |
| Cumplimiento de revisiones | [ej., Todos los docs de alto riesgo revisados en 5 días] | `devtrail metrics` | [Fecha] | [Responsable] |
| Cobertura de evaluación de riesgos | [ej., ETH para todos los cambios de alto riesgo] | `devtrail compliance` | [Fecha] | [Responsable] |

### 3.3 Planificación de Cambios

Cuando cambios significativos afectan al sistema de gestión de IA:

1. Documentar la decisión de cambio en un **ADR**
2. Evaluar riesgos en un documento **ETH**
3. Actualizar esta política si cambia el alcance de gobernanza
4. Comunicar cambios a todas las partes interesadas

---

## 4. Soporte y Recursos (ISO 42001 Cláusula 7)

> Define recursos, competencias y comunicación.

### 4.1 Recursos

| Recurso | Descripción | Estado |
|---------|-------------|--------|
| DevTrail Framework | Sistema de gobernanza de documentación | [Instalado/Versión] |
| DevTrail CLI | Herramientas de automatización y validación | [Versión] |
| Plataformas de Agentes IA | [Claude, Gemini, Copilot, Cursor] | [Configurado] |
| Formación | Formación en gobernanza de IA para el equipo | [Estado] |

### 4.2 Competencias

| Rol | Competencias Requeridas | Plan de Formación |
|-----|------------------------|-------------------|
| Desarrolladores | Uso de DevTrail, fundamentos de ética de IA, conciencia regulatoria | [Plan] |
| Agentes de IA | Cumplimiento de AGENT-RULES.md, uso de plantillas | [Configuración de directivas] |
| Revisores | Evaluación de riesgos, requisitos de EU AI Act, fundamentos de ISO 42001 | [Plan] |

### 4.3 Concienciación

Todos los miembros del equipo deben conocer:
- Esta Política de Gobernanza de IA
- AGENT-RULES.md y los requisitos de documentación
- PRINCIPLES.md y las directrices éticas
- Su rol en el sistema de gestión de IA

### 4.4 Comunicación

| Qué | A Quién | Cuándo | Método | Registro DevTrail |
|-----|---------|--------|--------|-------------------|
| Actualizaciones de política | Todo el equipo | Al cambiar | [Método] | ADR |
| Evaluaciones de riesgo | Revisores | Por creación de ETH | Notificación DevTrail | ETH |
| Informes de incidentes | Dirección | En 24h | [Método] | INC |
| Métricas de gobernanza | Dirección | [Mensual/Trimestral] | `devtrail metrics` | Informe de métricas |

### 4.5 Información Documentada

DevTrail sirve como sistema de información documentada para el AIMS. Documentos clave:

| Requisito ISO 42001 | Documento DevTrail |
|---------------------|-------------------|
| Política de IA | Este documento (§2) |
| Evaluación de Riesgos | ETH, AI-RISK-CATALOG.md |
| Procedimientos operativos | AGENT-RULES.md, DOCUMENTATION-POLICY.md |
| Registros de cambios | AILOG (todos los cambios) |
| Registros de decisiones | AIDEC, ADR |

---

## 5. Operaciones del Ciclo de Vida de IA (ISO 42001 Cláusula 8)

> Define cómo se gestionan los sistemas de IA a lo largo de su ciclo de vida.

### 5.1 Fases del Ciclo de Vida

| Fase | Actividades | Documentos DevTrail | Control ISO 42001 |
|------|------------|--------------------|--------------------|
| Diseño | Requisitos, decisiones de arquitectura | REQ, ADR, AIDEC | A.6.2.2 |
| Desarrollo | Implementación, cambios de código | AILOG, AIDEC | A.6.2.2, A.6.2.9 |
| Pruebas | Validación, verificación | TES | A.6.2.3, A.6.2.4 |
| Despliegue | Lanzamiento, configuración | AILOG | A.6.2.5 |
| Monitorización | Operaciones, observabilidad | AILOG, INC | A.6.2.6 |
| Retirada | Desmantelamiento | ADR, AILOG | A.6.2.7 |

> **Referencia**: Ver AI-LIFECYCLE-TRACKER.md para el seguimiento del estado del ciclo de vida del sistema.

### 5.2 Requisitos de Documentación

Según AGENT-RULES.md, la documentación es requerida cuando:

- Cambios que afectan auth/autorización/PII → AILOG + borrador ETH
- Cambios en API pública o esquema de BD → AILOG
- Cambios en modelos ML o prompts de IA → AILOG + revisión humana
- Código sobre umbral de complejidad cognitiva (ejecutar `devtrail analyze`; alternativa: >20 líneas) → AILOG
- Decisión entre 2+ alternativas → AIDEC
- Cambios en dependencias críticas de seguridad → AILOG + revisión humana

### 5.3 Componentes de IA de Terceros

| Componente | Proveedor | Propósito | Nivel de Riesgo | Última Revisión |
|-----------|----------|----------|-----------------|----------------|
| [Componente] | [Proveedor] | [Propósito] | [A/M/B] | [Fecha] |

> **Mapeo DevTrail**: SBOM documenta la cadena de suministro de IA. ETH evalúa riesgos de terceros.

---

## 6. Evaluación del Desempeño (ISO 42001 Cláusula 9)

> Define cómo se monitoriza y revisa el desempeño de gobernanza.

### 6.1 Monitorización y Medición

| KPI | Meta | Método de Medición | Frecuencia |
|-----|------|-------------------|-----------|
| Cobertura de documentación | [Meta %] | `devtrail metrics` | [Frecuencia] |
| Tasa de finalización de revisiones | [Meta %] | `devtrail metrics` | [Frecuencia] |
| Tiempo medio de documentación | [Meta días] | `devtrail metrics` | [Frecuencia] |
| Tiempo de revisión de docs de alto riesgo | [Meta días] | Seguimiento manual | [Frecuencia] |
| Documentación de incidentes | [Meta %] | Conteo de INC vs incidentes | [Frecuencia] |

> **Referencia**: Ver AI-KPIS.md para definiciones detalladas de KPIs.

### 6.2 Auditoría Interna

- **Frecuencia**: [ej., Trimestral]
- **Alcance**: Cumplimiento con esta política, AGENT-RULES.md y requisitos regulatorios
- **Método**: `devtrail compliance --all` + revisión manual
- **Auditor**: [Rol/Nombre]

### 6.3 Revisión de la Dirección

- **Frecuencia**: [ej., Trimestral]
- **Entradas**: Métricas de gobernanza, resultados de auditoría, informes de incidentes, evaluaciones de riesgos
- **Salidas**: Actualizaciones de política, decisiones de recursos, acciones de mejora

> **Referencia**: Ver MANAGEMENT-REVIEW-TEMPLATE.md para la plantilla de agenda de revisión.

---

## 7. Mejora Continua (ISO 42001 Cláusula 10)

> Define cómo se gestionan las no conformidades y se impulsan las mejoras.

### 7.1 No Conformidad y Acción Correctiva

Cuando se identifica una no conformidad:

1. **Documentar** la no conformidad (INC o AILOG con `risk_level: high`)
2. **Evaluar** causa raíz e impacto
3. **Implementar** acción correctiva
4. **Verificar** efectividad
5. **Actualizar** documentos de gobernanza si es necesario (ADR para cambios de política)

### 7.2 Fuentes de Mejora

| Fuente | Entrada DevTrail | Acción |
|--------|-----------------|--------|
| Fallos de validación | Errores de `devtrail validate` | Corregir y documentar |
| Brechas de cumplimiento | Informe de `devtrail compliance` | Planificar remediación |
| Post-mortems de incidentes | Documentos INC | Implementar acciones correctivas |
| Revisiones de la dirección | Salidas de reuniones de revisión | Actualizar política/objetivos |
| Retroalimentación de agentes | AILOG con sugerencias | Evaluar y priorizar |
| Cambios regulatorios | Monitorización externa | Actualizar plantillas y política |

---

## Anexo: Controles del Anexo A de ISO 42001 → Mapeo DevTrail

> Este mapeo permite a los equipos demostrar la cobertura de controles del Anexo A a través de la documentación de DevTrail.

| Área Temática | Control | ID | Documento(s) DevTrail |
|--------------|---------|-----|----------------------|
| **A.2 Políticas para IA** | Política de IA | A.2.2 | Este documento §2 |
| | Temas de IA Responsable | A.2.3 | Este documento §2, PRINCIPLES.md |
| **A.3 Organización Interna** | Roles y Responsabilidades | A.3.2 | Este documento §2, AGENT-RULES.md |
| | Reporte de Preocupaciones de IA | A.3.3 | INC, ETH |
| | Impacto de Cambios Organizacionales | A.3.4 | ADR |
| **A.4 Recursos** | Recursos | A.4.2 | Este documento §4 |
| | Competencias | A.4.3 | Este documento §4 |
| | Concienciación sobre Uso Responsable | A.4.4 | PRINCIPLES.md, directivas de agentes |
| | Consulta | A.4.5 | MANAGEMENT-REVIEW-TEMPLATE |
| | Comunicación sobre Sistema de IA | A.4.6 | ADR, REQ |
| **A.5 Evaluación de Impactos** | Evaluación de Riesgos | A.5.2 | ETH, AI-RISK-CATALOG |
| | Evaluación de Impacto | A.5.3 | ETH, DPIA |
| | Documentación de Impacto | A.5.4 | ETH, DPIA |
| **A.6 Ciclo de Vida del Sistema de IA** | Diseño y Desarrollo | A.6.2.2 | ADR, AIDEC |
| | Entrenamiento y Pruebas del Modelo de IA | A.6.2.3 | MCARD, TES |
| | Verificación y Validación | A.6.2.4 | TES |
| | Despliegue | A.6.2.5 | AILOG, AI-LIFECYCLE-TRACKER |
| | Operación y Monitorización | A.6.2.6 | AILOG, AI-LIFECYCLE-TRACKER, OBSERVABILITY-GUIDE |
| | Retirada | A.6.2.7 | AI-LIFECYCLE-TRACKER, ADR |
| | Integración Responsable | A.6.2.8 | ADR, AIDEC |
| | Documentación | A.6.2.9 | AILOG (todos los cambios documentados) |
| | Uso Definido y Mal Uso | A.6.2.10 | MCARD |
| | Componentes de Terceros | A.6.2.11 | SBOM |
| **A.7 Datos para IA** | Datos para Desarrollo | A.7.2 | MCARD |
| | Calidad de Datos para ML | A.7.3 | MCARD |
| | Preparación de Datos | A.7.4 | MCARD |
| | Adquisición de Datos | A.7.5 | SBOM, DPIA |
| | Procedencia de Datos | A.7.6 | SBOM |
| **A.8 Información para las Partes** | Transparencia en Interacción con IA | A.8.2 | ETH (Transparencia) |
| | Información sobre Resultados de IA | A.8.3 | ETH (Explicabilidad) |
| | Acceso a Información | A.8.4 | REQ, ADR |
| | Habilitación de Acciones Humanas | A.8.5 | AGENT-RULES.md (disparadores de revisión humana) |
| **A.9 Uso de Sistemas de IA** | Objetivos para Uso Responsable | A.9.2 | Este documento §5, PRINCIPLES.md |
| | Uso Previsto | A.9.3 | MCARD, REQ |
| | Procesos para Uso Responsable | A.9.4 | DOCUMENTATION-POLICY.md, AGENT-RULES.md |
| | Supervisión Humana | A.9.5 | AGENT-RULES.md (tabla de autonomía) |
| **A.10 Terceros** | Proveedores de Componentes de IA | A.10.2 | SBOM |
| | Modelos ML Compartidos | A.10.3 | SBOM |
| | Provisión a Terceros | A.10.4 | ETH, MCARD |

---

*Plantilla de Política de Gobernanza de IA — DevTrail Framework*
*Alineado con ISO/IEC 42001:2023*

<!-- Template: DevTrail | https://strangedays.tech -->
