mod common;

use osakit::{AppleEventDescriptor, LanguageInstance};

#[test]
fn language_instance_default_target_and_rich_text_round_trip() {
    let apple_script = common::apple_script_language();
    let instance =
        LanguageInstance::new(&apple_script).expect("language instance should be created");
    let shared = apple_script
        .shared_instance()
        .expect("shared instance should be available");

    instance
        .set_default_target(Some(&AppleEventDescriptor::null()))
        .expect("default target should be settable");
    let default_target = instance
        .default_target()
        .expect("default target lookup should succeed");
    assert!(default_target.is_some());

    instance
        .set_default_target(None)
        .expect("default target should be clearable");
    assert!(instance
        .default_target()
        .expect("default target lookup should succeed")
        .is_none());

    let rich_text = shared
        .rich_text_from_descriptor(
            &AppleEventDescriptor::string("return 1 + 2")
                .expect("string descriptor should be created"),
        )
        .expect("rich text should decode")
        .expect("rich text should exist");
    assert!(rich_text.contains("return 1 + 2"));
    assert_eq!(
        shared
            .language()
            .expect("shared instance should expose its language")
            .name()
            .expect("language name should load")
            .as_deref(),
        Some("AppleScript")
    );
}
