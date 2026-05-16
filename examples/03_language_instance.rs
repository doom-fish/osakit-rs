use osakit::{AppleEventDescriptor, Language, LanguageInstance};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let instance = LanguageInstance::new(&apple_script)?;
    let shared = apple_script.shared_instance()?;

    instance.set_default_target(Some(&AppleEventDescriptor::null()))?;
    let default_target = instance
        .default_target()?
        .ok_or_else(|| "default target missing".to_string())?;
    assert_ne!(default_target.descriptor_type(), 0);
    instance.set_default_target(None)?;
    assert!(instance.default_target()?.is_none());

    let rich_text = shared
        .rich_text_from_descriptor(&AppleEventDescriptor::string("return 1 + 2")?)?
        .ok_or_else(|| "rich text missing".to_string())?;
    assert!(rich_text.contains("return 1 + 2"));

    println!(
        "language instance OK for {}",
        shared.language()?.name()?.unwrap_or_default()
    );
    Ok(())
}
