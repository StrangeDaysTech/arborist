# Catálogo de Riesgos de IA

> **Alineado con**: NIST AI 600-1 e ISO/IEC 42001:2023 Anexo C
>
> Este documento proporciona un registro centralizado de riesgos para los sistemas de IA gestionados bajo DevTrail. Mapea las entradas de riesgo a las 12 categorías de riesgo de NIST AI 600-1 y se alinea con ISO 42001 Anexo A.5 (Evaluación de Impactos de Sistemas de IA) e ISO/IEC 23894 (Gestión de Riesgos de IA).
>
> **Esta es una plantilla** — complete con los riesgos específicos de los sistemas de IA de su organización.

---

## 1. Propósito

Este catálogo de riesgos sirve como la fuente única de verdad para los riesgos relacionados con IA en toda la organización. Permite:

- **Seguimiento centralizado** de todos los riesgos de IA identificados
- **Evaluación estandarizada** utilizando escalas consistentes de probabilidad e impacto
- **Mapeo regulatorio** a las categorías de NIST AI 600-1 y controles de ISO 42001
- **Monitoreo continuo** mediante ciclos de revisión periódicos

> **Mapeo DevTrail**: Los documentos ETH evalúan riesgos individuales a nivel de cambio. Este catálogo consolida los riesgos organizacionales según ISO 42001 Anexo A.5.

---

## 2. Metodología de Evaluación de Riesgos

### 2.1 Escalas

| Puntuación | Probabilidad | Impacto |
|-------|-----------|--------|
| 1 | Rara — improbable que ocurra | Insignificante — efecto mínimo en las operaciones |
| 2 | Improbable — podría ocurrir en circunstancias excepcionales | Menor — efecto limitado, fácilmente gestionable |
| 3 | Posible — podría ocurrir en algún momento | Moderado — efecto notable, requiere respuesta |
| 4 | Probable — probablemente ocurrirá en la mayoría de circunstancias | Mayor — efecto significativo en operaciones o individuos |
| 5 | Casi Seguro — se espera que ocurra regularmente | Severo — efecto crítico, posible acción regulatoria |

### 2.2 Cálculo de la Puntuación de Riesgo

**Puntuación de Riesgo = Probabilidad x Impacto**

| Puntuación de Riesgo | Nivel de Riesgo | Acción Requerida |
|-----------|-----------|-----------------|
| 1–4 | Bajo | Aceptar o monitorear; documentar en el próximo ciclo de revisión |
| 5–9 | Medio | Implementar controles adicionales; revisar trimestralmente |
| 10–15 | Alto | Implementar controles urgentemente; revisar mensualmente |
| 16–25 | Crítico | Acción inmediata requerida; escalar a la dirección |

### 2.3 Fases de Gestión de Riesgos (ISO/IEC 23894)

| Fase | Descripción | Referencia ISO 23894 | Evidencia DevTrail |
|-------|-------------|--------------------|--------------------|
| Identificación | Descubrir y describir los riesgos de IA | Cláusula 6.1 | Documentos ETH, este catálogo |
| Evaluación | Evaluar probabilidad, impacto y puntuación de riesgo | Cláusula 6.2 | Puntuación de Riesgo en este catálogo |
| Tratamiento | Seleccionar e implementar controles para mitigar riesgos | Cláusula 6.3 | Columna Controles Actuales, ADR para decisiones |
| Monitoreo | Seguir el riesgo residual y la efectividad de los controles | Cláusula 6.4 | Fecha de Revisión, `devtrail metrics` |

---

## 3. Registro de Riesgos

### RISK-001 — Sesgo en Modelo de Clasificación

| Campo | Valor |
|-------|-------|
| **ID de Riesgo** | RISK-001 |
| **Categoría** | Sesgo (NIST: `bias`) |
| **Descripción** | El modelo de clasificación produce resultados sistemáticamente injustos para grupos demográficos protegidos debido a datos de entrenamiento desequilibrados o selección sesgada de características. |
| **Probabilidad** | 4 |
| **Impacto** | 4 |
| **Puntuación de Riesgo** | 16 (Crítico) |
| **Controles Actuales** | Métricas de equidad evaluadas durante el entrenamiento; auditoría de sesgo en documentos ETH; verificaciones de paridad demográfica en el conjunto de pruebas. |
| **Riesgo Residual** | Medio — los controles reducen pero no eliminan el sesgo en casos límite. |
| **Responsable** | [Revisor de Ética de IA] |
| **Fecha de Revisión** | [YYYY-MM-DD] |

> **Mapeo ISO 42001**: A.5.2 (Evaluación de Riesgos), A.6.2.3 (Entrenamiento y Pruebas), A.7.3 (Calidad de Datos para ML)

---

### RISK-002 — Filtración de Datos de Entrenamiento

| Campo | Valor |
|-------|-------|
| **ID de Riesgo** | RISK-002 |
| **Categoría** | Privacidad (NIST: `privacy`) |
| **Descripción** | Información de identificación personal (PII) o datos propietarios incluidos en los conjuntos de datos de entrenamiento se exponen a través de las salidas del modelo, memorización o ataques de extracción. |
| **Probabilidad** | 3 |
| **Impacto** | 5 |
| **Puntuación de Riesgo** | 15 (Alto) |
| **Controles Actuales** | Pipeline de anonimización de datos; escaneo de detección de PII antes del entrenamiento; controles de acceso sobre datos de entrenamiento; DPIA completada. |
| **Riesgo Residual** | Medio — los ataques de extracción en modelos grandes siguen siendo una amenaza en evolución. |
| **Responsable** | [Delegado de Protección de Datos] |
| **Fecha de Revisión** | [YYYY-MM-DD] |

> **Mapeo ISO 42001**: A.5.3 (Evaluación de Impacto), A.7.2 (Datos para Desarrollo), A.7.5 (Adquisición de Datos)

---

### RISK-003 — Alucinación en Generador de Texto

| Campo | Valor |
|-------|-------|
| **ID de Riesgo** | RISK-003 |
| **Categoría** | Confabulación (NIST: `confabulation`) |
| **Descripción** | El generador de texto basado en LLM produce contenido plausible pero factualmente incorrecto que se presenta a usuarios o sistemas posteriores sin verificación adecuada. |
| **Probabilidad** | 4 |
| **Impacto** | 3 |
| **Puntuación de Riesgo** | 12 (Alto) |
| **Controles Actuales** | Revisión humana requerida para todo el contenido generado en producción; umbrales de confianza configurados; generación aumentada por recuperación (RAG) con citas de fuentes. |
| **Riesgo Residual** | Medio — las alucinaciones aún pueden pasar la revisión humana en dominios especializados. |
| **Responsable** | [Líder del Equipo de Desarrollo] |
| **Fecha de Revisión** | [YYYY-MM-DD] |

> **Mapeo ISO 42001**: A.6.2.4 (Verificación y Validación), A.8.3 (Información de Resultados de IA), A.9.5 (Supervisión Humana)

---

### RISK-004 — Dependencia de la Cadena de Suministro de IA

| Campo | Valor |
|-------|-------|
| **ID de Riesgo** | RISK-004 |
| **Categoría** | Cadena de Valor (NIST: `value_chain`) |
| **Descripción** | Dependencia crítica de un proveedor externo de modelos de IA (API, pesos o infraestructura) que genera riesgo de interrupción del servicio, cambios no anunciados en el modelo o vulnerabilidades heredadas. |
| **Probabilidad** | 3 |
| **Impacto** | 4 |
| **Puntuación de Riesgo** | 12 (Alto) |
| **Controles Actuales** | SBOM mantenido para componentes de IA; monitoreo de SLA del proveedor; proveedor de respaldo configurado; fijación de versión del modelo. |
| **Riesgo Residual** | Medio — la dependencia del proveedor limita las opciones de respaldo para algunas capacidades. |
| **Responsable** | [Líder de Gobernanza de IA] |
| **Fecha de Revisión** | [YYYY-MM-DD] |

> **Mapeo ISO 42001**: A.6.2.11 (Componentes de Terceros), A.10.2 (Proveedores de Componentes de IA), A.10.3 (Modelos de ML Compartidos)

---

### [RISK-NNN] — [Título del Riesgo]

| Campo | Valor |
|-------|-------|
| **ID de Riesgo** | RISK-NNN |
| **Categoría** | [Categoría] (NIST: `[nist_category]`) |
| **Descripción** | [Descripción del riesgo] |
| **Probabilidad** | [1-5] |
| **Impacto** | [1-5] |
| **Puntuación de Riesgo** | [Calculada] |
| **Controles Actuales** | [Controles implementados] |
| **Riesgo Residual** | [Nivel de riesgo residual y justificación] |
| **Responsable** | [Responsable] |
| **Fecha de Revisión** | [YYYY-MM-DD] |

---

## 4. Categorías de Riesgo NIST AI 600-1

> Tabla de referencia de las 12 categorías de riesgo de NIST AI 600-1. Utilice el **ID de Categoría** al clasificar riesgos en el registro anterior.

| ID de Categoría | Nombre de Categoría | Descripción |
|------------|--------------|-------------|
| `cbrn` | Información QBRN | Riesgos relacionados con la IA que facilita el acceso a información sobre armas químicas, biológicas, radiológicas o nucleares. |
| `confabulation` | Confabulación | Riesgos derivados de que la IA genere información plausible pero factualmente incorrecta o fabricada. |
| `dangerous_content` | Contenido Peligroso | Riesgos derivados de que la IA genere contenido que pueda causar daño físico o facilitar actividades peligrosas. |
| `privacy` | Privacidad de Datos | Riesgos para la privacidad individual a través de la recopilación, inferencia, memorización o divulgación no autorizada de datos. |
| `environmental` | Impacto Ambiental | Riesgos derivados de los costos ambientales del entrenamiento y operación de IA, incluyendo consumo energético y emisiones de carbono. |
| `bias` | Sesgo Perjudicial y Homogeneización | Riesgos de discriminación sistemática o reducción de la diversidad en las salidas y decisiones de IA. |
| `human_ai_config` | Configuración Humano-IA | Riesgos derivados de niveles inadecuados de supervisión humana o dependencia excesiva de las salidas del sistema de IA. |
| `information_integrity` | Integridad de la Información | Riesgos para la integridad de los ecosistemas de información a través de desinformación o manipulación generada por IA. |
| `information_security` | Seguridad de la Información | Riesgos derivados de que los sistemas de IA sean explotados mediante ataques adversarios, inyección de prompts o robo de modelos. |
| `intellectual_property` | Propiedad Intelectual | Riesgos relacionados con que los sistemas de IA infrinjan derechos de autor, patentes o secretos comerciales en el entrenamiento o la salida. |
| `obscene_content` | Contenido Obsceno, Degradante o Abusivo | Riesgos derivados de que la IA genere contenido sexualmente explícito, degradante o abusivo. |
| `value_chain` | Cadena de Valor e Integración de Componentes | Riesgos de dependencias en componentes, modelos o servicios de IA de terceros en la cadena de suministro de IA. |

---

## 5. Mapeo ISO 42001 Anexo A.5

> Este catálogo cumple con los siguientes requisitos del Anexo A.5 de ISO 42001.

| Control | Requisito | Cómo Este Catálogo lo Aborda |
|---------|------------|-------------------------------|
| A.5.2 | Evaluación de riesgos de IA | Registro de riesgos con metodología de puntuación estandarizada |
| A.5.3 | Evaluación de impactos de los sistemas de IA en individuos | La columna de Impacto captura efectos en individuos y grupos |
| A.5.4 | Documentación y reporte de evaluaciones de impacto de sistemas de IA | Cada entrada de riesgo documenta los resultados de la evaluación con fechas de revisión |

> **Referencias cruzadas**: Los documentos ETH individuales proporcionan evaluaciones de riesgo a nivel de cambio. Los documentos DPIA abordan los impactos en la protección de datos. Este catálogo proporciona la vista consolidada a nivel organizacional.

---

## 6. Calendario de Revisiones

| Tipo de Revisión | Frecuencia | Próxima Revisión | Responsable |
|------------|-----------|-------------|-------------|
| Revisión completa del catálogo | Trimestral | [YYYY-MM-DD] | [Líder de Gobernanza de IA] |
| Revisión de riesgos Altos/Críticos | Mensual | [YYYY-MM-DD] | [Gestor de Riesgos] |
| Actualización de riesgo post-incidente | Después de cada INC | Según necesidad | [Responsable del Incidente] |
| Incorporación de nuevo sistema | Según AI-LIFECYCLE-TRACKER | Según necesidad | [Responsable del Sistema] |

---

*Plantilla del Catálogo de Riesgos de IA — DevTrail Framework*
*Alineado con NIST AI 600-1 e ISO/IEC 42001:2023 Anexo C*

<!-- Template: DevTrail | https://strangedays.tech -->
