# Arborist

Multi-language code complexity metrics powered by [tree-sitter](https://tree-sitter.github.io/).

Arborist computes **cognitive complexity** (SonarSource), **cyclomatic complexity**
(McCabe), and **source lines of code** (SLOC) for functions and methods across
12 programming languages -- all from a single, embeddable Rust library.

## Supported Languages

| Language | Feature flag | Extensions |
|----------|-------------|------------|
| Rust | `rust` | `.rs` |
| Python | `python` | `.py`, `.pyi` |
| JavaScript | `javascript` | `.js`, `.jsx`, `.mjs`, `.cjs` |
| TypeScript | `typescript` | `.ts`, `.tsx`, `.mts`, `.cts` |
| Java | `java` | `.java` |
| Go | `go` | `.go` |
| C# | `csharp` | `.cs` |
| C++ | `cpp` | `.cpp`, `.cc`, `.cxx`, `.hpp`, `.hxx`, `.hh` |
| C | `c` | `.c`, `.h` |
| PHP | `php` | `.php` |
| Kotlin | `kotlin` | `.kt`, `.kts` |
| Swift | `swift` | `.swift` |

## Installation

Add to your `Cargo.toml`:

```toml
# Default features: Rust, Python, JavaScript, TypeScript, Java, Go
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

## Feature Flags

| Flag | Includes |
|------|----------|
| `default` | `rust`, `python`, `javascript`, `typescript`, `java`, `go` |
| `all` | All 12 languages (Tier 1 + Tier 2) |
| Individual | One language each (e.g., `rust`, `python`, `csharp`) |

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
