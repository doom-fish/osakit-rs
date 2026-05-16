mod common;

use osakit::{script_error_constants, OsaKitError, Script};

#[test]
fn script_errors_are_normalized_with_constants() {
    let constants = script_error_constants().expect("script error constants should load");
    assert_eq!(constants.message_key, constants.message);
    assert_eq!(constants.brief_message_key, constants.brief_message);

    let apple_script = common::apple_script_language();
    let script = Script::new("return )", Some(&apple_script))
        .expect("broken script should still be created");
    match script.compile() {
        Err(OsaKitError::ScriptError(details)) => {
            assert!(details
                .message
                .as_deref()
                .unwrap_or_default()
                .contains("Expected"));
            assert!(details.range.is_some());
        }
        other => panic!("expected script error, got {other:?}"),
    }
}
