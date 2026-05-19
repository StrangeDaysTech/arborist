# Plantilla de Revisión por la Dirección

> **Alineado con**: ISO/IEC 42001:2023 Cláusula 9.3 — Revisión por la Dirección
>
> Esta plantilla estructura las revisiones periódicas por la dirección del sistema de gestión de IA. Asegura que todos los insumos requeridos según la Cláusula 9.3 sean considerados y que las decisiones y acciones se registren para seguimiento.
>
> **Esta es una plantilla** — complete una instancia por período de revisión y archive en los registros de su proyecto.

---

## Metadatos de la Revisión

| Campo | Valor |
|-------|-------|
| **Fecha de Revisión** | [YYYY-MM-DD] |
| **Asistentes** | [Nombres y roles] |
| **Período de Revisión** | [Fecha de inicio] a [Fecha de fin] |
| **Próxima Fecha de Revisión** | [YYYY-MM-DD] |
| **Facilitador** | [Nombre] |
| **Acta Redactada Por** | [Nombre] |

---

## Agenda

### 1. Estado de Acciones de Revisiones Anteriores

> ISO 42001 Cláusula 9.3 insumo (a): estado de las acciones de revisiones por la dirección anteriores.

| Acción | Responsable | Fecha Límite | Estado | Notas |
|--------|-------|----------|--------|-------|
| [Acción de revisión anterior] | [Responsable] | [Fecha] | [Abierta / En progreso / Cerrada] | [Notas] |
| [Acción de revisión anterior] | [Responsable] | [Fecha] | [Abierta / En progreso / Cerrada] | [Notas] |

**Notas de discusión**: [Registrar cualquier discusión sobre acciones vencidas o incompletas]

---

### 2. Cambios en Cuestiones Externas e Internas

> ISO 42001 Cláusula 9.3 insumo (b): cambios en cuestiones externas e internas relevantes para el sistema de gestión de IA.

#### 2.1 Cambios Regulatorios

| Cambio | Impacto | Acción Requerida | Responsable |
|--------|--------|-----------------|-------|
| [Nueva regulación o enmienda] | [Impacto en sistemas de IA] | [Acción necesaria] | [Responsable] |

#### 2.2 Cambios Organizacionales

| Cambio | Impacto | Acción Requerida | Responsable |
|--------|--------|-----------------|-------|
| [Reestructuración de equipos, nuevos sistemas, cambios de estrategia] | [Impacto en la gobernanza] | [Acción necesaria] | [Responsable] |

#### 2.3 Cambios Tecnológicos

| Cambio | Impacto | Acción Requerida | Responsable |
|--------|--------|-----------------|-------|
| [Nuevas capacidades de IA, cambios de proveedor, actualizaciones de infraestructura] | [Impacto en el perfil de riesgo] | [Acción necesaria] | [Responsable] |

**Notas de discusión**: [Registrar discusión sobre cambios significativos]

---

### 3. Información sobre el Desempeño del Sistema de IA

> ISO 42001 Cláusula 9.3 insumo (c): información sobre el desempeño del sistema de gestión de IA.

#### 3.1 Métricas de Gobernanza (de AI-KPIS.md)

| KPI | Objetivo | Actual | Anterior | Tendencia | Estado |
|-----|--------|---------|----------|-------|--------|
| Cobertura de Documentación | > 80% | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tasa de Cumplimiento de Revisiones | 100% | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tiempo Medio de Documentación | < 1 día | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Distribución de Riesgos | < 20% alto/crítico | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Tiempo de Respuesta a Incidentes | < 24h | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |
| Puntuación de Cumplimiento | > 75% | [Valor] | [Valor] | [Alza/Baja/Estable] | [En objetivo / En riesgo / Por debajo del objetivo] |

> **Fuente**: Ejecute `devtrail metrics` para recopilar los valores actuales antes de la revisión.

#### 3.2 Estado de los Sistemas de IA (de AI-LIFECYCLE-TRACKER.md)

| Sistema | Fase | Nivel de Riesgo | Incidencias en Este Período |
|--------|-------|-----------|-------------------|
| [Nombre del sistema] | [Fase actual] | [Nivel de riesgo] | [Resumen de incidencias] |

**Notas de discusión**: [Registrar discusión sobre tendencias de desempeño y salud del sistema]

---

### 4. Resultados de Auditorías

> ISO 42001 Cláusula 9.3 insumo (d): resultados de auditorías.

#### 4.1 Hallazgos de Auditoría Interna

| ID del Hallazgo | Descripción | Severidad | Acción Correctiva | Estado |
|-----------|-------------|----------|-------------------|--------|
| [ID] | [Descripción del hallazgo] | [Mayor / Menor / Observación] | [Acción tomada o planificada] | [Abierto / Cerrado] |

#### 4.2 Hallazgos de Auditoría Externa (si aplica)

| ID del Hallazgo | Descripción | Severidad | Acción Correctiva | Estado |
|-----------|-------------|----------|-------------------|--------|
| [ID] | [Descripción del hallazgo] | [Severidad] | [Acción tomada o planificada] | [Abierto / Cerrado] |

> **Fuente**: Ejecute `devtrail compliance --all` para generar el informe de cumplimiento antes de la revisión.

**Notas de discusión**: [Registrar discusión sobre hallazgos de auditoría y progreso de remediación]

---

### 5. Logro de Objetivos de IA

> ISO 42001 Cláusula 9.3 insumo (e): información sobre el logro de los objetivos de IA.

| Objetivo | Meta | Estado Actual | En Camino | Notas |
|-----------|--------|----------------|----------|-------|
| [Objetivo de la política de gobernanza §3.2] | [Meta] | [Estado] | [Sí / En riesgo / No] | [Notas] |

> **Referencia**: Los objetivos se definen en AI-GOVERNANCE-POLICY.md Sección 3.2 (Objetivos de IA).

**Notas de discusión**: [Registrar discusión sobre el logro de objetivos y ajustes necesarios]

---

### 6. No Conformidades y Acciones Correctivas

> ISO 42001 Cláusula 9.3 insumo (f): no conformidades y acciones correctivas.

| ID de NC | Descripción | Causa Raíz | Acción Correctiva | Estado |
|-------|-------------|-----------|-------------------|--------|
| NC-[NNN] | [Descripción de la no conformidad] | [Análisis de causa raíz] | [Acción correctiva tomada o planificada] | [Abierto / En progreso / Cerrado / Verificado] |

> **Fuente**: Documentos INC en DevTrail y fallos de `devtrail compliance`.

**Notas de discusión**: [Registrar discusión sobre problemas recurrentes y problemas sistémicos]

---

### 7. Oportunidades de Mejora

> ISO 42001 Cláusula 9.3 insumo (g): oportunidades de mejora continua.

| Mejora | Prioridad | Responsable | Fecha Límite | Beneficio Esperado |
|------------|----------|-------|----------|-----------------|
| [Descripción de la mejora] | [Alta / Media / Baja] | [Responsable] | [Fecha] | [Beneficio esperado] |

**Notas de discusión**: [Registrar discusión sobre prioridades de mejora y asignación de recursos]

---

## Decisiones y Acciones

> ISO 42001 Cláusula 9.3 salida: decisiones y acciones relacionadas con oportunidades de mejora continua y cualquier necesidad de cambios en el sistema de gestión de IA.

| Acción | Responsable | Fecha Límite | Estado | Prioridad |
|--------|-------|----------|--------|----------|
| [Descripción de la acción] | [Responsable] | [YYYY-MM-DD] | [No iniciada / En progreso / Completada] | [Alta / Media / Baja] |

---

## Notas

[Notas adicionales, observaciones o contexto de la sesión de revisión]

---

## Aprobación

| Rol | Nombre | Firma | Fecha |
|------|------|-----------|------|
| Facilitador de la Revisión | [Nombre] | | [Fecha] |
| Líder de Gobernanza de IA | [Nombre] | | [Fecha] |
| Representante de la Dirección | [Nombre] | | [Fecha] |

---

*Plantilla de Revisión por la Dirección — DevTrail Framework*
*Alineado con ISO/IEC 42001:2023 Cláusula 9.3*

<!-- Template: DevTrail | https://strangedays.tech -->
