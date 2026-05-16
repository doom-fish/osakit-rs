use osakit::{Language, LanguageFeatures};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let languages = Language::available_languages()?;
    assert!(!languages.is_empty());

    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let default_language =
        Language::default_language().ok_or_else(|| "default language missing".to_string())?;
    apple_script.set_as_default()?;

    let summary = apple_script.summary()?;
    let flags = summary.feature_flags();
    assert!(flags.contains(LanguageFeatures::SUPPORTS_COMPILING));
    assert_eq!(summary.name.as_deref(), Some("AppleScript"));
    assert!(default_language.name()?.is_some());

    println!(
        "languages = {} / AppleScript features = 0x{:x}",
        languages.len(),
        summary.features
    );
    Ok(())
}
