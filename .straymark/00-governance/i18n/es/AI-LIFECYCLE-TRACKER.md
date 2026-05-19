# Rastreador del Ciclo de Vida de Sistemas de IA

> **Alineado con**: ISO/IEC 42001:2023 Anexo A.6 — Ciclo de Vida del Sistema de IA
>
> Este documento rastrea todos los sistemas de IA bajo gobernanza a lo largo de sus fases del ciclo de vida. Proporciona visibilidad sobre el estado actual, la propiedad y el estado de cumplimiento de cada sistema, permitiendo a los equipos gestionar transiciones y asegurar controles apropiados en cada fase.
>
> **Esta es una plantilla** — registre los sistemas de IA de su organización y actualice a medida que progresen a través de las fases del ciclo de vida.

---

## 1. Inventario de Sistemas de IA

| Nombre del Sistema | Tipo | Fase Actual | Versión | Responsable | Última Fecha de Revisión | Nivel de Riesgo |
|------------|------|--------------|---------|-------|-----------------|-----------|
| Acme Sentiment Classifier | Clasificador (NLP) | Monitoreo | 2.1.0 | [Líder del Equipo de ML] | [YYYY-MM-DD] | Medio |
| [Nombre del sistema] | [LLM / Clasificador / Recomendador / Agente / Otro] | [Fase] | [Versión] | [Responsable] | [Fecha] | [Bajo / Medio / Alto / Crítico] |

> **Mapeo DevTrail**: Cada sistema debe tener documentos ETH, MCARD y AILOG correspondientes. Utilice `devtrail status` para verificar la cobertura de documentación.

---

## 2. Definiciones de las Fases del Ciclo de Vida

> Cada fase se mapea a los controles del Anexo A.6.2 de ISO 42001. Las listas de verificación a continuación definen los requisitos mínimos antes de que un sistema pueda transicionar a la siguiente fase.

### Fase 1: Diseño (ISO 42001 A.6.2.2)

> Definir requisitos, arquitectura y evaluación inicial de riesgos antes de que comience el desarrollo.

- [ ] Propósito del sistema y uso previsto documentado (REQ)
- [ ] Decisión de arquitectura registrada (ADR o AIDEC)
- [ ] Evaluación inicial de riesgos completada (ETH)
- [ ] Requisitos de datos identificados
- [ ] Criterios de éxito y métricas definidos
- [ ] Partes interesadas y afectadas identificadas
- [ ] Requisitos regulatorios mapeados (REQ)

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Especificación de requisitos | REQ |
| Decisión de arquitectura | ADR, AIDEC |
| Evaluación inicial de riesgos | ETH |

---

### Fase 2: Entrenamiento y Pruebas (ISO 42001 A.6.2.3)

> Construir, entrenar y probar el modelo de IA con datos de calidad asegurada y evaluación de sesgo.

- [ ] Calidad de los datos de entrenamiento verificada y documentada
- [ ] Procedencia de los datos registrada (SBOM)
- [ ] Evaluación de sesgo completada (ETH — Sección de Sesgo)
- [ ] Tarjeta del modelo creada (MCARD)
- [ ] Configuración de entrenamiento documentada
- [ ] Benchmarks de rendimiento iniciales registrados
- [ ] Evaluación de privacidad de datos completada (DPIA si aplica)

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Evaluación de calidad de datos | MCARD |
| Evaluación de sesgo | ETH |
| Documentación del modelo | MCARD |
| Procedencia de datos | SBOM |

---

### Fase 3: Verificación y Validación (ISO 42001 A.6.2.4)

> Ejecutar planes de prueba, validar contra requisitos y confirmar que el sistema cumple con los criterios definidos.

- [ ] Plan de pruebas ejecutado y resultados documentados (TES)
- [ ] Métricas de rendimiento cumplen los objetivos definidos
- [ ] Métricas de equidad evaluadas a través de grupos demográficos
- [ ] Pruebas de seguridad completadas (entradas adversarias, inyección de prompts)
- [ ] Análisis de casos límite y modos de fallo completado
- [ ] Evaluación de explicabilidad aprobada
- [ ] Aprobación del revisor designado

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Resultados de pruebas | TES |
| Evaluación de seguridad | ETH (Sección de Seguridad) |
| Aprobación del revisor | ETH (Aprobación) |

---

### Fase 4: Despliegue (ISO 42001 A.6.2.5)

> Liberar el sistema a producción con monitoreo, planes de reversión y documentación operativa.

- [ ] Plan de despliegue documentado
- [ ] Monitoreo y alertas configurados
- [ ] Plan de reversión definido y probado
- [ ] Documentación de usuario o divulgación preparada
- [ ] Mecanismos de supervisión humana implementados
- [ ] Procedimientos de respuesta a incidentes definidos
- [ ] Despliegue registrado (AILOG)

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Registro de despliegue | AILOG |
| Configuración de monitoreo | AILOG |
| Plan de reversión | ADR |

---

### Fase 5: Operación y Monitoreo (ISO 42001 A.6.2.6)

> Monitorear continuamente el rendimiento del sistema, la deriva de datos y el cumplimiento en producción.

- [ ] SLOs definidos y rastreados
- [ ] Monitoreo de deriva de datos activo
- [ ] Rendimiento del modelo rastreado contra línea base
- [ ] Procedimientos de respuesta a incidentes activos
- [ ] Verificaciones de cumplimiento periódicas programadas
- [ ] Mecanismo de recopilación de retroalimentación de usuarios implementado
- [ ] Reevaluación periódica de riesgos programada

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Registros de rendimiento | AILOG |
| Incidentes | INC |
| Reevaluación de riesgos | ETH |
| Verificaciones de cumplimiento | `devtrail compliance` |

---

### Fase 6: Retiro (ISO 42001 A.6.2.7)

> Descomisionar el sistema de forma segura con el manejo adecuado de datos y notificación a las partes interesadas.

- [ ] Decisión de retiro documentada (ADR)
- [ ] Partes interesadas y afectadas notificadas
- [ ] Datos archivados o eliminados según la política de retención
- [ ] Artefactos del modelo archivados o eliminados
- [ ] Endpoints de API deprecados con período de aviso apropiado
- [ ] Sistema de reemplazo documentado (si aplica)
- [ ] Estado final registrado en este rastreador

| Evidencia DevTrail | Tipo de Documento |
|-------------------|---------------|
| Decisión de retiro | ADR |
| Actualización de estado final | AILOG |
| Registro de disposición de datos | AILOG |

---

## 3. Ejemplo: Acme Sentiment Classifier

> Ejemplo pre-completado mostrando un sistema en la fase de monitoreo.

### Detalles del Sistema

| Campo | Valor |
|-------|-------|
| **Nombre del Sistema** | Acme Sentiment Classifier |
| **Tipo** | Clasificador (NLP) |
| **Fase Actual** | Operación y Monitoreo |
| **Versión** | 2.1.0 |
| **Responsable** | Líder del Equipo de ML |
| **Nivel de Riesgo** | Medio |
| **Fecha de Despliegue** | 2025-09-15 |
| **Última Revisión** | [YYYY-MM-DD] |
| **Próxima Revisión** | [YYYY-MM-DD] |

### Historial de Completación de Fases

| Fase | Completada | Fecha | Documentos Clave |
|-------|-----------|------|--------------|
| Diseño | Sí | 2025-03-10 | REQ-001, ADR-012, ETH-008 |
| Entrenamiento/Pruebas | Sí | 2025-06-20 | MCARD-003, ETH-008 (actualizado) |
| Verificación/Validación | Sí | 2025-08-05 | TES-015, TES-016 |
| Despliegue | Sí | 2025-09-15 | AILOG-042 |
| Operación/Monitoreo | Activa | — | AILOG-050, INC-003 |
| Retiro | — | — | — |

### Estado Actual de Monitoreo

| Métrica | Objetivo | Actual | Estado |
|--------|--------|---------|--------|
| Precisión | > 92% | 94.1% | En objetivo |
| Equidad (paridad demográfica) | < 5% de diferencia | 3.2% de diferencia | En objetivo |
| Latencia (p95) | < 200ms | 145ms | En objetivo |
| Puntuación de deriva de datos | < 0.15 | 0.08 | En objetivo |

---

## 4. Aprobación de Transición de Fase

> Documentar las aprobaciones requeridas para cada transición de fase.

| Transición | Aprobador Requerido | Método de Aprobación |
|-----------|-------------------|-----------------|
| Diseño a Entrenamiento/Pruebas | [Responsable del Sistema] | AIDEC o ADR |
| Entrenamiento/Pruebas a Verificación | [Líder del Equipo de ML] | Completación de MCARD |
| Verificación a Despliegue | [Revisor de Ética de IA + Responsable del Sistema] | Aprobación ETH + TES aprobado |
| Despliegue a Monitoreo | [Líder de Operaciones] | AILOG de despliegue |
| Cualquier fase a Retiro | [Líder de Gobernanza de IA + Dirección] | ADR con justificación |

---

## 5. Resumen de Controles ISO 42001 Anexo A.6

| Control | Descripción | Fase(s) del Ciclo de Vida | Evidencia DevTrail |
|---------|-------------|--------------------|--------------------|
| A.6.2.2 | Diseño y desarrollo de sistemas de IA | Diseño, Desarrollo | ADR, AIDEC, REQ |
| A.6.2.3 | Entrenamiento y pruebas de modelos de IA | Entrenamiento/Pruebas | MCARD, TES |
| A.6.2.4 | Verificación y validación | Verificación | TES |
| A.6.2.5 | Despliegue de sistemas de IA | Despliegue | AILOG |
| A.6.2.6 | Operación y monitoreo | Monitoreo | AILOG, INC |
| A.6.2.7 | Retiro de sistemas de IA | Retiro | ADR, AILOG |
| A.6.2.8 | Integración responsable de IA | Todas las fases | ADR, AIDEC |
| A.6.2.9 | Documentación de sistemas de IA | Todas las fases | AILOG (todos los cambios) |
| A.6.2.10 | Uso definido y uso indebido | Diseño, Despliegue | MCARD, REQ |
| A.6.2.11 | Componentes de IA de terceros | Todas las fases | SBOM |

---

*Plantilla del Rastreador del Ciclo de Vida de Sistemas de IA — DevTrail Framework*
*Alineado con ISO/IEC 42001:2023 Anexo A.6*

<!-- Template: DevTrail | https://strangedays.tech -->
