mod common;

use osakit::{Language, LanguageFeatures, Script, ScriptStorageType, StorageOptions};

#[test]
fn language_catalog_and_defaults_are_available() {
    let languages = Language::available_languages().expect("language list should load");
    assert!(!languages.is_empty());

    let apple_script = common::apple_script_language();
    let summary = apple_script
        .summary()
        .expect("language summary should load");
    assert_eq!(summary.name.as_deref(), Some("AppleScript"));
    assert!(summary
        .feature_flags()
        .contains(LanguageFeatures::SUPPORTS_COMPILING));

    let default_language = Language::default_language().expect("default language should exist");
    Language::set_default(&default_language).expect("setting the current default should succeed");
    apple_script
        .set_as_default()
        .expect("setting AppleScript as default should succeed");

    let script = Script::new("return 7", Some(&apple_script)).expect("script should be created");
    script.compile().expect("script should compile");
    let compiled_path = common::artifact_path("language-descriptor", "scpt");
    script
        .write_to_file(
            &compiled_path,
            ScriptStorageType::Script,
            StorageOptions::NONE,
        )
        .expect("compiled script should write");
    let descriptor = Script::script_data_descriptor_from_file(&compiled_path)
        .expect("descriptor should load from file");
    let detected = Language::for_script_data_descriptor(&descriptor)
        .expect("descriptor lookup should succeed")
        .expect("descriptor should map back to a language");
    assert_eq!(
        detected
            .name()
            .expect("detected language should have a name")
            .as_deref(),
        Some("AppleScript")
    );
}
