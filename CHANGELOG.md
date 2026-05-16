# Changelog

## [0.1.0] - 2026-05-16

### Added

- `Language`, `LanguageInstance`, and `LanguageSummary` wrappers around `OSALanguage` and `OSALanguageInstance`.
- `Script` wrapper for source-based and file-based `OSAScript` creation, compilation, execution, and Apple-event dispatch.
- `AppleEventDescriptor` helper type with integer, string, and null constructors plus typed result accessors.
- Structured `ScriptErrorDetails` decoding from OSAKit compile / execute error dictionaries.
- Smoke example `examples/01_smoke.rs` that verifies both AppleScript and JavaScript for Automation execution.
