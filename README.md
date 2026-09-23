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

## Requirements

- macOS 10.15 or later (the Swift bridge's deployment target).
- Xcode or the Command Line Tools, so `build.rs` can run `swift build`.

## Highlights

- `Language::available_languages`, `for_name`, `for_script_data_descriptor`, `default_language`, and default-language selection
- `LanguageInstance::new`, `shared_instance`, `default_target`, and rich-text source rendering
- `Script::from_source_with_options`, `from_file_with_options`, `from_compiled_data`, `from_script_data_descriptor`, `compiled_data`, `write_to_file`, `execute_handler`, and display-value execution
- Structured `ScriptErrorDetails`, normalized error-key constants, and decoded descriptor fields for `OSAKit` compile / runtime failures
- `ScriptController` actions (`compile`, `record`, `run`, `stop`) and `ScriptView` editor-property round-trips without opening a window
- `OsaComponent` and `OsaComponentInstance` summaries that expose underlying OSA component-instance pointers alongside language metadata

## Automation permission

Scripts that only compute values need no permission. Scripts that send Apple events to other applications, such as `tell application "Finder"`, are subject to the Automation privacy controls:

- The first Apple event from a host app to a target app asks the user for consent. The answer is stored per host and target pair under System Settings > Privacy & Security > Automation, and this crate never answers the prompt. A denied event fails with `errAEEventNotPermitted` (-1743), reported as `OsaKitError::ScriptError` with `number == Some(-1743)`.
- An app must declare `NSAppleEventsUsageDescription` in its `Info.plist`. Without it, macOS denies the event without asking.
- An app built with the hardened runtime needs the `com.apple.security.automation.apple-events` entitlement. A sandboxed app also needs `com.apple.security.scripting-targets` or `com.apple.security.temporary-exception.apple-events` for the apps it controls.
- A command-line tool is covered by the permission of the app that launched it, typically the terminal.

## Threading and blocking

- `ScriptView` and `ScriptController` wrap `AppKit` objects and must be used on the main thread. Off the main thread `ScriptView::new` and `ScriptController::new` return `OsaKitError::MainThreadRequired` instead of blocking on the main queue, which would deadlock command-line and async-runtime hosts. The handles are `!Send`, so they are also released on the main thread.
- The other wrappers can be used on any thread, but they are `!Send` too, so each value stays on the thread that created it.
- `Script::execute`, `execute_apple_event`, `execute_and_return_display_value`, and `execute_handler` run synchronously on the calling thread, with no timeout and no way to cancel. Each Apple event a script sends is bounded by `AppleScript`'s event timeout (two minutes unless the script uses `with timeout`), but the script as a whole is not. Run scripts on a dedicated thread if the caller must stay responsive.

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
