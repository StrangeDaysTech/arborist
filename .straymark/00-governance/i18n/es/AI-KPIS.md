# KPIs de Gobernanza de IA

> **Alineado con**: ISO/IEC 42001:2023 Cláusula 9 — Evaluación del Desempeño
>
> Este documento define indicadores clave de rendimiento (KPIs) para medir la efectividad del sistema de gestión de IA. Estas métricas proporcionan evidencia objetiva para las revisiones por la dirección (Cláusula 9.3) y auditorías internas (Cláusula 9.2), y apoyan la mejora continua (Cláusula 10).
>
> **Esta es una plantilla** — ajuste los objetivos y métodos de medición según la madurez y el contexto de su organización.

---

## 1. Propósito

Los KPIs cumplen tres funciones dentro del sistema de gestión de IA:

- **Monitoreo**: Rastrear la salud de la gobernanza y detectar degradaciones tempranamente
- **Rendición de cuentas**: Proporcionar evidencia objetiva de cumplimiento para auditorías y revisiones
- **Mejora**: Identificar tendencias y áreas que requieren acciones correctivas

> **Mapeo DevTrail**: Utilice `devtrail metrics` para la recopilación automatizada de KPIs. Los resultados alimentan MANAGEMENT-REVIEW-TEMPLATE.md para las revisiones periódicas.

---

## 2. Definiciones de KPIs

### KPI-01: Cobertura de Documentación

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Cobertura de Documentación |
| **Descripción** | Porcentaje de cambios relacionados con IA que tienen documentación DevTrail asociada (AILOG, AIDEC, ETH, etc.). |
| **Objetivo** | > 80% |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | `devtrail metrics` — ratio de cambios documentados respecto al total de commits relacionados con IA |
| **Frecuencia** | Mensual |
| **Responsable** | [Líder del Equipo de Desarrollo] |
| **Referencia ISO 42001** | Cláusula 9.1 (Seguimiento, medición, análisis y evaluación) |

> **Guía**: Comience con una línea base realista. Los equipos nuevos en DevTrail pueden establecer un objetivo inicial del 50% e incrementar un 10% por trimestre. Los cambios que requieren documentación se definen en AGENT-RULES.md.

---

### KPI-02: Tasa de Cumplimiento de Revisiones

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Tasa de Cumplimiento de Revisiones |
| **Descripción** | Porcentaje de documentos que requieren revisión humana que fueron revisados dentro del plazo definido. |
| **Objetivo** | 100% |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | `devtrail metrics` — ratio de documentos revisados respecto a documentos marcados para revisión |
| **Frecuencia** | Mensual |
| **Responsable** | [Revisor de Ética de IA] |
| **Referencia ISO 42001** | Cláusula 9.1 (Seguimiento, medición, análisis y evaluación) |

> **Guía**: Los documentos que requieren revisión incluyen: documentos ETH, entradas AILOG con `risk_level: high` o `risk_level: critical`, y cambios que afectan autenticación/PII según AGENT-RULES.md.

---

### KPI-03: Tiempo Medio de Documentación

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Tiempo Medio de Documentación |
| **Descripción** | Número promedio de días entre un cambio de código y la creación de su entrada AILOG correspondiente. |
| **Objetivo** | < 1 día |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | Comparar marcas de tiempo de commits en git con fechas de creación de AILOG en los encabezados de documentos |
| **Frecuencia** | Mensual |
| **Responsable** | [Líder del Equipo de Desarrollo] |
| **Referencia ISO 42001** | Cláusula 9.1 (Seguimiento, medición, análisis y evaluación) |

> **Guía**: Los agentes de IA deben crear documentación en la misma sesión que el cambio, con objetivo de 0 días. La documentación creada por humanos no debe exceder 2 días hábiles.

---

### KPI-04: Distribución de Riesgos

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Distribución de Riesgos |
| **Descripción** | Ratio de entradas de riesgo alto y crítico respecto al total de entradas en el AI-RISK-CATALOG. |
| **Objetivo** | < 20% alto/crítico |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | `devtrail metrics` — conteo de riesgos con puntuación >= 10 dividido por el total de riesgos |
| **Frecuencia** | Trimestral |
| **Responsable** | [Gestor de Riesgos] |
| **Referencia ISO 42001** | Cláusula 6 (Planificación — Acciones para abordar riesgos y oportunidades) |

> **Guía**: Un alto ratio de riesgos críticos indica controles inmaduros o una identificación agresiva de riesgos. En ambos casos, requiere atención de la dirección. Rastree la tendencia a lo largo del tiempo en lugar del valor absoluto.

---

### KPI-05: Tiempo de Respuesta a Incidentes

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Tiempo de Respuesta a Incidentes |
| **Descripción** | Tiempo transcurrido desde la detección del incidente hasta la creación de un documento INC correspondiente en DevTrail. |
| **Objetivo** | < 24 horas |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | Comparar marca de tiempo de detección del incidente (desde monitoreo/alertas) con fecha de creación del documento INC |
| **Frecuencia** | Por incidente (agregado trimestralmente) |
| **Responsable** | [Líder de Operaciones] |
| **Referencia ISO 42001** | Cláusula 10.1 (No conformidad y acción correctiva) |

> **Guía**: Esto mide la respuesta de documentación, no la resolución. Una documentación más rápida permite un análisis de causa raíz más rápido y previene la pérdida de conocimiento. El objetivo debe ser más estricto para incidentes de alta severidad.

---

### KPI-06: Puntuación de Cumplimiento

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | Puntuación de Cumplimiento |
| **Descripción** | Porcentaje general de cumplimiento regulatorio medido por el motor de validación de cumplimiento de DevTrail. |
| **Objetivo** | > 75% |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | `devtrail compliance --all` — porcentaje de reglas aprobadas en todas las regulaciones aplicables |
| **Frecuencia** | Trimestral |
| **Responsable** | [Líder de Gobernanza de IA] |
| **Referencia ISO 42001** | Cláusula 9.2 (Auditoría interna) |

> **Guía**: El motor de cumplimiento verifica la completitud de la documentación, campos requeridos, estado de revisión y mapeos regulatorios. Una puntuación por debajo del 50% indica brechas significativas que requieren planificación de remediación inmediata.

---

## 3. Panel Resumen de KPIs

| KPI | Objetivo | Actual | Tendencia | Estado |
|-----|--------|---------|-------|--------|
| Cobertura de Documentación | > 80% | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tasa de Cumplimiento de Revisiones | 100% | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tiempo Medio de Documentación | < 1 día | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Distribución de Riesgos | < 20% alto/crítico | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tiempo de Respuesta a Incidentes | < 24h | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Puntuación de Cumplimiento | > 75% | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |

---

## 4. Calendario de Medición

| Actividad | Frecuencia | Responsable | Resultado |
|----------|-----------|-------------|--------|
| Ejecutar `devtrail metrics` | Mensual | [Líder del Equipo de Desarrollo] | Valores de KPIs actualizados |
| Ejecutar `devtrail compliance --all` | Trimestral | [Líder de Gobernanza de IA] | Puntuación de cumplimiento |
| Actualizar Panel Resumen de KPIs | Mensual | [Líder de Gobernanza de IA] | Este documento (Sección 3) |
| Revisar KPIs en revisión por la dirección | Trimestral | [Dirección] | MANAGEMENT-REVIEW-TEMPLATE |
| Reevaluar objetivos | Anual | [Líder de Gobernanza de IA] | Objetivos actualizados en este documento |

---

## 5. Agregar KPIs Personalizados

Las organizaciones pueden definir KPIs adicionales. Utilice la siguiente plantilla:

### KPI-NN: [Nombre del KPI]

| Campo | Valor |
|-------|-------|
| **Nombre del KPI** | [Nombre] |
| **Descripción** | [Qué mide este KPI] |
| **Objetivo** | [Valor objetivo] |
| **Valor Actual** | [Valor medido] |
| **Método de Medición** | [Cómo medir] |
| **Frecuencia** | [Con qué frecuencia] |
| **Responsable** | [Persona responsable] |
| **Referencia ISO 42001** | [Cláusula aplicable] |

---

## 6. Mapeo ISO 42001 Cláusula 9

| Cláusula | Requisito | Cobertura de KPIs |
|--------|-------------|-------------|
| 9.1 | Seguimiento, medición, análisis y evaluación | KPI-01, KPI-02, KPI-03 |
| 9.2 | Auditoría interna | KPI-06 (Puntuación de Cumplimiento) |
| 9.3 | Revisión por la dirección | Todos los KPIs alimentan MANAGEMENT-REVIEW-TEMPLATE |
| 6.1 | Acciones para abordar riesgos y oportunidades | KPI-04 (Distribución de Riesgos) |
| 10.1 | No conformidad y acción correctiva | KPI-05 (Tiempo de Respuesta a Incidentes) |

---

*Plantilla de KPIs de Gobernanza de IA — DevTrail Framework*
*Alineado con ISO/IEC 42001:2023 Cláusula 9*

<!-- Template: DevTrail | https://strangedays.tech -->
