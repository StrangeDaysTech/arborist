---
id: AILOG-YYYY-MM-DD-NNN
title: [Título descriptivo de la acción]
status: accepted
created: YYYY-MM-DD
agent: [agent-name-v1.0]
confidence: high | medium | low
review_required: false
risk_level: low | medium | high | critical
eu_ai_act_risk: not_applicable  # unacceptable | high | limited | minimal | not_applicable
nist_genai_risks: []            # privacy | bias | confabulation | cbrn | dangerous_content | environmental | human_ai_config | information_integrity | information_security | intellectual_property | obscene_content | value_chain
iso_42001_clause: []            # 4 | 5 | 6 | 7 | 8 | 9 | 10
lines_changed: 0                # Auto-calculable
files_modified: []              # Auto-calculable
observability_scope: none        # none | basic | full — activar cuando la instrumentación OTel sea relevante
tags: []
related: []
---

# AILOG: [Título]

## Resumen

[Descripción breve de lo que se hizo y por qué]

## Contexto

[Situación que motivó esta acción]

## Acciones Realizadas

1. [Acción 1]
2. [Acción 2]
3. [Acción 3]

## Archivos Modificados

| Archivo | Líneas Cambiadas (+/-) | Descripción del Cambio |
|---------|----------------------|------------------------|
| `path/to/file.ts` | +N/-M | [Descripción del cambio] |

## Decisiones Tomadas

[Si hubo decisiones relevantes, documentarlas brevemente o referenciar AIDEC]

## Impacto

- **Funcionalidad**: [Descripción]
- **Rendimiento**: [Descripción o N/A]
- **Seguridad**: [Descripción o N/A]
- **Privacidad**: [Descripción o N/A]
- **Ambiental**: [Descripción o N/A]

## Verificación

- [ ] El código compila sin errores
- [ ] Las pruebas pasan
- [ ] Se realizó revisión manual
- [ ] Escaneo de seguridad pasado (si risk_level: high/critical)
- [ ] Revisión de privacidad completada (si maneja PII)

## Consideraciones EU AI Act

> Completar esta sección solo si `eu_ai_act_risk` es `high` o `limited`.

- **Clasificación de Riesgo**: [high | limited]
- **Categoría Anexo III**: [Si aplica — especificar categoría]
- **Evaluación de Conformidad Requerida**: [Sí/No]
- **Obligaciones de Transparencia**: [Descripción de obligaciones aplicables]

## Evaluación de Riesgos NIST GenAI

> Completar esta sección cuando el cambio involucre componentes de IA generativa.
> Referencia: NIST AI 600-1 (Perfil de IA Generativa).

| # | Categoría | Aplica | Descripción | Mitigación |
|---|----------|--------|-------------|------------|
| 1 | Información CBRN | [Sí/No] | | |
| 2 | Confabulación | [Sí/No] | | |
| 3 | Contenido Peligroso/Violento/de Odio | [Sí/No] | | |
| 4 | Privacidad de Datos | [Sí/No] | | |
| 5 | Impactos Ambientales | [Sí/No] | | |
| 6 | Sesgo Dañino / Homogeneización | [Sí/No] | | |
| 7 | Configuración Humano-IA | [Sí/No] | | |
| 8 | Integridad de la Información | [Sí/No] | | |
| 9 | Seguridad de la Información | [Sí/No] | | |
| 10 | Propiedad Intelectual | [Sí/No] | | |
| 11 | Contenido Obsceno/Abusivo | [Sí/No] | | |
| 12 | Cadena de Valor / Integración de Componentes | [Sí/No] | | |

## Notas Adicionales

[Cualquier información adicional relevante]

> **Nota de observabilidad**: Si este cambio modifica instrumentación de observabilidad (nuevos spans, atributos cambiados, configuración de pipeline), describir el impacto en observabilidad e incluir tag `observabilidad`.

---

<!-- Template: DevTrail | https://strangedays.tech -->
