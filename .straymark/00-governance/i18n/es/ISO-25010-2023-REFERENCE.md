# ISO/IEC 25010:2023 — Referencia de calidad de software

> **Estándar**: ISO/IEC 25010:2023 — Systems and software Quality Requirements and Evaluation (SQuaRE) — Modelo de calidad del producto
> **Reemplaza**: ISO/IEC 25010:2011
> **Propósito**: Documento de referencia para las plantillas StrayMark (ADR, REQ) que evalúan características de calidad de software.

---

## Características de calidad (2023 vs 2011)

La revisión 2023 actualiza el modelo de 8 a **9 características**, con renombres y reestructuraciones importantes.

| # | Característica (2023) | Nombre anterior (2011) | Cambio |
|---|----------------------|------------------------|--------|
| 1 | **Adecuación funcional** | Adecuación funcional | Sin cambios |
| 2 | **Eficiencia de desempeño** | Eficiencia de desempeño | Sin cambios |
| 3 | **Compatibilidad** | Compatibilidad | Sin cambios |
| 4 | **Capacidad de interacción** | Usabilidad | Renombrada |
| 5 | **Confiabilidad** | Confiabilidad | Reestructurada |
| 6 | **Seguridad** | Seguridad | Nueva sub: Resistencia |
| 7 | **Mantenibilidad** | Mantenibilidad | Sin cambios |
| 8 | **Flexibilidad** | Portabilidad | Renombrada |
| 9 | **Seguridad funcional (Safety)** | *(nueva)* | Característica nueva |

---

## Características y sub-características detalladas

### 1. Adecuación funcional

Grado en el que un producto provee funciones que cumplen las necesidades declaradas e implícitas.

| Sub-característica | Descripción |
|--------------------|-------------|
| Completitud funcional | Grado en el que el conjunto de funciones cubre todas las tareas y objetivos de usuario especificados |
| Corrección funcional | Grado en el que un producto provee resultados correctos con la precisión requerida |
| Pertinencia funcional | Grado en el que las funciones facilitan el cumplimiento de tareas y objetivos especificados |

### 2. Eficiencia de desempeño

Desempeño relativo a la cantidad de recursos usados bajo condiciones declaradas.

| Sub-característica | Descripción |
|--------------------|-------------|
| Comportamiento temporal | Grado en el que los tiempos de respuesta, procesamiento y throughput cumplen los requisitos |
| Utilización de recursos | Grado en el que las cantidades y tipos de recursos usados cumplen los requisitos |
| Capacidad | Grado en el que los límites máximos de un parámetro del producto cumplen los requisitos |

### 3. Compatibilidad

Grado en el que un producto puede intercambiar información y realizar sus funciones requeridas compartiendo el mismo entorno.

| Sub-característica | Descripción |
|--------------------|-------------|
| Coexistencia | Grado en el que un producto realiza sus funciones eficientemente compartiendo un entorno y recursos comunes con otros productos |
| Interoperabilidad | Grado en el que dos o más sistemas pueden intercambiar información y usarla |

### 4. Capacidad de interacción *(renombrada desde Usabilidad)*

Grado en el que un producto puede ser usado por usuarios especificados para lograr objetivos especificados con eficacia, eficiencia y satisfacción.

| Sub-característica | Descripción | Cambio respecto a 2011 |
|--------------------|-------------|------------------------|
| Reconocibilidad de pertinencia | Grado en el que los usuarios pueden reconocer si un producto es apropiado para sus necesidades | Sin cambios |
| Aprendibilidad | Grado en el que un producto puede ser usado para alcanzar objetivos de aprendizaje especificados con eficacia, eficiencia, ausencia de riesgo y satisfacción | Sin cambios |
| Operabilidad | Grado en el que un producto tiene atributos que lo hacen fácil de operar y controlar | Sin cambios |
| Protección frente a errores de usuario | Grado en el que un producto protege a los usuarios contra cometer errores | Sin cambios |
| Compromiso del usuario | Grado en el que un producto provee una experiencia de interacción atractiva y motivadora | Reemplaza a "Estética de la interfaz de usuario" |
| Inclusividad | Grado en el que un producto puede ser usado por personas con el rango más amplio de características y capacidades | Separada de Accesibilidad |
| Asistencia al usuario | Grado en el que un producto provee ayuda y guía apropiadas a los usuarios | Separada de Accesibilidad |
| Auto-descripción | Grado en el que un producto presenta información que hace sus capacidades y uso inmediatamente obvios | Nueva |

### 5. Confiabilidad

Grado en el que un sistema realiza las funciones especificadas bajo condiciones especificadas durante un periodo de tiempo especificado.

| Sub-característica | Descripción | Cambio respecto a 2011 |
|--------------------|-------------|------------------------|
| Ausencia de fallos | Grado en el que un sistema opera sin fallos bajo condiciones normales | Reemplaza a "Madurez" |
| Disponibilidad | Grado en el que un sistema está operativo y accesible cuando se requiere para uso | Sin cambios |
| Tolerancia a fallos | Grado en el que un sistema opera como se pretende a pesar de fallos de hardware o software | Sin cambios |
| Recuperabilidad | Grado en el que un producto puede recuperar datos y reestablecer el estado deseado después de una interrupción o fallo | Sin cambios |

### 6. Seguridad

Grado en el que un producto protege información y datos.

| Sub-característica | Descripción | Cambio respecto a 2011 |
|--------------------|-------------|------------------------|
| Confidencialidad | Grado en el que los datos son accesibles sólo para quienes están autorizados a tener acceso | Sin cambios |
| Integridad | Grado en el que un sistema previene el acceso no autorizado o la modificación de datos | Sin cambios |
| No repudio | Grado en el que se puede probar que acciones o eventos ocurrieron | Sin cambios |
| Trazabilidad (accountability) | Grado en el que las acciones de una entidad pueden trazarse unívocamente a la entidad | Sin cambios |
| Autenticidad | Grado en el que la identidad de un sujeto o recurso puede probarse como la declarada | Sin cambios |
| Resistencia | Grado en el que un producto resiste ataques de actores no autorizados o maliciosos | Nueva |

### 7. Mantenibilidad

Grado de eficacia y eficiencia con la que un producto puede ser modificado.

| Sub-característica | Descripción |
|--------------------|-------------|
| Modularidad | Grado en el que un sistema está compuesto por componentes discretos de manera que un cambio en uno tiene impacto mínimo en los otros |
| Reusabilidad | Grado en el que un activo puede ser usado en más de un sistema o en la construcción de otros activos |
| Analizabilidad | Grado de eficacia y eficiencia con la que es posible evaluar el impacto de un cambio |
| Modificabilidad | Grado en el que un producto puede ser modificado eficaz y eficientemente sin introducir defectos |
| Testabilidad | Grado de eficacia y eficiencia con la que se pueden establecer criterios de prueba y realizar pruebas |

### 8. Flexibilidad *(renombrada desde Portabilidad)*

Grado en el que un producto puede ser adaptado a entornos de hardware, software o uso diferentes o cambiantes.

| Sub-característica | Descripción | Cambio respecto a 2011 |
|--------------------|-------------|------------------------|
| Adaptabilidad | Grado en el que un producto puede ser adaptado a entornos diferentes o cambiantes | Sin cambios |
| Instalabilidad | Grado de eficacia y eficiencia con la que un producto puede ser instalado o desinstalado exitosamente | Sin cambios |
| Reemplazabilidad | Grado en el que un producto puede reemplazar a otro producto especificado para el mismo propósito en el mismo entorno | Sin cambios |
| Escalabilidad | Grado en el que un producto puede manejar cargas de trabajo crecientes o decrecientes | Nueva |

### 9. Seguridad funcional (Safety) *(característica nueva)*

Grado en el que un producto logra niveles aceptables de riesgo para personas, negocio, software, propiedad o el entorno.

| Sub-característica | Descripción |
|--------------------|-------------|
| Restricción operacional | Grado en el que un producto restringe su operación dentro de parámetros o estados seguros |
| Identificación de riesgos | Grado en el que un producto identifica riesgos que podrían afectar la seguridad funcional |
| Fail safe | Grado en el que un producto automáticamente se ubica en un modo de operación seguro, o vuelve a una condición segura ante una falla |
| Advertencia de peligros | Grado en el que un producto provee advertencias de peligros |
| Integración segura | Grado en el que un producto puede mantener seguridad funcional durante y después de la integración con uno o más componentes |

---

## Uso en StrayMark

- **TEMPLATE-REQ.md**: la sección de Requisitos No Funcionales usa estas 9 características como categorías
- **TEMPLATE-ADR.md**: la sección de Consecuencias evalúa decisiones contra las características de calidad relevantes
- **TEMPLATE-TES.md**: la planificación de pruebas considera las características de calidad como dimensiones de cobertura

## Cambios clave a recordar

Al revisar o crear documentos StrayMark:

1. Usar **"Capacidad de interacción"** en lugar de "Usabilidad"
2. Usar **"Flexibilidad"** en lugar de "Portabilidad"
3. Considerar siempre **"Seguridad funcional (Safety)"** como dimensión de calidad, especialmente para sistemas IA
4. **"Resistencia"** (bajo Seguridad) es relevante para evaluaciones de amenazas
5. **"Escalabilidad"** (bajo Flexibilidad) es ahora una sub-característica formal

<!-- Reference: StrayMark | https://strangedays.tech -->
