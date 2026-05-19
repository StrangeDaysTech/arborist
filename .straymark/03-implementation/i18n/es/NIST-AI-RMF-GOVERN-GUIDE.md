# NIST AI RMF --- Guía de Implementación de la Función GOVERN

> **Marco de referencia**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Función**: GOVERN --- Estructuras de gobernanza organizacional para IA
>
> La función GOVERN establece y mantiene las estructuras organizacionales, políticas, procesos y cultura necesarios para la gestión responsable de riesgos de IA. Es una función transversal que informa y se nutre de las funciones MAP, MEASURE y MANAGE.

---

## GV-1: Políticas para la Gobernanza de IA

Establecer, documentar y comunicar políticas organizacionales que definan las expectativas para el desarrollo, despliegue y uso de IA. Las políticas deben ser documentos vivos, revisados y actualizados regularmente.

> Las políticas establecen la línea base organizacional. Sin políticas explícitas, los equipos recurren a juicios individuales inconsistentes.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Política de gobernanza de IA | AI-GOVERNANCE-POLICY.md | Documento completo |
| Estándares de documentación | DOCUMENTATION-POLICY.md | Documento completo |
| Límites de autonomía de agentes | AGENT-RULES.md | Autonomy Table |
| Principios éticos | PRINCIPLES.md | Documento completo |
| Política de cumplimiento regulatorio | AI-GOVERNANCE-POLICY.md | Sección 1.3 (Legal and Regulatory Requirements) |

### Lista de Verificación de Implementación

- [ ] Personalizar AI-GOVERNANCE-POLICY.md para el contexto, alcance y entorno regulatorio de su organización
- [ ] Adoptar DOCUMENTATION-POLICY.md como el estándar para toda la documentación relacionada con IA
- [ ] Configurar AGENT-RULES.md para reflejar la tolerancia al riesgo de su organización respecto a la autonomía de agentes de IA
- [ ] Revisar y actualizar todas las políticas de gobernanza al menos anualmente o cuando ocurran cambios significativos

---

## GV-2: Responsabilidad y Roles

Definir roles, responsabilidades y estructuras de rendición de cuentas claras para la gestión de riesgos de IA. Asegurar que cada función de gobernanza tenga un responsable asignado.

> La responsabilidad sin asignación es una ilusión. Cada actividad de gestión de riesgos necesita una parte responsable designada.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Roles y responsabilidades | AI-GOVERNANCE-POLICY.md | Sección 2.2 (Roles and Responsibilities) |
| Límites de autonomía de agentes | AGENT-RULES.md | Autonomy Table, Human Review Triggers |
| Autoridad de decisión | ADR | sección Decision Makers |
| Responsabilidades de revisión | DOCUMENTATION-POLICY.md | sección Review Requirements |
| Responsabilidad ante incidentes | INC | campos Owner, Responders |

### Lista de Verificación de Implementación

- [ ] Completar la tabla de Roles y Responsabilidades en la Sección 2.2 de AI-GOVERNANCE-POLICY.md
- [ ] Definir los disparadores de revisión humana en AGENT-RULES.md para todas las actividades de alto riesgo
- [ ] Asegurar que cada ADR identifique a los tomadores de decisiones y su autoridad
- [ ] Asignar roles de respuesta a incidentes y documentarlos en la plantilla INC

---

## GV-3: Diversidad e Inclusión de la Fuerza Laboral

Asegurar que los equipos de desarrollo y gobernanza de IA incluyan perspectivas diversas. Los equipos diversos son mejores para identificar riesgos, sesgos y puntos ciegos.

> Los equipos homogéneos producen evaluaciones de riesgos homogéneas. La diversidad de perspectivas es un control de gobernanza, no solo un objetivo de recursos humanos.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Guía de composición de equipos | AI-GOVERNANCE-POLICY.md | Sección 4 (Support and Resources) |
| Requisitos de competencia | AI-GOVERNANCE-POLICY.md | Sección 4.2 (Competencies) |
| Procesos de revisión inclusivos | DOCUMENTATION-POLICY.md | sección Review Requirements |
| Representación de partes interesadas | ETH | sección Stakeholder Analysis |

### Lista de Verificación de Implementación

- [ ] Documentar las expectativas de composición de equipos en la Sección 4 de AI-GOVERNANCE-POLICY.md
- [ ] Definir requisitos de competencia que abarquen experiencia técnica, ética, legal y de dominio
- [ ] Asegurar que el Stakeholder Analysis del ETH considere las perspectivas de las comunidades afectadas
- [ ] Incluir revisores diversos en los flujos de revisión de documentos de alto riesgo

---

## GV-4: Cultura Organizacional

Fomentar una cultura organizacional que valore el desarrollo responsable de IA, incentive plantear preocupaciones y apoye el aprendizaje continuo sobre riesgos y ética de la IA.

> La cultura determina si las políticas se siguen en la práctica o existen solo en papel. La gobernanza es tan fuerte como la cultura que la sostiene.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Principios organizacionales | PRINCIPLES.md | Documento completo |
| Directrices éticas | PRINCIPLES.md | sección Core Principles |
| Expectativas de transparencia | AGENT-RULES.md | Requisitos de documentación |
| Aprendizaje de incidentes | INC | sección Lessons Learned |

### Lista de Verificación de Implementación

- [ ] Adoptar y comunicar PRINCIPLES.md como la base ética compartida del equipo
- [ ] Establecer una cultura sin culpabilización para reportar preocupaciones de IA, tratando los documentos INC como herramientas de aprendizaje
- [ ] Integrar la conciencia sobre ética de IA en la incorporación de nuevos miembros usando PRINCIPLES.md y AI-GOVERNANCE-POLICY.md
- [ ] Celebrar y compartir ejemplos de prácticas responsables de IA documentadas en registros AILOG y ETH

---

## GV-5: Participación de las Partes Interesadas

Involucrar a las partes interesadas internas y externas regularmente en las actividades de gobernanza de IA. La participación de las partes interesadas mejora la identificación de riesgos, genera confianza y asegura que la gobernanza refleje necesidades diversas.

> La gobernanza realizada de forma aislada carece de la perspectiva externa necesaria para anticipar los impactos del mundo real.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Revisiones de gestión | MANAGEMENT-REVIEW-TEMPLATE.md | Documento completo |
| Retroalimentación de partes interesadas | MANAGEMENT-REVIEW-TEMPLATE.md | sección Feedback Summary |
| Comunicación externa | AI-GOVERNANCE-POLICY.md | Sección 4.4 (Communication) |
| Transparencia pública | MCARD | Documento completo (documentación de modelos orientada al público) |
| Resultados de revisiones | MANAGEMENT-REVIEW-TEMPLATE.md | sección Action Items |

### Lista de Verificación de Implementación

- [ ] Programar revisiones periódicas de gestión usando MANAGEMENT-REVIEW-TEMPLATE.md
- [ ] Incluir la retroalimentación de partes interesadas como punto fijo en la agenda de revisiones de gestión
- [ ] Publicar documentos MCARD para sistemas de IA desplegados externamente para apoyar la transparencia pública
- [ ] Documentar las actividades de participación de partes interesadas y sus resultados como evidencia de auditoría

---

## GV-6: Gobernanza de la Cadena de Suministro de IA

Gobernar la cadena de suministro de IA manteniendo transparencia sobre componentes de terceros, modelos, fuentes de datos y servicios utilizados en los sistemas de IA.

> No se puede gestionar lo que no se puede ver. La gobernanza de la cadena de suministro comienza con un inventario completo y actualizado.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Inventario de componentes | SBOM | Documento completo |
| Servicios de terceros | SBOM | sección Third-Party Services |
| Cumplimiento de licencias | SBOM | columna License |
| Evaluación de riesgos de proveedores | ETH | sección Third-Party Risk |
| Políticas de cadena de suministro | AI-GOVERNANCE-POLICY.md | Sección 5.3 (Third-Party AI Components) |

### Lista de Verificación de Implementación

- [ ] Mantener un SBOM actualizado para cada sistema de IA, incluyendo modelos, bibliotecas, APIs y fuentes de datos
- [ ] Rastrear los términos de licencia de todos los componentes de terceros en la columna License del SBOM
- [ ] Realizar evaluaciones ETH para proveedores críticos cuyos componentes afecten funciones de alto riesgo
- [ ] Definir requisitos de gobernanza de la cadena de suministro en la Sección 5.3 de AI-GOVERNANCE-POLICY.md

---

## Resumen: Mapeo de la Función GOVERN a DevTrail

| Categoría | Descripción | Documento DevTrail Principal | Campos / Secciones Clave |
|----------|-------------|---------------------------|----------------------|
| GV-1 | Políticas | AI-GOVERNANCE-POLICY.md, DOCUMENTATION-POLICY.md | Documentos completos |
| GV-2 | Responsabilidad | AGENT-RULES.md, AI-GOVERNANCE-POLICY.md | Autonomy Table, Sección 2.2 |
| GV-3 | Diversidad de la Fuerza Laboral | AI-GOVERNANCE-POLICY.md | Sección 4 |
| GV-4 | Cultura Organizacional | PRINCIPLES.md | Core Principles |
| GV-5 | Participación de Partes Interesadas | MANAGEMENT-REVIEW-TEMPLATE.md | Feedback Summary, Action Items |
| GV-6 | Cadena de Suministro | SBOM | Documento completo |

---

*Guía de Implementación de la Función GOVERN del NIST AI RMF --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
