mod common;

use osakit::{AppleEventDescriptor, Language, Script, ScriptStorageType, StorageOptions};

#[test]
fn script_round_trips_across_file_and_data_constructors() {
    let apple_script = common::apple_script_language();
    let instance = apple_script
        .shared_instance()
        .expect("shared instance should exist");
    let source_path = common::write_text_fixture("source-script", "applescript", "return 40 + 2");

    let from_source = Script::from_source_with_options(
        "return 40 + 2",
        Some(source_path.as_path()),
        Some(&instance),
        StorageOptions::DONT_SET_SCRIPT_LOCATION,
    )
    .expect("script from source should be created");
    from_source.compile().expect("source script should compile");
    assert_eq!(
        from_source.url().expect("url lookup should succeed"),
        Some(source_path.clone())
    );
    assert_eq!(
        from_source
            .execute()
            .expect("script should execute")
            .int32_value(),
        Some(42)
    );

    let from_file =
        Script::from_file_with_options(&source_path, Some(&instance), StorageOptions::NONE)
            .expect("script from file should load");
    from_file.compile().expect("file script should compile");
    assert_eq!(
        from_file
            .execute()
            .expect("file script should execute")
            .int32_value(),
        Some(42)
    );

    let display = from_source
        .execute_and_return_display_value()
        .expect("display-value execution should succeed");
    assert_eq!(display.display_value.as_deref(), Some("42"));

    let compiled_path = common::artifact_path("compiled-script", "scpt");
    from_source
        .write_to_file(
            &compiled_path,
            ScriptStorageType::Script,
            StorageOptions::NONE,
        )
        .expect("compiled script should write to disk");
    let compiled_data = from_source
        .compiled_data(ScriptStorageType::Script, StorageOptions::NONE)
        .expect("compiled data should be produced");
    assert!(!compiled_data.is_empty());

    let from_compiled = Script::from_compiled_data(&compiled_data, None, StorageOptions::NONE)
        .expect("compiled-data constructor should work");
    assert_eq!(
        from_compiled
            .source()
            .expect("compiled script source should decode"),
        "return 40 + 2"
    );

    let descriptor = Script::script_data_descriptor_from_file(&compiled_path)
        .expect("script data descriptor should load");
    let detected_language = Language::for_script_data_descriptor(&descriptor)
        .expect("language lookup by descriptor should succeed")
        .expect("language should be detected");
    assert_eq!(
        detected_language
            .name()
            .expect("name lookup should succeed")
            .as_deref(),
        Some("AppleScript")
    );

    let from_descriptor = Script::from_script_data_descriptor(
        &descriptor,
        Some(compiled_path.as_path()),
        Some(&instance),
        StorageOptions::NONE,
    )
    .expect("descriptor constructor should work");
    assert_eq!(
        from_descriptor
            .execute()
            .expect("descriptor script should run")
            .int32_value(),
        Some(42)
    );
}

#[test]
fn script_handler_rich_text_and_apple_event_paths_work() {
    let apple_script = common::apple_script_language();
    let script = Script::new(
        "on greet(name)\nreturn \"hello \" & name\nend greet",
        Some(&apple_script),
    )
    .expect("handler script should be created");
    script.compile().expect("handler script should compile");

    let handler_result = script
        .execute_handler(
            "greet",
            &[AppleEventDescriptor::string("world").expect("string descriptor")],
        )
        .expect("handler should execute");
    assert_eq!(
        handler_result.string_value().as_deref(),
        Some("hello world")
    );

    assert!(script
        .rich_text_source()
        .expect("rich text source should decode")
        .is_some());
    assert!(script
        .rich_text_from_descriptor(
            &AppleEventDescriptor::string("return \"hello world\"")
                .expect("string descriptor")
        )
        .expect("rich text conversion should succeed")
        .is_some());

    let broken_event = script.execute_apple_event(&AppleEventDescriptor::null());
    assert!(
        broken_event.is_err(),
        "non-AppleEvent descriptors should surface an error"
    );
}
