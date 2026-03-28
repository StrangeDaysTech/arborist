# Implementation Plan: Code Metrics Analysis Library (Arborist)

**Branch**: `001-code-metrics-library` | **Date**: 2026-03-27 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-code-metrics-library/spec.md`

## Summary

Arborist is an independent Rust crate that computes cognitive complexity, cyclomatic complexity, and SLOC for source code across 10 programming languages. It uses tree-sitter as the single parsing foundation. Each language is added by implementing a `LanguageProfile` trait, and languages are gated by compile-time feature flags. The library exposes two primary entry points: `analyze_file()` (auto-detects language from extension) and `analyze_source()` (explicit language).

## Technical Context

**Language/Version**: Rust (edition 2024, MSRV to be determined by tree-sitter 0.25 requirements)
**Primary Dependencies**: tree-sitter 0.25, serde (serialization), 10 tree-sitter grammar crates (see research.md)
**Storage**: N/A (pure computation library, reads files via `std::fs`)
**Testing**: `cargo test`, fixture-driven (TDD per constitution)
**Target Platform**: Any platform supported by Rust + tree-sitter (Linux, macOS, Windows, WASM potential future)
**Project Type**: Library (crate published on crates.io)
**Performance Goals**: <100ms for files under 1000 lines on standard hardware
**Constraints**: No `unsafe` code (per constitution), no runtime dependencies beyond tree-sitter, UTF-8 only
**Scale/Scope**: Single-file analysis at a time; 10 Tier 1 languages at v0.1.0

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Library-First, Always Embeddable | PASS | Two public functions (`analyze_file`, `analyze_source`) + types. No CLI, no framework coupling. |
| II. Correctness Over Coverage | PASS | 10 languages with 5+ fixtures each. SonarSource paper as reference. |
| III. Tree-sitter as Single Parser | PASS | All parsing via tree-sitter. No regex, no ad-hoc parsers. |
| IV. One Language = One Trait | PASS | `LanguageProfile` trait per language. Walker and metrics are generic. |
| V. Feature Flags for Modularity | PASS | Each language is an optional feature. `default` = 6 most common. `all` = 10 Tier 1. |
| VI. Test-First, Fixture-Driven | PASS | TDD workflow: fixtures with known values first, then implementation. |
| VII. Semver Strict | PASS | v0.1.0 initial release. Public API = `lib.rs` + `types.rs`. |

**Quality Gates**:
- `cargo clippy -- -D warnings`: Enforced in CI
- `cargo test --all-features`: Enforced in CI
- 6 fixtures per LanguageProfile: Required for each of 10 languages (simple, nesting, booleans, else-if, closures, recursion)
- Doc comments with examples on all public items: Required
- No `unsafe`: Enforced via `#![forbid(unsafe_code)]`

**Gate result**: ALL PASS — proceed to Phase 0.

## Project Structure

### Documentation (this feature)

```text
specs/001-code-metrics-library/
├─�� plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (public API contract)
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
src/
├── lib.rs                 # Public API: analyze_file(), analyze_source(), re-exports
├── types.rs               # FunctionMetrics, FileReport, Language, AnalysisConfig
├── error.rs               # ArboristError enum with variants per FR-013
├── walker.rs              # Generic AST walker: extracts functions, computes metrics
├─��� metrics/
│   ├── mod.rs             # Metric computation orchestration
│   ├── cognitive.rs       # Cognitive complexity (SonarSource algorithm)
│   ├── cyclomatic.rs      # Cyclomatic complexity
│   └── loc.rs             # SLOC counting
└── languages/
    ├── mod.rs             # LanguageProfile trait + language detection by extension
    ├── rust.rs            # Rust profile
    ├── python.rs          # Python profile
    ├── javascript.rs      # JavaScript + JSX profile
    ├── typescript.rs      # TypeScript + TSX profile
    ├── java.rs            # Java profile
    ├── csharp.rs          # C# profile
    ├── cpp.rs             # C++ profile
    ├── c.rs               # C profile
    ├── go.rs              # Go profile
    └── php.rs             # PHP profile

tests/
├── fixtures/              # Source files with known complexity values
│   ├── rust/              # 5+ .rs files per fixture category
│   ├── python/            # 5+ .py files
│   ├── javascript/        # 5+ .js files
│   ├── typescript/        # 5+ .ts/.tsx files
│   ├── java/              # 5+ .java files
│   ├── csharp/            # 5+ .cs files
│   ├── cpp/               # 5+ .cpp files
│   ├── c/                 # 5+ .c files
│   ├── go/                # 5+ .go files
│   └── php/               # 5+ .php files
└���─ integration/           # Cross-cutting integration tests

Cargo.toml                 # Feature flags, dependencies
LICENSE-MIT
LICENSE-APACHE
```

**Structure Decision**: Single-crate library following idiomatic Rust layout. `src/` contains the library code organized by responsibility (types, metrics, languages, walker). `tests/` contains fixture files and integration tests. Each language module is behind a feature flag. No workspace, no binary targets.

## Complexity Tracking

No constitution violations to justify. Design is fully aligned with all 7 principles.
