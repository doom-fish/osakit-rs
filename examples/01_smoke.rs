use osakit::{Language, Script};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script_language = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let java_script_language = Language::for_name("JavaScript")?
        .ok_or_else(|| "JavaScript language missing".to_string())?;

    let apple_script = Script::new("return 1 + 2", Some(&apple_script_language))?;
    apple_script.compile()?;
    let apple_result = apple_script.execute_and_return_display_value()?;
    assert_eq!(apple_result.descriptor.int32_value(), Some(3));
    assert_eq!(apple_result.display_value.as_deref(), Some("3"));

    let java_script = Script::new("1 + 2", Some(&java_script_language))?;
    java_script.compile()?;
    let java_result = java_script.execute()?;
    assert_eq!(java_result.int32_value(), Some(3));

    println!("✅ osakit AppleScript + JavaScript OK");
    Ok(())
}
