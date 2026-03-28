# Research: Code Metrics Analysis Library (Arborist)

**Branch**: `001-code-metrics-library` | **Date**: 2026-03-27

## R1: tree-sitter 0.25 Compatibility with Grammar Crates

**Decision**: Use tree-sitter 0.25 as the parsing foundation with the following verified grammar crates.

**Rationale**: All 10 Tier 1 grammar crates have been verified to compile together with tree-sitter 0.25 on Rust 1.93+. This was validated in the feasibility analysis prior to specification.

**Verified crate versions**:

| Language | Crate | Version |
|----------|-------|---------|
| Rust | `tree-sitter-rust` | 0.23 |
| Python | `tree-sitter-python` | 0.23 |
| JavaScript | `tree-sitter-javascript` | 0.23 |
| TypeScript | `tree-sitter-typescript` | 0.23 |
| Java | `tree-sitter-java` | 0.23 |
| C# | `tree-sitter-c-sharp` | 0.23 |
| C++ | `tree-sitter-cpp` | 0.23 |
| C | `tree-sitter-c` | 0.23 |
| Go | `tree-sitter-go` | 0.23 |
| PHP | `tree-sitter-php` | 0.23 |

**Alternatives considered**:
- `rust-code-analysis` (Mozilla): Dormant since Jan 2023, crates.io version doesn't compile with Rust 1.93+, private API fields
- Custom parsers per language: Rejected per Constitution Principle III (tree-sitter only)

---

## R2: Cognitive Complexity Algorithm Reference

**Decision**: Implement cognitive complexity following the SonarSource specification by G. Ann Campbell (2017).

**Rationale**: This is the de facto standard for cognitive complexity. The paper is publicly available and provides clear, testable rules. Constitution Principle II mandates fidelity to this reference.

**Key algorithm rules**:
1. **Increment (+1)** for each: `if`, `else if`, `else`, `for`, `while`, `do-while`, `match`/`switch`, `catch`/`except`, `goto`, ternary operator
2. **Nesting penalty**: Nested control flow adds `nesting_level` instead of just +1. Each nesting level increments by 1.
3. **Boolean operators**: A sequence of the same operator (`a && b && c`) counts as +1 total. Switching operators (`a && b || c`) adds +1 per switch.
4. **else-if is flat**: Does not increment nesting level (treated as continuation, not nesting).
5. **Lambdas/closures**: Increment nesting level for their body.
6. **Recursion**: +1 for direct recursive calls.

**Alternatives considered**:
- Custom complexity metric: Rejected — no industry validation, harder to compare across tools
- Halstead metrics: Deferred to v0.2.0 per roadmap

---

## R3: Cyclomatic Complexity Implementation

**Decision**: Standard McCabe cyclomatic complexity with +1 base.

**Rationale**: Well-established metric (1976). Every function starts at 1, each decision point adds +1.

**Decision points counted**: `if`, `else if`, `for`, `while`, `do-while`, `match`/`switch` arm (each arm), `catch`/`except`, `&&`, `||`, ternary operator.

**Alternatives considered**:
- Modified cyclomatic (exclude boolean operators): Less useful for test coverage estimation
- Essential complexity: Too specialized, better as future addition

---

## R4: SLOC Counting Strategy

**Decision**: Count physical source lines of code excluding blank lines and comment-only lines.

**Rationale**: Simple, well-understood metric that complements complexity. Uses line-by-line counting after parsing to identify comment nodes via tree-sitter.

**What counts as SLOC**:
- Lines containing at least one non-whitespace, non-comment token
- Lines with both code and inline comments count as 1 SLOC
- Lines that are purely comments or purely whitespace are excluded

**Alternatives considered**:
- Logical LOC (statements): Too language-specific, harder to implement consistently across 10 languages
- Count all lines: Less useful, inflated by formatting

---

## R5: Error Handling Strategy

**Decision**: Define an `ArboristError` enum with distinct variants for each error class specified in FR-013.

**Rationale**: Idiomatic Rust error handling. Distinct variants enable consumers to pattern-match on error types and handle them differently (e.g., skip unsupported files vs. abort on file-not-found).

**Error variants**:
- `FileNotFound { path }` — file doesn't exist at the given path
- `UnsupportedLanguage { language }` — language identifier not recognized
- `UnrecognizedExtension { extension }` — file extension doesn't map to any language
- `LanguageNotEnabled { language }` — language recognized but feature flag not compiled in
- `ParseError { details }` — tree-sitter parse failure (should be rare given error tolerance)
- `IoError(std::io::Error)` — underlying I/O error (permissions, encoding, etc.)

**Alternatives considered**:
- Single error type with message string: Not idiomatic, loses type safety
- `anyhow`/`eyre`: Too opaque for a library — consumers need to match on variants

---

## R6: Feature Flag Design

**Decision**: One feature flag per language, a `default` feature for the 6 most popular, and an `all` feature for all 10 Tier 1 languages.

**Rationale**: Minimizes compilation time and binary size. Consumers include only what they need. Constitution Principle V mandates this approach.

**Feature configuration**:
```toml
[features]
default = ["rust", "python", "javascript", "typescript", "java", "go"]
all = ["default", "csharp", "cpp", "c", "php"]
rust = ["dep:tree-sitter-rust"]
python = ["dep:tree-sitter-python"]
javascript = ["dep:tree-sitter-javascript"]
typescript = ["dep:tree-sitter-typescript"]
java = ["dep:tree-sitter-java"]
csharp = ["dep:tree-sitter-c-sharp"]
cpp = ["dep:tree-sitter-cpp"]
c = ["dep:tree-sitter-c"]
go = ["dep:tree-sitter-go"]
php = ["dep:tree-sitter-php"]
```

**Alternatives considered**:
- All languages always compiled: Rejected — 10 grammars significantly increase compile time
- Language groups (e.g., "web", "systems"): Over-engineered, individual flags are simpler

---

## R7: LanguageProfile Trait Design

**Decision**: A trait with declarative methods that return slices of AST node type names, plus a method to get the tree-sitter language and extract function names.

**Rationale**: Keeps language profiles simple and declarative. The walker and metric calculators are generic and operate on the trait's output. Constitution Principle IV mandates this — adding a language must not modify core logic.

**Trait methods**:
- `function_nodes() -> &[&str]`: Node types that define function/method boundaries
- `control_flow_nodes() -> &[&str]`: Nodes that increment complexity (+1)
- `nesting_nodes() -> &[&str]`: Nodes that increment nesting level
- `boolean_operators() -> &[&str]`: Boolean operator node types
- `else_if_nodes() -> &[&str]`: Nodes treated as flat (no nesting increment)
- `lambda_nodes() -> &[&str]`: Closure/lambda nodes (increment nesting)
- `comment_nodes() -> &[&str]`: Comment node types (for SLOC exclusion)
- `extract_function_name(node, source) -> Option<String>`: Extract function name from AST node
- `language() -> tree_sitter::Language`: Get the tree-sitter language for parsing
- `extensions() -> &[&str]`: File extensions associated with this language

**Alternatives considered**:
- Configuration file per language (TOML/JSON): Less type-safe, harder to validate at compile time
- Procedural macros: Over-engineered for static data

---

## R8: Serialization Strategy

**Decision**: Use `serde` with `Serialize` and `Deserialize` derives on all public types. Serde is a non-optional dependency.

**Rationale**: serde is the Rust ecosystem standard for serialization. Making it always available (not feature-gated) simplifies the API and meets FR-010. The cost is minimal (serde is lightweight and ubiquitous in Rust projects).

**Alternatives considered**:
- Feature-gated serde: Adds API complexity for minimal binary savings
- Custom serialization: Not idiomatic, reinventing the wheel
- No serialization (let consumers implement): Violates FR-010

---

## All NEEDS CLARIFICATION: Resolved

No unresolved technical questions remain. All decisions are grounded in the feasibility analysis, constitution principles, and specification clarifications.
