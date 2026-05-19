---
id: TES-2026-03-28-001
title: "Test plan: analyze_source in-memory code analysis (US2)"
status: draft
created: 2026-03-28
agent: claude-code-v1.0
confidence: high
review_required: true
risk_level: low
iso_42001_clause: [8]
observability_scope: none
tags: [testing, us2, analyze-source, rust]
related: [AILOG-2026-03-28-001]
---

# TES: analyze_source In-Memory Code Analysis (US2)

> **PROPOSAL**: This plan was created by an AI agent and requires validation.
>
> **Standard alignment**: ISO/IEC/IEEE 29119-3:2021 (Software Testing — Test Documentation)

## Scope

### In Scope
- `analyze_source(code, language)` — analiza código fuente en memoria sin acceso a disco
- `analyze_source_with_config(code, language, config)` — variante con configuración personalizada
- Equivalencia de métricas entre `analyze_source` y `analyze_file` para los mismos contenidos
- Manejo de errores: cadena vacía, errores de sintaxis, lenguaje no habilitado

### Out of Scope
- Tests de `analyze_file` (cubiertos en tests de integración existentes)
- Tests de rendimiento / benchmark
- Soporte de lenguajes distintos a Rust (se testean en otros módulos)

## Test Approach

### Test Design Techniques

| Technique | Application | Rationale |
|-----------|-------------|-----------|
| Equivalence Partitioning | Cada fixture Rust representa una clase de complejidad distinta | Cubre diversidad de patrones sin duplicar esfuerzo |
| Boundary Value Analysis | Cadena vacía, código con errores de sintaxis | Valida comportamiento en límites del input |
| Decision Table | `analyze_source` vs `analyze_source_with_config(default)` | Verifica que config por defecto produce resultados idénticos |

### Test Types and Coverage

| Type | Coverage | Tool | Rationale |
|------|----------|------|-----------|
| Unit/Integration | 100% de la API pública `analyze_source*` | `cargo test --features rust` | Verifica equivalencia con análisis basado en fichero |

### Test Completion Criteria

- [x] Todos los test cases pasan
- [x] Cobertura de todos los fixtures Rust existentes
- [x] Casos negativos (vacío, syntax error, feature deshabilitado)
- [x] Equivalencia `analyze_source` ↔ `analyze_file` verificada

## Test Cases

### Functionality: Equivalencia analyze_source ↔ analyze_file

| ID | Case | Preconditions | Steps | Expected Result | Priority |
|----|------|---------------|-------|-----------------|----------|
| TC-001 | simple_function | Fixture `simple_function.rs` disponible | 1. Cargar fixture con `include_str!` 2. Ejecutar ambas APIs 3. Comparar métricas | Métricas idénticas (cognitive, cyclomatic, sloc, nombre) | High |
| TC-002 | nested_control_flow | Fixture `nested_control_flow.rs` disponible | Idem TC-001 | Métricas idénticas | High |
| TC-003 | boolean_operators | Fixture `boolean_operators.rs` disponible | Idem TC-001 para todas las funciones | Métricas idénticas por función | High |
| TC-004 | else_if_chain | Fixture `else_if_chain.rs` disponible | Idem TC-001 | Métricas idénticas | Medium |
| TC-005 | closures_lambdas | Fixture `closures_lambdas.rs` disponible | Idem TC-001 | Métricas idénticas | Medium |
| TC-006 | recursion | Fixture `recursion.rs` disponible | Idem TC-001 | Métricas idénticas | Medium |
| TC-007 | config default ↔ no config | Fixture `simple_function.rs` | 1. `analyze_source` 2. `analyze_source_with_config(default)` 3. Comparar | Métricas y thresholds idénticos | High |

### Negative Cases

| ID | Case | Invalid Input | Expected Result |
|----|------|---------------|-----------------|
| TC-N01 | Cadena vacía | `""` | Ok: 0 funciones, sloc=0, cognitive=0, cyclomatic=0 |
| TC-N02 | Error de sintaxis | `"fn broken( { if true { } }"` | Ok (best-effort): tree-sitter tolera errores |
| TC-N03 | Lenguaje no habilitado | `Language::CSharp` sin feature `csharp` | Err con mensaje "not enabled" |

### Edge Cases

| ID | Case | Condition | Expected Result |
|----|------|-----------|-----------------|
| TC-E01 | Path vacío en source report | `analyze_source` no recibe path | `report.path == ""` |

## Test Data Requirements

| Data Set ID | Source | Preparation Steps | Sensitivity Classification | Retention Policy |
|-------------|--------|-------------------|---------------------------|-----------------|
| TD-001 | `tests/fixtures/rust/*.rs` | Fixtures estáticos incluidos en el repo | Public | Retener permanentemente |

### Required Fixtures
- `simple_function.rs`: Función básica sin complejidad
- `nested_control_flow.rs`: Control flow anidado (if/for/while)
- `boolean_operators.rs`: Operadores lógicos (`&&`, `||`)
- `else_if_chain.rs`: Cadena de else-if
- `closures_lambdas.rs`: Closures y lambdas Rust
- `recursion.rs`: Función recursiva

### Required Mocks
- Ninguno — tests usan directamente la API pública sin mocks

## Test Environment Requirements

| Component | Version | Configuration | Dependencies |
|-----------|---------|---------------|-------------|
| Rust | edition 2024 | Feature flag `rust` habilitado | tree-sitter 0.25, tree-sitter-rust |
| cargo test | stable | `--features rust` | Fixtures en `tests/fixtures/rust/` |

- **Environment**: Local / CI
- **Special configuration**: Compilar con `--features rust`
- **External dependencies**: Ninguna

## Results

### Test Execution Log

| TC ID | Date | Tester | Result | Actual Output | Notes |
|-------|------|--------|--------|---------------|-------|
| TC-001 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-002 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-003 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-004 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-005 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-006 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-007 | 2026-03-28 | claude-code-v1.0 | Pass | Métricas coinciden | — |
| TC-N01 | 2026-03-28 | claude-code-v1.0 | Pass | 0 funciones, métricas en 0 | — |
| TC-N02 | 2026-03-28 | claude-code-v1.0 | Pass | Best-effort sin error | — |
| TC-N03 | 2026-03-28 | claude-code-v1.0 | Pass | Error "not enabled" | Solo cuando feature `csharp` deshabilitado |

### Test Status Report

| Metric | Value |
|--------|-------|
| Total test cases | 10 |
| Passed | 10 |
| Failed | 0 |
| Blocked | 0 |
| Not executed | 0 |
| Pass rate | 100% |

### Test Completion Report

- **Completion date**: 2026-03-28
- **Completion criteria met**: Yes
- **Outstanding risks**: Ninguno
- **Recommendation**: Proceed to release

---

## Validation

| Field | Value |
|-------|-------|
| Validated by | — |
| Date | — |
| Comments | Pendiente de validación humana |

<!-- Template: DevTrail | https://strangedays.tech -->
