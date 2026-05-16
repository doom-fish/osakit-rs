# osakit-rs

Safe Rust bindings for Apple's [OSAKit](https://developer.apple.com/documentation/osakit) framework on macOS.

> **Status:** v0.1.0 covers `OSALanguage`, `OSALanguageInstance`, `OSAScript`, and `NSAppleEventDescriptor` helpers for compiling and executing AppleScript or JavaScript for Automation (`JXA`) scripts. `OSAScriptController` is intentionally skipped for now because it is AppKit editor UI.

## Quick start

```rust,no_run
use osakit::{Language, Script};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?.expect("AppleScript available");
    let script = Script::new("return 1 + 2", Some(&apple_script));
    script.compile()?;
    let result = script.execute()?;
    println!("result = {}", result.int32_value().unwrap_or_default());
    Ok(())
}
```

## Highlights

- `Language::for_name`, `Language::default_language`, `Language::available_languages`, and `LanguageSummary`
- `LanguageInstance` access via `Language::shared_instance` and `Script::language_instance`
- `Script::new`, `Script::from_file`, `compile`, `execute`, `execute_apple_event`, `source`, and `is_compiled`
- `AppleEventDescriptor` constructors for `int32`, `string`, and `null`, plus typed result accessors
- Built-in `AppleScript` and `JavaScript` (`JXA`) smoke coverage

## Smoke example

Run the framework smoke test with:

```bash
cargo run --all-features --example 01_smoke
```

It compiles and executes `return 1 + 2` in `AppleScript` plus `1 + 2` in `JavaScript` for Automation, verifies both results are `3`, and exits with `✅ osakit AppleScript + JavaScript OK`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
