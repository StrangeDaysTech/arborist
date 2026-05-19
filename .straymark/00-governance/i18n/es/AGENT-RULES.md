# Reglas para Agentes IA - DevTrail

> Este documento define las reglas que todos los agentes de IA deben seguir cuando trabajan en proyectos bajo DevTrail.

**Idiomas**: [English](../../AGENT-RULES.md) | Español

---

## 1. Identificación Obligatoria

### Al Iniciar una Sesión

Cada agente debe identificarse con:
- Nombre del agente (ej.: `claude-code-v1.0`, `cursor-v1.0`, `gemini-cli-v1.0`)
- Versión del agente si está disponible

### En Cada Documento

Incluir en el frontmatter:
```yaml
agent: agent-name-v1.0
confidence: high | medium | low
```

---

## 2. Cuándo Documentar

### OBLIGATORIO - Crear documento

| Situación | Tipo | Notas |
|-----------|------|-------|
| Complejidad de código sobre el umbral | AILOG | Ejecutar `devtrail analyze <archivos-modificados> --output json`. Si `summary.above_threshold > 0`, crear AILOG (umbral por defecto: 8). **Alternativa**: si el CLI no está disponible, usar heurística de >20 líneas de lógica de negocio |
| Decisión entre 2+ alternativas técnicas | AIDEC | Documentar alternativas |
| Cambios en auth/autorización/PII | AILOG + ETH | `risk_level: high`, ETH requiere aprobación |
| Cambios en API pública o esquema de BD | AILOG | `risk_level: medium+`, considerar ADR |
| Cambios en modelos ML o prompts de IA | AILOG | `risk_level: medium+`, revisión humana requerida |
| Integración con servicio externo | AILOG | - |
| Adición/eliminación/actualización de dependencias críticas de seguridad | AILOG | Revisión humana requerida |
| Cambios que afectan el ciclo de vida del sistema de IA (despliegue, retirada) | AILOG + ADR | Revisión humana requerida |
| Cambios en instrumentación OTel (spans, atributos, pipeline) | AILOG | Tag `observabilidad`, ver §9 |

### PROHIBIDO - No documentar

- Credenciales, tokens, API keys
- Información personal identificable
- Secretos de cualquier tipo

### OPCIONAL - No requiere documento

- Cambios de formato (espacios, indentación)
- Correcciones de erratas
- Comentarios de código
- Cambios menores de estilo

---

## 3. Límites de Autonomía

### Crear Libremente

| Tipo | Descripción |
|------|-------------|
| AILOG | Logs de acciones realizadas |
| AIDEC | Decisiones técnicas tomadas |

### Crear Borrador → Requiere Aprobación Humana

| Tipo | Descripción |
|------|-------------|
| ETH | Revisiones éticas |
| ADR | Decisiones arquitectónicas |

### Proponer → Requiere Validación Humana

| Tipo | Descripción |
|------|-------------|
| REQ | Requisitos del sistema |
| TES | Planes de prueba |

### Crear Borrador → Requiere Aprobación Humana (tipos nuevos)

| Tipo | Descripción |
|------|-------------|
| SEC | Evaluaciones de seguridad (`review_required: true` siempre) |
| MCARD | Tarjetas de modelo/sistema (`review_required: true` siempre) |
| DPIA | Evaluaciones de impacto en protección de datos (`review_required: true` siempre) |

### Crear Libremente (tipos nuevos)

| Tipo | Descripción |
|------|-------------|
| SBOM | Lista de materiales de software (inventario factual) |

### Solo Identificar → Humano Prioriza

| Tipo | Descripción |
|------|-------------|
| TDE | Deuda técnica |
| INC | Conclusiones de incidentes |

---

## 4. Cuándo Solicitar Revisión Humana

Marcar `review_required: true` cuando:

1. **Baja confianza**: `confidence: low`
2. **Alto riesgo**: `risk_level: high | critical`
3. **Decisiones de seguridad**: Cualquier cambio en auth/authz
4. **Cambios irreversibles**: Migraciones, eliminaciones
5. **Impacto en usuarios**: Cambios que afectan UX
6. **Preocupaciones éticas**: Privacidad, sesgo, accesibilidad
7. **Cambios en modelos ML**: Cambios en parámetros del modelo, arquitectura o datos de entrenamiento
8. **Cambios en prompts de IA**: Modificaciones a prompts o instrucciones de agentes
9. **Dependencias críticas de seguridad**: Adición, eliminación o actualización de paquetes sensibles a la seguridad
10. **Cambios en ciclo de vida de IA**: Despliegue, retirada o cambios de versión mayor de sistemas de IA

---

## 5. Formato de Documentos

### Usar Plantillas

Antes de crear un documento, cargar la plantilla correspondiente:

```
.devtrail/templates/TEMPLATE-[TIPO].md
```

### Convención de Nomenclatura

```
[TIPO]-[YYYY-MM-DD]-[NNN]-[descripcion].md
```

### Ubicación

| Tipo | Carpeta |
|------|---------|
| AILOG | `.devtrail/07-ai-audit/agent-logs/` |
| AIDEC | `.devtrail/07-ai-audit/decisions/` |
| ETH | `.devtrail/07-ai-audit/ethical-reviews/` |
| ADR | `.devtrail/02-design/decisions/` |
| REQ | `.devtrail/01-requirements/` |
| TES | `.devtrail/04-testing/` |
| INC | `.devtrail/05-operations/incidents/` |
| TDE | `.devtrail/06-evolution/technical-debt/` |
| SEC | `.devtrail/08-security/` |
| MCARD | `.devtrail/09-ai-models/` |
| SBOM | `.devtrail/07-ai-audit/` |
| DPIA | `.devtrail/07-ai-audit/ethical-reviews/` |

### Tags y Related

Al poblar los campos `tags` y `related` en el frontmatter:

**Tags:**
- Usar palabras clave en kebab-case: `sqlite`, `api-design`, `gnome-integration`
- 3 a 8 tags por documento describiendo tema, tecnología o componente
- Los tags habilitan búsqueda y categorización en `devtrail explore`

**Related:**
- Referenciar únicamente otros **documentos DevTrail** — usar el nombre de archivo con extensión `.md`
- Si el documento está en un subdirectorio dentro de `.devtrail/`, incluir la ruta relativa: `07-ai-audit/agent-logs/daemon/AILOG-2026-02-03-001-archivo.md`
- Si el documento está en el mismo directorio, el nombre de archivo es suficiente
- **No** colocar IDs de tareas (T001, US3), números de issues ni URLs externas en `related` — esos van en el cuerpo del documento

---

## 6. Comunicación con Humanos

### Ser Transparente

- Explicar el razonamiento detrás de las decisiones
- Documentar alternativas consideradas
- Admitir incertidumbre cuando existe

### Ser Conciso

- Ir al grano
- Evitar jerga innecesaria
- Usar listas y tablas cuando sea apropiado

### Ser Proactivo

- Identificar riesgos potenciales
- Sugerir mejoras cuando sean evidentes
- Alertar sobre deuda técnica

---

## 7. Manejo de Errores

Si el agente comete un error:

1. **Documentar** el error en un AILOG
2. **Explicar** qué salió mal
3. **Proponer** corrección
4. **Marcar** `review_required: true`

---

## 8. Actualización de Documentos

### Crear Nuevo vs Actualizar

| Situación | Acción |
|-----------|--------|
| Corrección menor | Actualizar documento existente |
| Cambio significativo | Crear nuevo documento |
| Documento obsoleto | Marcar como `deprecated` |
| Reemplazo completo | Crear nuevo + marcar anterior como `superseded` |

### Al Actualizar

- Actualizar el campo `updated` en frontmatter
- Agregar nota en la sección de historial si existe
- Mantener consistencia con documentos relacionados

---

## 9. Observabilidad (OpenTelemetry)

Cuando se trabaja en proyectos que usan OpenTelemetry:

### Reglas

- **No** capturar PII, tokens o secretos en atributos o logs de OTel
- **Registrar** cambios en el pipeline de instrumentación (nuevos spans, atributos modificados, configuración del Collector) en AILOG con tag `observabilidad`
- **Crear** AIDEC o ADR al adoptar OTel en proyectos distribuidos — documentar la decisión de adopción y selección de backend
- **Establecer** `observability_scope` en el frontmatter cuando el cambio involucra instrumentación OTel

### Disparadores de Documentación

| Cambio | Documento | Adicional |
|--------|----------|-----------|
| Nuevos spans o atributos modificados | AILOG | Tag `observabilidad` |
| Selección de backend OTel | AIDEC o ADR | Si sistema distribuido |
| Configuración del pipeline del Collector | AILOG | Tag `observabilidad` |
| Cambios en estrategia de muestreo | AIDEC | Documentar justificación |
| Requisitos de observabilidad | REQ | Usar sección de Requisitos de Observabilidad |
| Pruebas de propagación de trazas | TES | Usar sección de Pruebas de Observabilidad |
| Incidente con evidencia de trazas | INC | Incluir trace_id/span_id en la línea temporal |
| Deuda de instrumentación | TDE | Tag `observabilidad` |

---

## 10. Diagramas de Arquitectura (Modelo C4)

Al crear documentos ADR que involucren cambios arquitectónicos:

- **Incluir** un diagrama C4 con Mermaid al nivel apropiado
- **Usar** `C4Context` para decisiones a nivel de sistema (quién usa el sistema, dependencias externas)
- **Usar** `C4Container` para decisiones a nivel de servicio/contenedor (aplicaciones, bases de datos, colas de mensajes)
- **Usar** `C4Component` para decisiones de módulos internos (componentes dentro de un servicio)
- **Ver** `00-governance/C4-DIAGRAM-GUIDE.md` para referencia de sintaxis y ejemplos

> Los diagramas son opcionales para decisiones menores. Usarlos cuando la decisión cambie fronteras del sistema, introduzca nuevos servicios o modifique comunicación entre servicios.

---

## 11. Seguimiento de Especificaciones de API

Cuando un cambio modifica endpoints de API:

- **Verificar** que la especificación OpenAPI o AsyncAPI correspondiente esté actualizada
- **Referenciar** la ruta del spec en el AILOG o ADR usando el campo `api_spec_path` (en REQ) o `api_changes` (en ADR)
- **Documentar** cambios de API que rompen compatibilidad en un ADR con `risk_level: high`

---

*DevTrail v4.1.1 | [Strange Days Tech](https://strangedays.tech)*
