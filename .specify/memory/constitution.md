# Arborist Constitution

## Core Principles

### I. Library-First, Always Embeddable
Arborist existe como crate independiente publicado en crates.io. Toda funcionalidad se expone como API de librería (`analyze_file()`, `analyze_source()`). Ningún consumidor (StrayMark, CI tools, linters) debe requerir más que una línea en `Cargo.toml` para integrarlo. No se acopla a frameworks, CLIs específicos, ni runtimes externos.

### II. Correctness Over Coverage
La complejidad cognitiva es la métrica principal y debe implementarse fielmente según el paper de SonarSource (G. Ann Campbell, 2017). Cada lenguaje soportado debe tener fixtures con valores de complejidad verificados contra referencias públicas. Un lenguaje sin tests de validación no se publica — es preferible soportar 5 lenguajes correctos que 20 aproximados.

### III. Tree-sitter as Single Parser Foundation
Todo el parsing se realiza a través de tree-sitter. No se introducen parsers ad-hoc, regex sobre código fuente, ni dependencias de parsing alternativas. Esto garantiza consistencia, mantenibilidad y que agregar un lenguaje nuevo sea implementar un trait, no construir un parser.

### IV. One Language = One Trait Implementation
Cada lenguaje se agrega implementando `LanguageProfile`. El walker de AST y el cálculo de métricas son genéricos. Los perfiles de lenguaje son declarativos: mapean nodos del AST a conceptos de control de flujo. Si agregar un lenguaje requiere modificar el walker o las métricas core, el diseño tiene un defecto que debe corregirse primero.

### V. Feature Flags for Modularity
Cada lenguaje es un feature flag que controla su grammar como dependencia opcional. El feature `default` incluye los lenguajes Tier 1 más usados. Los consumidores eligen exactamente lo que necesitan — nadie paga el costo de compilación de 20+ grammars si solo necesita Python y Rust.

### VI. Test-First, Fixture-Driven
TDD obligatorio. Para cada lenguaje:
1. Se crean fixtures de código con complejidad conocida y calculada manualmente
2. Se escriben tests que validan contra esos valores esperados
3. Se implementa el `LanguageProfile`
4. Los tests pasan

Los fixtures sirven como documentación viva del comportamiento esperado.

### VII. Semver Estricto, Evolución Independiente
Arborist sigue semver sin excepciones. La API pública (`pub` items en `lib.rs` y `types.rs`) es el contrato. Breaking changes solo en MAJOR. El versionado es independiente de StrayMark o cualquier consumidor. Los lenguajes Tier 2/3 se agregan en releases MINOR.

## Boundaries

### Qué es Arborist
- Una librería de análisis estático de métricas de complejidad de código
- Multilenguaje, basada en tree-sitter, embeddable en cualquier proyecto Rust
- Enfocada en: complejidad cognitiva, complejidad ciclomática, SLOC

### Qué NO es Arborist
- No es un linter ni sugiere correcciones
- No es un formateador de código
- No ejecuta ni interpreta código fuente
- No hace análisis de seguridad ni detección de vulnerabilidades
- No tiene CLI propio — los consumidores construyen el suyo
- No tiene opinión sobre umbrales — los reporta, no los impone

## Quality Gates

- **Compilación limpia:** `cargo clippy -- -D warnings` sin excepciones
- **Tests passing:** `cargo test --all-features` pasa en CI antes de merge
- **Cobertura de fixtures:** Cada `LanguageProfile` tiene al menos 5 fixtures que cubren: funciones simples, anidamiento profundo, operadores booleanos mixtos, else-if chains, y closures/lambdas
- **Documentación:** Toda función pública tiene doc comments con ejemplos ejecutables (`cargo test --doc`)
- **Sin `unsafe`:** Salvo justificación explícita documentada en ADR, el crate es 100% safe Rust

## Governance

Esta constitución es el documento rector de todas las decisiones de diseño e implementación de Arborist. Las enmiendas requieren:
1. Documentación del cambio propuesto (AIDEC o ADR)
2. Justificación de por qué la constitución actual es insuficiente
3. Actualización de este documento con fecha de enmienda

Todo PR y review debe verificar cumplimiento con estos principios. En caso de conflicto entre velocidad y correctness, gana correctness.

**Version**: 1.0.0 | **Ratified**: 2026-03-27 | **Last Amended**: 2026-03-27
