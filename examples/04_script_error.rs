use osakit::{script_error_constants, Language, OsaKitError, Script};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let constants = script_error_constants()?;
    let script = Script::new("return )", Some(&apple_script))?;

    match script.compile() {
        Err(OsaKitError::ScriptError(details)) => {
            assert_eq!(constants.message_key, constants.message);
            assert!(details
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("Expected"));
            println!(
                "decoded script error: {}",
                details.message.unwrap_or_default()
            );
        }
        other => return Err(format!("expected script error, got {other:?}").into()),
    }

    Ok(())
}
