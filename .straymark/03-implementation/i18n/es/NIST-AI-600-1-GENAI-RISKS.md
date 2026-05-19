# NIST AI 600-1 --- Referencia de Categorías de Riesgo de IA Generativa

> **Estándar**: NIST AI 600-1 --- Artificial Intelligence Risk Management Framework: Generative AI Profile
>
> NIST AI 600-1 define 12 categorías de riesgo específicas para sistemas de IA generativa. Esta referencia mapea cada categoría a las plantillas de DevTrail, valores de frontmatter y mitigaciones prácticas. Úsela junto con las guías de funciones del NIST AI RMF para asegurar una cobertura integral de riesgos de GenAI.

---

## 1. Información CBRN

**Identificador**: `cbrn`

Los sistemas de IA generativa pueden producir información que podría asistir en la creación, adquisición o despliegue de armas o materiales químicos, biológicos, radiológicos o nucleares (CBRN).

- Un modelo de lenguaje grande proporciona instrucciones paso a paso para la síntesis de sustancias químicas controladas
- Un modelo de generación de código produce código de explotación funcional dirigido a sistemas de control de infraestructura crítica
- Un asistente de IA responde a consultas sobre el cultivo de agentes biológicos con detalle procesable

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [cbrn]` |
| Secciones clave | Risk Assessment, Recommendations |
| Documento de apoyo | SEC (controles de filtrado de salida) |

### Mitigaciones Recomendadas

- Implementar filtrado de salida y clasificadores de seguridad de contenido que detecten contenido relacionado con CBRN
- Restringir las capacidades del modelo en dominios adyacentes a CBRN mediante ajuste fino o prompts de sistema
- Establecer flujos de revisión humana para consultas que activen indicadores de seguridad CBRN

---

## 2. Confabulación

**Identificador**: `confabulation`

Los sistemas de IA generativa pueden producir salidas que son factualmente incorrectas, fabricadas o inconsistentes con los datos de entrenamiento, mientras las presentan con alta confianza aparente.

- Un modelo de resumen fabrica citas que no existen en el material fuente
- Un modelo de generación de código inventa métodos de API que no forman parte de la biblioteca referenciada
- Una IA conversacional proporciona información médica que contradice las guías clínicas establecidas

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [confabulation]` |
| Secciones clave | Risk Assessment, Transparency, Recommendations |
| Documento de apoyo | MCARD (sección Limitations), TES (pruebas de precisión) |

### Mitigaciones Recomendadas

- Implementar generación aumentada por recuperación (RAG) para fundamentar las salidas en fuentes verificadas
- Agregar indicadores de confianza y atribución de fuentes a las salidas del modelo
- Crear documentos TES con benchmarks de precisión factual y pruebas de detección de alucinaciones

---

## 3. Contenido Peligroso, Violento u Odioso

**Identificador**: `dangerous_content`

Los sistemas de IA generativa pueden generar contenido que promueva, glorifique o proporcione instrucciones para violencia, autolesiones u odio dirigido a individuos o grupos.

- Un modelo de generación de texto produce contenido que incita a la violencia contra un grupo específico
- Un modelo de generación de imágenes crea representaciones realistas de violencia gráfica bajo solicitud
- Un chatbot proporciona instrucciones detalladas de autolesiones cuando es consultado por un usuario vulnerable

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [dangerous_content]` |
| Secciones clave | Risk Assessment, Bias Evaluation, Recommendations |
| Documento de apoyo | SEC (controles de moderación de contenido) |

### Mitigaciones Recomendadas

- Desplegar clasificadores de seguridad de contenido tanto en entradas como en salidas
- Implementar protocolos de escalamiento para intentos detectados de contenido dañino
- Probar la efectividad del filtrado de contenido con red-teaming adversarial (documentado en TES)

---

## 4. Privacidad de Datos

**Identificador**: `privacy`

Los sistemas de IA generativa pueden memorizar, filtrar o revelar inadvertidamente datos personales, información sensible o detalles privados de los datos de entrenamiento o interacciones con usuarios.

- Un modelo de lenguaje reproduce pasajes textuales que contienen información de identificación personal de sus datos de entrenamiento
- Una IA conversacional retiene y muestra información de la sesión de un usuario a otro
- Un modelo entrenado con datos propietarios filtra secretos comerciales a través de prompts cuidadosamente elaborados

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH, DPIA |
| Valor de frontmatter | `nist_genai_risks: [privacy]` |
| Secciones clave | sección Data Privacy (ETH), DPIA completo |
| Documento de apoyo | SEC (controles de acceso, aislamiento de datos) |

### Mitigaciones Recomendadas

- Realizar un DPIA antes de desplegar sistemas entrenados con o que procesen datos personales
- Implementar técnicas de privacidad diferencial o saneamiento de datos en los pipelines de entrenamiento
- Probar vulnerabilidades de memorización y extracción de datos (documentado en TES)

---

## 5. Impactos Ambientales

**Identificador**: `environmental`

El entrenamiento, ajuste fino e inferencia de modelos de IA generativa consumen recursos computacionales sustanciales, contribuyendo al uso de energía, emisiones de carbono y residuos electrónicos.

- Entrenar un modelo fundacional grande consume energía equivalente a cientos de hogares durante un año
- Los ciclos de reentrenamiento frecuentes multiplican el costo ambiental sin ganancias proporcionales de capacidad
- Desplegar inferencia a escala genera un consumo energético sostenido a través de la infraestructura de centros de datos

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [environmental]` |
| Secciones clave | sección Environmental Impact |
| Documento de apoyo | MCARD (requisitos de cómputo), AI-KPIS.md (métricas de eficiencia) |

### Mitigaciones Recomendadas

- Documentar los requisitos de cómputo y estimaciones de energía en el MCARD
- Rastrear y reportar métricas de huella de carbono en AI-KPIS.md
- Evaluar alternativas de eficiencia de modelos (modelos más pequeños, destilación, cuantización) en documentos ADR

---

## 6. Sesgo Perjudicial y Homogeneización

**Identificador**: `bias`

Los sistemas de IA generativa pueden amplificar sesgos sociales presentes en los datos de entrenamiento, produciendo salidas que discriminan o representan erróneamente a grupos específicos. La homogeneización ocurre cuando el uso generalizado de IA reduce la diversidad de pensamiento y expresión.

- Un modelo de generación de imágenes subrepresenta consistentemente a ciertos grupos demográficos en entornos profesionales
- Un modelo de texto asocia estereotipos negativos con nacionalidades o géneros específicos
- El uso generalizado de un único asistente de escritura de IA homogeneiza los estilos de comunicación en toda una industria

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [bias]` |
| Secciones clave | Bias Evaluation, Social Impact, Recommendations |
| Documento de apoyo | TES (pruebas de equidad), MCARD (documentación de datos de entrenamiento) |

### Mitigaciones Recomendadas

- Realizar evaluaciones de sesgo a través de dimensiones demográficas relevantes (documentado en ETH Bias Evaluation)
- Implementar métricas de equidad y probar el impacto dispar en documentos TES
- Documentar la composición de los datos de entrenamiento y las brechas de representación conocidas en el MCARD

---

## 7. Configuración Humano-IA

**Identificador**: `human_ai_config`

Riesgos derivados de niveles inadecuados de supervisión humana, dependencia excesiva de las salidas de IA, sesgo de automatización o configuración deficiente de los límites de interacción humano-IA.

- Los operadores aprueban automáticamente las recomendaciones de IA sin una revisión significativa debido al sesgo de automatización
- Un sistema se despliega sin rutas claras de escalamiento para casos donde se necesita el juicio humano
- Los usuarios desarrollan una confianza excesiva en las salidas de IA después de una interacción prolongada sin encontrar errores

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [human_ai_config]` |
| Secciones clave | Risk Assessment, Recommendations |
| Documento de apoyo | AGENT-RULES.md (límites de autonomía), MCARD (límites de uso previsto) |

### Mitigaciones Recomendadas

- Definir requisitos explícitos de supervisión humana en AGENT-RULES.md para cada nivel de autonomía
- Documentar los patrones previstos de interacción humano-IA en la sección Intended Use del MCARD
- Implementar puntos de control de revisión humana obligatoria para decisiones de alto impacto (documentado en ADR)

---

## 8. Integridad de la Información

**Identificador**: `information_integrity`

La IA generativa puede usarse para crear o amplificar desinformación, información falsa y medios manipulados, socavando la confianza pública y la integridad de los ecosistemas de información.

- Un modelo genera artículos de noticias altamente convincentes pero fabricados, indistinguibles del periodismo legítimo
- Medios deepfake generados por IA se usan para suplantar a figuras públicas en campañas de desinformación
- La generación automatizada de contenido inunda los canales de información con contenido de baja calidad y engañoso a escala

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [information_integrity]` |
| Secciones clave | Risk Assessment, Transparency, Social Impact |
| Documento de apoyo | SEC (controles de procedencia), MCARD (marca de agua de salida) |

### Mitigaciones Recomendadas

- Implementar procedencia de contenido y marca de agua para las salidas generadas por IA (documentar el enfoque en ADR)
- Establecer políticas de uso que prohíban el uso engañoso y documentarlas en AI-GOVERNANCE-POLICY.md
- Desplegar mecanismos de detección de contenido manipulado o generado por IA en sistemas posteriores

---

## 9. Seguridad de la Información

**Identificador**: `information_security`

Los sistemas de IA generativa introducen nuevas superficies de ataque, incluyendo inyección de prompts, extracción de modelos, envenenamiento de datos de entrenamiento y entradas adversariales que evaden los controles de seguridad.

- Un atacante usa inyección de prompts para anular las instrucciones del sistema y extraer configuración sensible
- Un modelo es manipulado a través de entradas adversariales para producir salidas que evaden los filtros de contenido
- El envenenamiento de datos de entrenamiento causa que un modelo produzca salidas sutilmente incorrectas en dominios específicos

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | SEC |
| Valor de frontmatter | `nist_genai_risks: [information_security]` |
| Secciones clave | Threat Model, Controls Implemented, Vulnerabilities |
| Documento de apoyo | ETH (Risk Assessment), TES (pruebas de seguridad) |

### Mitigaciones Recomendadas

- Crear documentos SEC con modelos de amenazas específicos para vectores de ataque de GenAI (inyección de prompts, extracción, envenenamiento)
- Implementar validación de entradas y saneamiento de salidas documentados en SEC
- Realizar ejercicios de red-teaming adversarial y documentar los resultados en TES

---

## 10. Propiedad Intelectual

**Identificador**: `intellectual_property`

La IA generativa puede infringir derechos de propiedad intelectual al reproducir material con derechos de autor, generar salidas sustancialmente similares a obras protegidas o usar datos propietarios sin autorización.

- Un modelo de generación de código reproduce secciones textuales de código fuente con derechos de autor de sus datos de entrenamiento
- Un modelo de generación de imágenes produce salidas que replican de cerca el estilo distintivo de un artista vivo
- Un modelo entrenado con documentos corporativos propietarios genera salidas que revelan secretos comerciales protegidos

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [intellectual_property]` |
| Secciones clave | Risk Assessment, Recommendations |
| Documento de apoyo | SBOM (procedencia de datos de entrenamiento, cumplimiento de licencias) |

### Mitigaciones Recomendadas

- Documentar las fuentes de datos de entrenamiento y sus términos de licencia en el SBOM
- Implementar detección de similitud de salidas contra obras conocidas con derechos de autor
- Establecer flujos de revisión de propiedad intelectual para contenido generado por IA usado en aplicaciones comerciales (documentado en ADR)

---

## 11. Contenido Obsceno o Degradante

**Identificador**: `obscene_content`

Los sistemas de IA generativa pueden producir contenido sexualmente explícito, obsceno o degradante, ya sea mediante generación directa o explotando debilidades en los filtros de contenido.

- Un modelo de generación de imágenes es manipulado mediante ingeniería de prompts para evadir los filtros de seguridad y producir contenido explícito
- Un modelo de texto genera descripciones degradantes de individuos basadas en sus características demográficas
- Un sistema de IA produce imágenes íntimas no consentidas combinando fotografías disponibles públicamente con técnicas generativas

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | ETH |
| Valor de frontmatter | `nist_genai_risks: [obscene_content]` |
| Secciones clave | Risk Assessment, Recommendations |
| Documento de apoyo | SEC (controles de filtrado de contenido) |

### Mitigaciones Recomendadas

- Desplegar filtros de seguridad de contenido multicapa tanto en entradas como en salidas
- Implementar clasificadores de contenido robustos que resistan técnicas comunes de jailbreaking
- Realizar ejercicios de red-teaming dirigidos específicamente a la evasión de filtros de contenido (documentado en TES)

---

## 12. Cadena de Valor e Integración de Componentes

**Identificador**: `value_chain`

Riesgos derivados de la integración de componentes de IA de terceros, modelos, conjuntos de datos y servicios en sistemas más grandes, donde los cambios, vulnerabilidades o problemas de calidad aguas arriba se propagan aguas abajo.

- Un modelo de embeddings de terceros introduce un sesgo sutil que se propaga a través de todas las aplicaciones posteriores
- Un proveedor de API cambia el comportamiento del modelo sin previo aviso, rompiendo supuestos en sistemas dependientes
- Un modelo ajustado hereda vulnerabilidades no reveladas de su modelo base

### Mapeo DevTrail

| Aspecto | Valor |
|--------|-------|
| Plantilla principal | SBOM |
| Valor de frontmatter | `nist_genai_risks: [value_chain]` |
| Secciones clave | Third-Party Services, Components, Dependencies |
| Documento de apoyo | ETH (riesgo de terceros), SEC (seguridad de la cadena de suministro) |

### Mitigaciones Recomendadas

- Mantener documentos SBOM completos que cubran todos los componentes de IA en la cadena de valor
- Fijar versiones de modelos y documentar líneas base de comportamiento esperado en MCARD
- Establecer requisitos contractuales para la notificación de cambios por parte de proveedores de servicios de IA (documentado en ADR)

---

## Resumen: Mapeo de Categorías de Riesgo de GenAI a DevTrail

| Categoría | Identificador | Plantilla DevTrail Principal | Valor de Frontmatter |
|----------|-----------|---------------------------|-------------------|
| Información CBRN | `cbrn` | ETH | `nist_genai_risks: [cbrn]` |
| Confabulación | `confabulation` | ETH | `nist_genai_risks: [confabulation]` |
| Contenido Peligroso/Violento/Odioso | `dangerous_content` | ETH | `nist_genai_risks: [dangerous_content]` |
| Privacidad de Datos | `privacy` | ETH, DPIA | `nist_genai_risks: [privacy]` |
| Impactos Ambientales | `environmental` | ETH | `nist_genai_risks: [environmental]` |
| Sesgo Perjudicial y Homogeneización | `bias` | ETH | `nist_genai_risks: [bias]` |
| Configuración Humano-IA | `human_ai_config` | ETH | `nist_genai_risks: [human_ai_config]` |
| Integridad de la Información | `information_integrity` | ETH | `nist_genai_risks: [information_integrity]` |
| Seguridad de la Información | `information_security` | SEC | `nist_genai_risks: [information_security]` |
| Propiedad Intelectual | `intellectual_property` | ETH | `nist_genai_risks: [intellectual_property]` |
| Contenido Obsceno/Degradante | `obscene_content` | ETH | `nist_genai_risks: [obscene_content]` |
| Cadena de Valor e Integración de Componentes | `value_chain` | SBOM | `nist_genai_risks: [value_chain]` |

---

*Referencia de Categorías de Riesgo de IA Generativa NIST AI 600-1 --- DevTrail Framework*

<!-- Template: DevTrail | https://strangedays.tech -->
