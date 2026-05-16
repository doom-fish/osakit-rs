# osakit-rs

Safe Rust bindings for Apple's [OSAKit](https://developer.apple.com/documentation/osakit) framework on macOS.

> **Status:** v0.2.0 covers `OSALanguage`, `OSALanguageInstance`, `OSAScript`, `OSAScriptError`, `OSAScriptController`, `OSAScriptView`, and OSA component metadata round-trips, including script storage, compiled-data loading, controller actions, and AppKit editor configuration.

## Quick start

```rust,no_run
use osakit::{Language, Script};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let script = Script::new("return 1 + 2", Some(&apple_script))?;
    script.compile()?;
    let result = script.execute()?;
    println!("result = {}", result.int32_value().unwrap_or_default());
    Ok(())
}
```

## Highlights

- `Language::available_languages`, `for_name`, `for_script_data_descriptor`, `default_language`, and default-language selection
- `LanguageInstance::new`, `shared_instance`, `default_target`, and rich-text source rendering
- `Script::from_source_with_options`, `from_file_with_options`, `from_compiled_data`, `from_script_data_descriptor`, `compiled_data`, `write_to_file`, `execute_handler`, and display-value execution
- Structured `ScriptErrorDetails`, normalized error-key constants, and decoded descriptor fields for `OSAKit` compile / runtime failures
- `ScriptController` actions (`compile`, `record`, `run`, `stop`) and `ScriptView` editor-property round-trips without opening a window
- `OsaComponent` and `OsaComponentInstance` summaries that expose underlying OSA component-instance pointers alongside language metadata

## Examples

Run every example with:

```bash
for ex in examples/*.rs; do
  cargo run --example "$(basename "$ex" .rs)"
done
```

The examples cover script execution, language discovery, language-instance configuration, script-error decoding, controller actions, script-view configuration, and OSA component summaries.

## Coverage audit

See [COVERAGE.md](COVERAGE.md) for the header-by-header API audit against `OSAKit.framework`.

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
