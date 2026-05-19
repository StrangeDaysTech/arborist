---
id: MCARD-YYYY-MM-DD-NNN
title: "[Nombre del Modelo] Card"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true
risk_level: medium
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: [8]
model_name: ""
model_type: LLM  # LLM | classifier | regressor | generator | recommender | other
model_version: ""
provider: ""
license: ""
tags: [ai-model]
related: []
---

# MCARD: [Nombre del Modelo] Card

> **IMPORTANTE**: Este documento es un BORRADOR creado por un agente de IA.
> Requiere revisión y aprobación humana antes de proceder.

## Detalles del Modelo

> Basado en Mitchell et al. (2019) — "Model Cards for Model Reporting".

| Campo | Valor |
|-------|-------|
| Desarrollador | [Organización o individuo que desarrolló el modelo] |
| Fecha del Modelo | [YYYY-MM-DD — fecha en que el modelo fue entrenado o publicado] |
| Versión del Modelo | [Identificador de versión] |
| Tipo de Modelo | [LLM / clasificador / regresor / generador / recomendador / otro] |
| Algoritmos de Entrenamiento | [Algoritmo(s) utilizados para el entrenamiento] |
| Modelo Base | [Nombre y versión del modelo base, si es fine-tuned; N/A en caso contrario] |
| Publicación / Recurso | [URL o cita al artículo, blog o documentación] |
| Cita | [BibTeX o cita en texto plano] |
| Licencia | [Licencia del modelo — ej., Apache 2.0, MIT, propietaria] |

## Uso Previsto

### Usos Principales Previstos

- [Caso de uso principal 1]
- [Caso de uso principal 2]

### Usuarios Principales Previstos

- [Grupo de usuarios 1]
- [Grupo de usuarios 2]

### Usos Fuera del Alcance

- [Caso de uso para el que el modelo NO está diseñado 1]
- [Caso de uso para el que el modelo NO está diseñado 2]

## Datos de Entrenamiento

> Para interoperabilidad con SBOM, considerar alinear con los campos de CycloneDX `modelCard.modelParameters`.

| Campo | Valor |
|-------|-------|
| Nombre del Dataset | [Nombre del dataset de entrenamiento] |
| Fuente | [Dónde se obtuvieron los datos] |
| Tamaño | [Número de muestras, tokens o tamaño de almacenamiento] |
| Metodología de Recolección | [Cómo se recolectaron los datos] |
| Preprocesamiento | [Pasos de limpieza, filtrado y aumento aplicados] |
| Limitaciones Conocidas | [Sesgos, vacíos o problemas de calidad en los datos] |
| Evaluación de PII | [Si hay PII presente y cómo se manejó] |
| Licencia | [Licencia que rige los datos de entrenamiento] |

## Métricas de Rendimiento

| Métrica | Valor | Dataset de Prueba | Intervalo de Confianza | Condiciones |
|---------|:-----:|-------------------|:----------------------:|-------------|
| [Accuracy / F1 / BLEU / etc.] | [Valor] | [Nombre del dataset y split] | [Rango IC 95%] | [Condiciones o configuración] |
| [Métrica 2] | [Valor] | [Nombre del dataset y split] | [Rango IC 95%] | [Condiciones o configuración] |
| [Métrica 3] | [Valor] | [Nombre del dataset y split] | [Rango IC 95%] | [Condiciones o configuración] |

### Evaluación Desagregada

> Reportar rendimiento desglosado por subgrupos relevantes cuando aplique.

| Subgrupo | Métrica | Valor | Comparación con Línea Base |
|----------|---------|:-----:|:--------------------------:|
| [Subgrupo 1] | [Métrica] | [Valor] | [+/- vs general] |
| [Subgrupo 2] | [Métrica] | [Valor] | [+/- vs general] |

## Evaluación de Sesgo y Equidad

| Grupo Demográfico | Métrica | Rendimiento | Disparidad vs Línea Base | Mitigación Aplicada |
|--------------------|---------|:-----------:|:------------------------:|--------------------|
| [Grupo 1 — ej., rango de edad, género, etnia] | [Métrica] | [Valor] | [+/- porcentaje o absoluto] | [Descripción de mitigación] |
| [Grupo 2] | [Métrica] | [Valor] | [+/- porcentaje o absoluto] | [Descripción de mitigación] |
| [Grupo 3] | [Métrica] | [Valor] | [+/- porcentaje o absoluto] | [Descripción de mitigación] |

## Impacto Ambiental

| Métrica | Valor | Notas |
|---------|-------|-------|
| Energía de Entrenamiento (kWh) | [Valor] | [Metodología o fuente de estimación] |
| Equivalente de CO2 (toneladas) | [Valor] | [Intensidad de carbono de la red utilizada] |
| Hardware Utilizado | [GPUs/TPUs, cantidad, modelo] | [Proveedor de nube / región] |
| Duración del Entrenamiento | [Horas / días] | [Tiempo total de cómputo] |
| Costo de Inferencia | [Costo por solicitud o por 1K tokens] | [Promedio / pico] |
| Región / Intensidad de Carbono de la Red | [Nombre de la región] | [gCO2/kWh] |

## Consideraciones de Seguridad

| Preocupación | Evaluación | Detalles |
|--------------|:----------:|---------|
| Vulnerabilidades Conocidas | [Ninguna / Descripción] | [Referencias CVE o descripción de problemas conocidos] |
| Robustez Adversarial | [Baja / Media / Alta] | [Metodología de evaluación y resultados] |
| Riesgo de Inyección de Prompt | [Bajo / Medio / Alto] | [Evaluación de susceptibilidad a inyección de prompt] |
| Riesgo de Envenenamiento de Datos | [Bajo / Medio / Alto] | [Evaluación de integridad de datos de entrenamiento] |
| Riesgo de Extracción del Modelo | [Bajo / Medio / Alto] | [Riesgo de que los pesos o comportamiento del modelo sean extraídos] |

## Consideraciones Éticas

- **Datos Sensibles Utilizados**: [Si se usaron datos sensibles o personales en el entrenamiento y cómo se manejaron]
- **Sujetos Humanos en Entrenamiento**: [Si se involucraron sujetos humanos en la recolección de datos; estado de revisión IRB o comité de ética]
- **Potencial de Uso Dual**: [Si el modelo podría reutilizarse para aplicaciones dañinas; salvaguardas implementadas]
- **Evaluación de Impacto Social**: [Implicaciones sociales más amplias — positivas y negativas]

## Limitaciones y Recomendaciones

### Limitaciones Conocidas

- [Limitación 1 — ej., bajo rendimiento en idiomas o dominios específicos]
- [Limitación 2 — ej., restricciones de ventana de contexto]
- [Limitación 3 — ej., tendencia a alucinar en escenarios específicos]

### Modos de Falla

- [Modo de falla 1 — condiciones bajo las cuales el modelo falla de manera predecible]
- [Modo de falla 2 — casos extremos o entradas adversariales]

### Recomendaciones para Implementadores

- [Recomendación 1 — ej., implementar filtrado de salidas]
- [Recomendación 2 — ej., configurar humano-en-el-bucle para decisiones de alto riesgo]
- [Recomendación 3 — ej., monitorear la degradación a lo largo del tiempo]

---

## Aprobación

| Campo | Valor |
|-------|-------|
| Aprobado por | [Nombre] |
| Fecha | [YYYY-MM-DD] |
| Decisión | [APROBADO / RECHAZADO / CONDICIONAL] |
| Condiciones | [Si aplica] |

<!-- Template: DevTrail | https://strangedays.tech -->
