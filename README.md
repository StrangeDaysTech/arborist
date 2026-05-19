# Arborist

Multi-language code complexity metrics powered by [tree-sitter](https://tree-sitter.github.io/).

Arborist computes **cognitive complexity** (SonarSource), **cyclomatic complexity**
(McCabe), and **source lines of code** (SLOC) for functions and methods across
12 programming languages -- all from a single, embeddable Rust library.

## Supported Languages

| Language | Feature flag | Extensions | Default |
|----------|-------------|------------|---------|
| Rust | `rust` | `.rs` | Yes |
| Python | `python` | `.py`, `.pyi` | Yes |
| JavaScript | `javascript` | `.js`, `.jsx`, `.mjs`, `.cjs` | Yes |
| TypeScript | `typescript` | `.ts`, `.tsx`, `.mts`, `.cts` | Yes |
| Java | `java` | `.java` | Yes |
| Go | `go` | `.go` | Yes |
| C# | `csharp` | `.cs` | Opt-in |
| C++ | `cpp` | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`, `.hh` | Opt-in |
| C | `c` | `.c`, `.h` | Opt-in |
| PHP | `php` | `.php` | Opt-in |
| Kotlin | `kotlin` | `.kt`, `.kts` | Opt-in |
| Swift | `swift` | `.swift` | Opt-in |

## Feature Flags

Arborist uses **Cargo feature flags** to control which tree-sitter grammars
are compiled. Each language is an independent, optional feature that pulls in
its corresponding tree-sitter grammar crate.

**To enable all 12 languages, use `features = ["all"]`.**

### Tiers

- **Tier 1 (default):** Rust, Python, JavaScript, TypeScript, Java, Go --
  the most mature tree-sitter grammars. Enabled automatically unless you
  set `default-features = false`.
- **Tier 2 (opt-in):** C#, C++, C, PHP, Kotlin, Swift -- require enabling
  their feature flag explicitly or using the `all` composite feature.

### Individual features

| Feature flag | Language | Tier | tree-sitter crate |
|-------------|----------|------|-------------------|
| `rust` | Rust | 1 | `tree-sitter-rust` |
| `python` | Python | 1 | `tree-sitter-python` |
| `javascript` | JavaScript | 1 | `tree-sitter-javascript` |
| `typescript` | TypeScript | 1 | `tree-sitter-typescript` |
| `java` | Java | 1 | `tree-sitter-java` |
| `go` | Go | 1 | `tree-sitter-go` |
| `csharp` | C# | 2 | `tree-sitter-c-sharp` |
| `cpp` | C++ | 2 | `tree-sitter-cpp` |
| `c` | C | 2 | `tree-sitter-c` |
| `php` | PHP | 2 | `tree-sitter-php` |
| `kotlin` | Kotlin | 2 | `tree-sitter-kotlin-ng` |
| `swift` | Swift | 2 | `tree-sitter-swift` |

### Composite features

| Feature | Expands to |
|---------|-----------|
| `default` | `rust`, `python`, `javascript`, `typescript`, `java`, `go` |
| `all` | All 12 languages (`default` + `csharp`, `cpp`, `c`, `php`, `kotlin`, `swift`) |

> **Compile-time note:** Each grammar adds compile time and binary size.
> Use `default-features = false` with only the languages you need for
> minimal builds, or `all` when you need broad language coverage.

## Installation

Add to your `Cargo.toml`:

```toml
# Default features (Tier 1): Rust, Python, JavaScript, TypeScript, Java, Go
[dependencies]
arborist-metrics = "0.1"
```

Select specific languages to reduce compile time:

```toml
[dependencies]
arborist-metrics = { version = "0.1", default-features = false, features = ["rust", "python"] }
```

Enable all 12 languages:

```toml
[dependencies]
arborist-metrics = { version = "0.1", features = ["all"] }
```

## Quick Start

### Analyze a file

```rust,no_run
use arborist::{analyze_file, FileReport};

fn main() -> Result<(), arborist::ArboristError> {
    let report: FileReport = analyze_file("src/main.rs")?;

    println!("File: {} ({:?})", report.path, report.language);
    println!("Total cognitive: {}, SLOC: {}", report.file_cognitive, report.file_sloc);

    for func in &report.functions {
        println!("  {} (lines {}-{}): cognitive={}, cyclomatic={}, sloc={}",
            func.name, func.start_line, func.end_line,
            func.cognitive, func.cyclomatic, func.sloc);
    }

    Ok(())
}
```

### Analyze source code from memory

```rust
use arborist::{analyze_source, Language};

let source = r#"
def hello(name):
    if name:
        print(f"Hello, {name}!")
    else:
        print("Hello, world!")
"#;

let report = analyze_source(source, Language::Python)?;
// report.functions[0].cognitive == 2 (if + else)
# Ok::<(), arborist::ArboristError>(())
```

### Configure thresholds

```rust,no_run
use arborist::{analyze_file_with_config, AnalysisConfig};

let config = AnalysisConfig {
    cognitive_threshold: Some(8),
    ..Default::default()
};

let report = analyze_file_with_config("src/complex.rs", &config)?;

for func in &report.functions {
    if func.exceeds_threshold == Some(true) {
        eprintln!("WARNING: {} has cognitive complexity {} (threshold: 8)",
            func.name, func.cognitive);
    }
}
# Ok::<(), arborist::ArboristError>(())
```

### Serialize to JSON

```rust,no_run
let report = arborist::analyze_file("src/main.rs")?;
let json = serde_json::to_string_pretty(&report)?;
println!("{}", json);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Metrics

### Cognitive Complexity

Follows the [SonarSource specification](https://www.sonarsource.com/docs/CognitiveComplexity.pdf)
by G. Ann Campbell. Measures how difficult code is to *understand*:

- +1 for each control flow break (`if`, `for`, `while`, `match`, `catch`, etc.)
- Nesting penalty: nested control flow adds the current nesting depth
- Boolean operator sequences: one increment per operator *switch* (`&&` to `||`)
- Flat `else if`: does not increase nesting
- +1 for recursive calls and lambda/closure nesting

### Cyclomatic Complexity

Standard McCabe cyclomatic complexity (base 1 + decision points). Measures the
number of linearly independent paths through a function.

### SLOC

Physical source lines of code, excluding blank lines and comment-only lines.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Follow TDD: write fixtures and failing tests *before* implementation
4. Run `cargo clippy -- -D warnings` and `cargo test --all-features`
5. Submit a pull request

### Adding a new language

1. Create `src/languages/<lang>.rs` implementing the `LanguageProfile` trait
2. Add the grammar crate as an optional dependency in `Cargo.toml`
3. Add a feature flag and wire up detection in `src/languages/mod.rs`
4. Create 6 test fixtures in `tests/fixtures/<lang>/`
5. Write integration tests

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

---

## Copyright

**Arborist** is © 2026 **[Strange Days Tech S.A.S. de C.V.](https://strangedays.tech/)** — original author and intellectual-property holder of the source code.

The library is released under the dual MIT / Apache-2.0 license above; this notice records authorship and does **not** modify those license terms. Each source file carries an SPDX header reflecting the same.

> Built by **[Strange Days Tech](https://strangedays.tech/)** — México.
