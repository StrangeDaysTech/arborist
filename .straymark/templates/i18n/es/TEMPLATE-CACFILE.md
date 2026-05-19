---
id: CACFILE-YYYY-MM-DD-NNN
title: "[Nombre del Servicio] Registro de Algoritmo CAC"
status: draft
created: YYYY-MM-DD
agent: [agent-name]
confidence: medium
review_required: true

# --- Workflow de aprobación (opcional, llenar al momento de la revisión) ---
# reviewed_by: <id-revisor>            # email | usuario-github | DID
# reviewed_at: YYYY-MM-DD
# review_outcome: approved             # approved | revisions_requested | rejected
risk_level: high
cac_filing_required: true
cac_filing_type: algorithm  # algorithm | generative_ai | dual
cac_filing_status: pending  # pending | provincial_submitted | provincial_approved | national_submitted | national_approved | rejected | not_required
cac_filing_number: null  # se rellena tras registro exitoso
cac_provincial_authority: null
cac_national_decision_date: null
iso_42001_clause: [8]
tags: [china, cac, algorithm-filing]
related: []
---

# CACFILE: [Nombre del Servicio] Registro de Algoritmo CAC

> **IMPORTANTE**: Este documento es un BORRADOR creado por un agente de IA. Requiere revisión por parte del equipo legal y del responsable de cumplimiento antes de cualquier presentación a la Cyberspace Administration of China.
>
> El registro de algoritmo CAC es obligatorio para servicios externos con **atributos de opinión pública** o **capacidad de movilización social**. Los modelos generativos requieren además **doble registro**: evaluación de seguridad provincial + registro de algoritmo nacional.

## Resumen del Servicio

- **Nombre del servicio**: [Nombre orientado al usuario]
- **Entidad legal proveedora**: [Empresa registrada en China continental]
- **Forma del servicio**: [servicio de información en internet / app / mini-program / API / otro]
- **Dominio de aplicación**: [recomendación / búsqueda / personalización / IA generativa / síntesis / otro]
- **Tipo de algoritmo**: [generation_synthesis / personalized_push / sequence_scheduling / search_filter / dispatch_decision]
- **Audiencia objetivo**: [Público / industria específica / menores / otro]
- **Escenarios aplicables**: [Casos de uso concretos]

## Evaluación de Atributos de Opinión Pública / Movilización Social

Por orientación CAC, los servicios con estos atributos deben registrarse:

- [ ] Genera / sintetiza / disemina contenido visible al público
- [ ] Influye en la opinión vía ranking, recomendación o selección
- [ ] Capaz de movilizar usuarios para acción colectiva
- [ ] Opera en noticias, redes sociales, audio/vídeo, streaming en vivo

**Conclusión**: [Requerido / No requerido] — [Justificación]

## Descripción de los Datos de Entrenamiento

- **Fuentes**: [Corpus público / propietario / licenciado / generado por usuarios]
- **Volumen**: [tokens / imágenes / muestras aproximadas]
- **Origen geográfico**: [Doméstico / extranjero / mixto — implicaciones PIPL]
- **Legalidad de fuentes**: [Per Art. 7 Medidas Provisionales: legalidad, exactitud, objetividad, diversidad]
- **Información personal en datos de entrenamiento**: [Sí/No — vincular PIPIA-XXXX]
- **Categorías sensibles filtradas**: [Métodos para remover datos sensibles per PIPL Art. 28]

## Estrategia de Palabras Clave Bloqueadas

> Las Medidas Provisionales obligan a impedir contenido que subvierta el poder estatal, incite al separatismo, socave la unidad nacional, promueva terrorismo / extremismo / odio étnico / discriminación, violencia o contenido obsceno.

- **Fuente de la lista**: [Lista interna, versión, actualización]
- **Cadencia de actualización**: [Frecuencia de revisión]
- **Controles por capas**: [Filtrado pre-prompt / post-generación / rechazo de respuesta / safe completion]
- **Muestra de auditoría**: [Ruta a la muestra retenida]

## Conjunto de Preguntas de Prueba

- **Tamaño**: [Número de prompts de red-team]
- **Cobertura**: [Categorías: sensibilidad política, violencia, autolesiones, odio, discriminación, privacidad, menores]
- **Umbral de aprobación**: [Tasa de aprobación requerida]
- **Último resultado**: [Fecha / tasa / referencia documental]

## Política Interna de Gestión de Algoritmos

- **Responsable designado**: [Nombre / rol con credenciales en chino]
- **Flujo de revisión interna**: [Pasos pre-despliegue]
- **Manejo de quejas**: [Canal + SLA]
- **Logging y trazabilidad**: [Cómo se registran entradas/salidas del modelo]

## Reporte de Responsabilidad de Seguridad del Algoritmo

Per Art. 24 de las Disposiciones de Recomendación Algorítmica:

- **Divulgación del mecanismo**: [Descripción accesible]
- **Notificación al usuario**: [Cómo se informa de la decisión algorítmica]
- **Mecanismo de opt-out**: [Cómo desactivar la personalización]
- **Capacidad de intervención manual**: [Cómo se anula el output algorítmico]

## Auto-evaluación

| Área de Riesgo | Hallazgo | Severidad | Mitigación |
|---------------|----------|-----------|-----------|
| Cumplimiento de contenido | [Hallazgo] | [B/M/A] | [Medida] |
| Protección de información personal | [Hallazgo] | [B/M/A] | [Medida] |
| Sesgo / trato diferenciado injusto | [Hallazgo] | [B/M/A] | [Medida] |
| Propiedad intelectual | [Hallazgo] | [B/M/A] | [Medida] |
| Protección de menores | [Hallazgo] | [B/M/A] | [Medida] |

## Trazabilidad del Registro

| Etapa | Fecha | Resultado / Referencia |
|-------|-------|------------------------|
| Sign-off interno | [YYYY-MM-DD] | [Referencia] |
| Presentación provincial | [YYYY-MM-DD] | [Número de recibo] |
| Decisión provincial | [YYYY-MM-DD] | [Aprobado / Rechazado — razones] |
| Presentación nacional | [YYYY-MM-DD] | [Número de recibo] |
| Decisión nacional | [YYYY-MM-DD] | [Número de filing emitido] |
| Divulgación pública | [YYYY-MM-DD] | [URL donde se muestra el filing al usuario final] |

## Aprobación

| Aprobado por | Fecha | Decisión | Condiciones |
|--------------|-------|----------|-------------|
| [Compliance officer] | [YYYY-MM-DD] | [aprobado / condicional / rechazado] | [Si aplica] |

<!-- Template: StrayMark | https://strangedays.tech -->
