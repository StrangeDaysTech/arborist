# NIST AI RMF --- Guía de Implementación de la Función MAP

> **Marco de referencia**: NIST AI Risk Management Framework (AI RMF 1.0)
> **Función**: MAP --- Contexto, categorización e identificación de riesgos
>
> La función MAP establece el contexto del sistema de IA, identifica y categoriza los riesgos, y evalúa los impactos potenciales sobre individuos, organizaciones y la sociedad. Es la base de todas las actividades posteriores de gestión de riesgos.

---

## MP-1: Establecimiento de Contexto

Documentar el propósito del sistema de IA, el entorno operativo, las partes interesadas y el dominio previsto. El contexto es la base para identificar los riesgos relevantes.

> Establecer un contexto exhaustivo garantiza que las evaluaciones de riesgos se fundamenten en las condiciones reales de despliegue del sistema, en lugar de suposiciones genéricas.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Propósito y alcance del sistema | AILOG | `title`, sección Summary |
| Entorno operativo | AILOG | sección Context |
| Partes interesadas y usuarios | ETH | sección Stakeholder Analysis |
| Contexto regulatorio | MCARD | `regulatory_frameworks`, sección Regulatory Compliance |
| Restricciones de despliegue | ADR | sección Context and Constraints |

### Lista de Verificación de Implementación

- [ ] Crear una entrada AILOG que documente el propósito, alcance y contexto operativo del sistema de IA
- [ ] Identificar a todas las partes interesadas (usuarios, partes afectadas, operadores) en el documento ETH
- [ ] Registrar el entorno regulatorio y los marcos aplicables en el MCARD
- [ ] Documentar las restricciones y supuestos de despliegue en un ADR

---

## MP-2: Categorización y Clasificación de Riesgos

Clasificar los sistemas de IA por nivel de riesgo según su uso previsto, el potencial de daño y la sensibilidad del dominio en el que operan.

> La clasificación de riesgos determina la profundidad de los requisitos de documentación y revisión. Los sistemas de alto riesgo exigen evidencia más rigurosa.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Clasificación del nivel de riesgo | ETH | `risk_level` (low / medium / high / critical) |
| Nivel de riesgo EU AI Act | ETH | `eu_ai_act_risk` (minimal / limited / high / unacceptable) |
| Riesgos específicos de GenAI | ETH | array `nist_genai_risks` |
| Sensibilidad del dominio | MCARD | `intended_use`, sección Domain |
| Justificación de la categorización del sistema | ADR | sección Decision and Rationale |

### Lista de Verificación de Implementación

- [ ] Asignar un `risk_level` en el frontmatter del ETH basado en el daño potencial
- [ ] Mapear el sistema a un nivel de riesgo del EU AI Act usando el campo `eu_ai_act_risk`
- [ ] Identificar las categorías de riesgo GenAI aplicables en `nist_genai_risks` (ver NIST-AI-600-1-GENAI-RISKS.md)
- [ ] Documentar la justificación de la clasificación elegida en un ADR si el nivel es discutido o no evidente

---

## MP-3: Capacidades y Limitaciones de la IA

Documentar las capacidades previstas del sistema de IA junto con sus limitaciones conocidas, modos de fallo y condiciones bajo las cuales el rendimiento se degrada.

> La documentación transparente de las limitaciones previene la dependencia excesiva y respalda la toma de decisiones informada por parte de operadores y usuarios.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Uso previsto y capacidades | MCARD | sección Intended Use, `intended_use` |
| Limitaciones conocidas | MCARD | sección Limitations |
| Límites de rendimiento | MCARD | sección Performance Metrics |
| Modos de fallo | TES | sección Edge Cases and Failure Tests |
| Usos fuera de alcance | MCARD | sección Out-of-Scope Uses |

### Lista de Verificación de Implementación

- [ ] Completar la sección Intended Use del MCARD con declaraciones explícitas de lo que el sistema está diseñado para hacer
- [ ] Documentar todas las limitaciones conocidas y condiciones de fallo en la sección Limitations del MCARD
- [ ] Definir los límites de rendimiento y condiciones de degradación en Performance Metrics del MCARD
- [ ] Crear documentos TES que validen el comportamiento en los límites y bajo condiciones adversas

---

## MP-4: Mapeo y Registro de Riesgos

Mantener un registro centralizado de los riesgos identificados, su severidad, probabilidad, controles actuales y responsables asignados.

> Un registro de riesgos dinámico asegura que los riesgos se rastreen a lo largo del tiempo, no solo se identifiquen una vez y se olviden.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Catálogo de riesgos | AI-RISK-CATALOG.md | Documento completo |
| Evaluaciones de riesgos individuales | ETH | sección Risk Assessment |
| Severidad y probabilidad del riesgo | AI-RISK-CATALOG.md | columnas Severity, Likelihood |
| Responsables de riesgos | AI-RISK-CATALOG.md | columna Owner |
| Estado de mitigación | AI-RISK-CATALOG.md | columnas Status, Current Controls |

### Lista de Verificación de Implementación

- [ ] Crear o actualizar AI-RISK-CATALOG.md con todos los riesgos identificados
- [ ] Vincular cada entrada de riesgo al documento ETH correspondiente que proporciona la evaluación detallada
- [ ] Asignar un responsable y una fecha de revisión a cada riesgo catalogado
- [ ] Programar revisiones periódicas usando `devtrail compliance` para verificar la vigencia del catálogo de riesgos

---

## MP-5: Evaluación de Impacto

Evaluar los impactos potenciales del sistema de IA sobre individuos (derechos, seguridad, bienestar), grupos (equidad, sesgo), comunidades, el medio ambiente y la organización.

> Las evaluaciones de impacto deben considerar tanto los efectos previstos como las consecuencias no intencionadas razonablemente previsibles en todas las partes afectadas.

### Mapeo DevTrail

| Requisito NIST | Documento DevTrail | Sección / Campo |
|------------------|-------------------|-----------------|
| Impacto en derechos individuales | DPIA | sección Rights Impact Assessment |
| Impacto de sesgo y equidad | ETH | sección Bias Evaluation |
| Impacto ambiental | ETH | sección Environmental Impact |
| Impacto social | ETH | sección Social Impact |
| Impacto organizacional | ETH | sección Risk Assessment |
| Evaluación de impacto en protección de datos | DPIA | Documento completo |

### Lista de Verificación de Implementación

- [ ] Completar un DPIA para sistemas que procesen datos personales o afecten derechos individuales
- [ ] Evaluar los impactos de sesgo y equidad en la sección Bias Evaluation del ETH
- [ ] Evaluar los costos ambientales (cómputo, energía, carbono) en la sección Environmental Impact del ETH
- [ ] Documentar los impactos sociales y comunitarios, especialmente para sistemas desplegados a gran escala

---

## Resumen: Mapeo de la Función MAP a DevTrail

| Categoría | Descripción | Documento DevTrail Principal | Campos / Secciones Clave |
|----------|-------------|---------------------------|----------------------|
| MP-1 | Establecimiento de Contexto | AILOG, ETH | sección Context, Stakeholder Analysis |
| MP-2 | Categorización | ETH | `risk_level`, `eu_ai_act_risk`, `nist_genai_risks` |
| MP-3 | Capacidades de IA | MCARD | Intended Use, Limitations, Performance Metrics |
| MP-4 | Mapeo de Riesgos | AI-RISK-CATALOG.md | Severity, Likelihood, Owner, Status |
| MP-5 | Evaluación de Impacto | DPIA, ETH | Rights Impact, Bias Evaluation, Environmental Impact |

---

*Guía de Implementación de la Función MAP del NIST AI RMF --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
