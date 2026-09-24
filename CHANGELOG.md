# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-09-24

### Fixed

- `ScriptView` and `ScriptController` no longer hop to the main thread with `DispatchQueue.main.sync`, which deadlocked command-line and async-runtime hosts whose main thread does not service the main queue.
- `ScriptView` and `ScriptController` are released on the main thread.
- `ScriptView` tab and indent widths convert to and from `NSUInteger` exactly instead of trapping for values above `isize::MAX`, and an out-of-range controller state no longer traps.
- NaN or infinite numbers in script-error dictionaries no longer raise an uncatchable Objective-C exception while the bridge encodes JSON.

### Changed

- **Breaking:** off the main thread, `ScriptView::new`, `ScriptController::new`, and the `ScriptView` / `ScriptController` setters and actions return `OsaKitError::MainThreadRequired` instead of blocking on the main queue.
- The README documents the Automation permission (`NSAppleEventsUsageDescription`, the apple-events entitlements), the threading rules, and that `Script::execute*` blocks with no timeout or cancellation.
- `rust-version` is now 1.82.

### Added

- `OsaKitError::MainThreadRequired`.

## [0.2.3] - 2026-06-06

- Corrected the ownership `SAFETY` comment on `script_error_constants`.

## [0.2.2] - 2026-05-18

### Changed

- Added concise rustdoc coverage across the public OSAKit API outside the FFI modules.

## [0.2.1] - 2026-05-17

### Fixed

- Added comprehensive `// SAFETY:` comments to unsafe blocks for memory-safety documentation and FFI pointer handling.

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
