# Changelog

## [0.2.0] - 2026-05-16

### Added

- `ScriptStorageType`, `StorageOptions`, and modern `OSAScript` constructors for source URLs, compiled data, and script-data descriptors.
- `ScriptDisplayValue`, `execute_handler`, `compiled_data`, `write_to_file`, and rich-text helpers for `OSAScript`.
- Dedicated `LanguageInstance`, `ScriptController`, `ScriptView`, `OsaComponent`, and `OsaComponentInstance` wrappers.
- Structured `ScriptErrorConstants`, expanded `ScriptErrorDetails` descriptor fields, and normalized range decoding.
- Seven numbered examples and seven integration-test files covering every logical OSAKit area.
- `COVERAGE.md` with a complete `OSAKit.framework` API audit.

## [0.1.0] - 2026-05-16

### Added

- `Language`, `LanguageInstance`, and `LanguageSummary` wrappers around `OSALanguage` and `OSALanguageInstance`.
- `Script` wrapper for source-based and file-based `OSAScript` creation, compilation, execution, and Apple-event dispatch.
- `AppleEventDescriptor` helper type with integer, string, and null constructors plus typed result accessors.
- Structured `ScriptErrorDetails` decoding from OSAKit compile / execute error dictionaries.
- Smoke example `examples/01_smoke.rs` that verifies both AppleScript and JavaScript for Automation execution.
