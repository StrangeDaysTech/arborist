---
id: AIDEC-2026-03-28-001
title: Use tree-sitter as the single parsing foundation for all languages
status: accepted
created: 2026-03-28
agent: claude-code-v1.0
confidence: high
review_required: false
risk_level: low
eu_ai_act_risk: not_applicable
nist_genai_risks: []
iso_42001_clause: [8]
tags: [tree-sitter, parsing, architecture, rust, multi-language]
related:
  - AILOG-2026-03-28-001-implement-mvp-core-metrics-engine.md
---

# AIDEC: Use tree-sitter as the single parsing foundation for all languages

## Context

The arborist library needs to parse source code in 10 programming languages (Rust, Python, JavaScript, TypeScript, Java, C#, C++, C, Go, PHP) to compute complexity metrics. A parsing strategy was needed that balances accuracy, maintainability, and performance across all target languages.

## Problem

How to parse 10 different programming languages accurately enough to extract function boundaries, control flow structures, nesting levels, and boolean operators — without maintaining 10 separate parser implementations.

## Alternatives Considered

### Alternative 1: tree-sitter (single parser library)

**Description**: Use tree-sitter 0.25 with language-specific grammar crates. Each language gets a grammar crate that produces a concrete syntax tree (CST). A shared `LanguageProfile` trait maps language-specific node types to generic metric concepts.

**Pros**:
- Single parsing API for all languages
- Grammars maintained by tree-sitter community (battle-tested in editors like Helix, Neovim, Zed)
- Error-tolerant parsing (produces partial ASTs for malformed code)
- Incremental parsing support (future optimization potential)
- No `unsafe` code needed in the library itself (tree-sitter handles FFI internally)
- C ABI grammars work across platforms

**Cons**:
- Grammar crates add compile-time dependencies (~10 C libraries)
- CST is more verbose than AST (requires careful node type mapping)
- tree-sitter grammars may lag behind bleeding-edge language features
- Build times increase with each grammar crate

### Alternative 2: syn (Rust) + language-specific parsers

**Description**: Use `syn` for Rust parsing and find equivalent parser libraries for each language (e.g., `swc` for JavaScript/TypeScript, `tree-sitter` for others where no native Rust parser exists).

**Pros**:
- `syn` provides the most accurate Rust AST
- Language-specific parsers may capture more semantic information

**Cons**:
- Inconsistent APIs across parser libraries
- Each parser requires its own walker implementation
- Many languages lack mature Rust-native parsers
- Significantly more maintenance burden
- Would still need tree-sitter for most languages anyway

### Alternative 3: Regex-based heuristic parsing

**Description**: Use regular expressions and line-by-line heuristics to identify function boundaries and control flow structures.

**Pros**:
- Zero external dependencies
- Fast compilation
- Simple implementation for basic cases

**Cons**:
- Fundamentally unreliable for nested structures
- Cannot handle multi-line constructs, string literals containing code patterns, or macros
- Would produce incorrect metrics for any non-trivial code
- Violates project constitution Principle III (Tree-sitter as Single Parser)

## Decision

**Chosen**: Alternative 1 - tree-sitter (single parser library)

**Justification**: tree-sitter provides the best balance of accuracy, consistency, and maintainability. A single `LanguageProfile` trait maps each language's grammar nodes to generic metric concepts, keeping the core metric calculators completely language-agnostic. The error-tolerant parsing ensures graceful handling of malformed code. The grammar ecosystem is mature and actively maintained. This aligns with the project constitution Principle III ("Tree-sitter as Single Parser") and Principle IV ("One Language = One Trait").

## Consequences

### Positive
- Adding a new language requires only implementing `LanguageProfile` (~50-75 lines) — no changes to core metrics or walker
- Consistent metric computation across all languages
- Error-tolerant parsing means no crashes on malformed input
- Compile-time feature flags allow users to include only needed language grammars

### Negative
- Build times increase with each grammar crate (mitigated by feature flags)
- CST node type mappings require careful research per language (documented in research.md)
- Grammar updates may occasionally change node type names (low risk, grammars are stable)

### Risks
- tree-sitter 0.25 is relatively new — potential for breaking changes in minor versions. Mitigated by pinning the dependency version in Cargo.toml.

## Implementation

- `Cargo.toml`: tree-sitter 0.25 + 10 grammar crates as optional dependencies
- `src/languages/mod.rs`: `LanguageProfile` trait with methods mapping node types
- `src/languages/*.rs`: One profile per language implementing the trait
- `src/walker.rs`: Generic AST walker operating on `dyn LanguageProfile`
- `src/metrics/*.rs`: Language-agnostic metric calculators

## References

- [tree-sitter documentation](https://tree-sitter.github.io/tree-sitter/)
- SonarSource cognitive complexity paper (referenced in `specs/001-code-metrics-library/research.md`)
- `specs/001-code-metrics-library/research.md` — R1 through R7 decisions
- `specs/001-code-metrics-library/plan.md` — Constitution check and architecture

---

<!-- Template: DevTrail | https://strangedays.tech -->
