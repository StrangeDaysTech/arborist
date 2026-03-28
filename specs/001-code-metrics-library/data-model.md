# Data Model: Code Metrics Analysis Library (Arborist)

**Branch**: `001-code-metrics-library` | **Date**: 2026-03-27

## Entities

### FunctionMetrics

Represents the analysis results for a single function or method.

| Field | Type | Description |
|-------|------|-------------|
| `name` | String | Function/method name. Methods include class qualifier (e.g., `MyStruct::method`). |
| `start_line` | usize | 1-based line number where the function starts. |
| `end_line` | usize | 1-based line number where the function ends. |
| `cognitive` | u64 | Cognitive complexity score (SonarSource algorithm). |
| `cyclomatic` | u64 | Cyclomatic complexity score (McCabe). |
| `sloc` | u64 | Source lines of code within the function (excludes blanks and comment-only lines). |
| `exceeds_threshold` | Option\<bool\> | Present and `true` when cognitive complexity exceeds configured threshold. `None` when no threshold is configured. |

**Identity**: Unique within a FileReport by `(name, start_line)`. Two functions can share a name (overloads) but not a start line.

**Scope**: Only named functions and methods produce entries. Closures/lambdas contribute to their containing function's metrics but do not produce independent entries.

---

### FileReport

Represents the analysis results for a complete source file.

| Field | Type | Description |
|-------|------|-------------|
| `path` | String | File path as provided by the caller (or empty for in-memory analysis). |
| `language` | Language | Detected or specified programming language. |
| `functions` | Vec\<FunctionMetrics\> | Ordered list of functions found in the file (by start_line ascending). |
| `file_cognitive` | u64 | Sum of all functions' cognitive complexity values. |
| `file_cyclomatic` | u64 | Sum of all functions' cyclomatic complexity values. |
| `file_sloc` | u64 | Total source lines in the entire file, including top-level code outside functions. |

**Aggregate rules**:
- `file_cognitive` = Σ `functions[i].cognitive`
- `file_cyclomatic` = Σ `functions[i].cyclomatic`
- `file_sloc` = total SLOC of the entire file (not just function bodies)

---

### Language

Enum representing supported programming languages.

| Variant | Extensions | Grammar Crate | Feature Flag |
|---------|------------|---------------|-------------|
| `Rust` | `.rs` | `tree-sitter-rust` | `rust` |
| `Python` | `.py`, `.pyi` | `tree-sitter-python` | `python` |
| `JavaScript` | `.js`, `.jsx`, `.mjs`, `.cjs` | `tree-sitter-javascript` | `javascript` |
| `TypeScript` | `.ts`, `.tsx`, `.mts`, `.cts` | `tree-sitter-typescript` | `typescript` |
| `Java` | `.java` | `tree-sitter-java` | `java` |
| `CSharp` | `.cs` | `tree-sitter-c-sharp` | `csharp` |
| `Cpp` | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`, `.hh` | `tree-sitter-cpp` | `cpp` |
| `C` | `.c`, `.h` | `tree-sitter-c` | `c` |
| `Go` | `.go` | `tree-sitter-go` | `go` |
| `Php` | `.php` | `tree-sitter-php` | `php` |

**Extension conflict**: `.h` defaults to C. Users needing C++ analysis for `.h` files should use `analyze_source()` with explicit `Language::Cpp`.

**Ambiguity resolution**: When both `c` and `cpp` features are enabled, `.h` resolves to C. `.hpp`/`.hxx`/`.hh` always resolve to C++.

---

### AnalysisConfig

Represents user-configurable analysis parameters.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cognitive_threshold` | Option\<u64\> | `None` | When set, `exceeds_threshold` is populated on each FunctionMetrics. |
| `include_methods` | bool | `true` | Whether to include class/struct methods in addition to top-level functions. |

**Defaults**: `AnalysisConfig::default()` = no threshold, methods included.

---

### LanguageProfile (trait)

Extension point for adding new language support. Not a data entity — it's a behavioral contract.

| Method | Returns | Purpose |
|--------|---------|---------|
| `function_nodes()` | `&[&str]` | AST node types that define function/method boundaries |
| `control_flow_nodes()` | `&[&str]` | Nodes that increment complexity (+1) |
| `nesting_nodes()` | `&[&str]` | Nodes that increment nesting level |
| `boolean_operators()` | `&[&str]` | Boolean operator node types |
| `else_if_nodes()` | `&[&str]` | Flat nodes (no nesting increment) |
| `lambda_nodes()` | `&[&str]` | Closure/lambda nodes (increment nesting) |
| `comment_nodes()` | `&[&str]` | Comment nodes (for SLOC exclusion) |
| `extract_function_name(node, source)` | `Option<String>` | Extract function name from AST node |
| `language()` | `tree_sitter::Language` | Get the tree-sitter parser language |
| `extensions()` | `&[&str]` | File extensions for this language |

---

### ArboristError (enum)

| Variant | Fields | Condition |
|---------|--------|-----------|
| `FileNotFound` | `path: String` | File path doesn't exist |
| `UnsupportedLanguage` | `language: String` | Language identifier not recognized |
| `UnrecognizedExtension` | `extension: String` | File extension doesn't map to any known language |
| `LanguageNotEnabled` | `language: String` | Language recognized but its feature flag is not compiled in |
| `ParseError` | `details: String` | tree-sitter parse failure (rare) |
| `Io` | `std::io::Error` | Underlying I/O error |

## Relationships

```text
AnalysisConfig ──configures──> analyze_file() / analyze_source()
                                      │
                                      ▼
                               FileReport
                               ├── language: Language
                               └── functions: Vec<FunctionMetrics>
                                                  │
                                                  └── exceeds_threshold (derived from AnalysisConfig.cognitive_threshold)

LanguageProfile ──implements──> (one per Language variant)
                                      │
                                      └──used by──> walker.rs (generic AST traversal)
```

## Validation Rules

- `FunctionMetrics.start_line` <= `FunctionMetrics.end_line`
- `FunctionMetrics.cognitive` >= 0 (always true for u64)
- `FileReport.functions` is sorted by `start_line` ascending
- `FileReport.file_cognitive` == sum of all `functions[i].cognitive`
- `FileReport.file_cyclomatic` == sum of all `functions[i].cyclomatic`
- `FileReport.file_sloc` >= sum of all `functions[i].sloc` (file SLOC includes top-level code)
- `AnalysisConfig.cognitive_threshold` when `Some(n)`: n > 0
