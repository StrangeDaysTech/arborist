# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-05-19

### Added

- New **Built with AI · Powered by StrayMark** section in README acknowledging
  the project's AI-assisted development process and its use of
  [StrayMark](https://straymark.dev) — a CLI for cognitive discipline in
  AI-assisted engineering — for decision tracking and governance.
- New **Copyright** section in README identifying Strange Days Tech S.A.S. de
  C.V. ([strangedays.tech](https://strangedays.tech/)) as the original author
  and intellectual-property holder.
- `SPDX-License-Identifier: MIT OR Apache-2.0` header with copyright
  attribution on every Rust source file in the library, in line with Rust
  ecosystem conventions.

## [0.1.2] - 2026-04-02

### Fixed

- Move `exclude` list from `[lib]` to `[package]` in Cargo.toml, reducing published
  crate size from 1.2 MiB (287 files) to 242 KiB (138 files).

## [0.1.1] - 2026-04-02

### Changed

- Restructure Feature Flags section in README: move before Installation, expand with
  per-language table showing feature flag, tier, and tree-sitter crate mapping.
- Add `Default` column to Supported Languages table (Yes / Opt-in).
- Add `[package.metadata.docs.rs]` with `all-features = true` so docs.rs renders
  documentation for all 12 languages.
- Add actionable doc comment on `LanguageNotEnabled` error with Cargo.toml fix examples.
- Add module-level documentation to `src/languages/mod.rs` explaining the feature gate
  system and tier structure.

## [0.1.0] - 2026-03-30

Initial release on [crates.io](https://crates.io/crates/arborist-metrics).

### Added

- Core metrics engine computing **cognitive complexity** (SonarSource specification),
  **cyclomatic complexity** (McCabe), and **SLOC** per function/method.
- 12 language profiles via tree-sitter grammars, each gated behind a Cargo feature flag:
  - **Tier 1 (default):** Rust, Python, JavaScript, TypeScript, Java, Go.
  - **Tier 2 (opt-in):** C#, C++, C, PHP, Kotlin, Swift.
- `analyze_file()` and `analyze_file_with_config()` for file-based analysis.
- `analyze_source()` and `analyze_source_with_config()` for in-memory source analysis.
- `AnalysisConfig` with configurable cognitive complexity threshold and
  `include_methods` toggle.
- Per-arm cyclomatic counting for `match`/`switch` constructs (McCabe/SonarSource spec).
- Recursive call detection contributing +1 to cognitive complexity.
- Boolean operator sequence tracking (increment per operator switch, e.g. `&&` to `||`).
- Serde serialization support for all public types (`FileReport`, `FunctionMetrics`,
  `Language`, `AnalysisConfig`).
- `dump_ast` example for language profile development.
- Composite feature flags: `default` (6 languages) and `all` (12 languages).
- 177 tests covering all languages, metrics, thresholds, serialization, and edge cases.

[unreleased]: https://github.com/StrangeDaysTech/arborist/compare/v0.1.3...HEAD
[0.1.3]: https://github.com/StrangeDaysTech/arborist/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/StrangeDaysTech/arborist/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/StrangeDaysTech/arborist/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/StrangeDaysTech/arborist/releases/tag/v0.1.0
