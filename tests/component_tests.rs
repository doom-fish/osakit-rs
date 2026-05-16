mod common;

use osakit::{OsaComponent, OsaComponentInstance};

#[test]
fn component_summaries_round_trip_to_languages() {
    let apple_script = common::apple_script_language();
    let component =
        OsaComponent::from_language(&apple_script).expect("component should resolve from language");
    let component_summary = component.summary().expect("component summary should load");
    assert_eq!(
        component_summary.language.name.as_deref(),
        Some("AppleScript")
    );
    assert!(component_summary.component_instance_pointer > 0);

    let component_instance = OsaComponentInstance::from_language(&apple_script)
        .expect("component instance should resolve from language");
    let instance_summary = component_instance
        .summary()
        .expect("component instance summary should load");
    assert_eq!(
        instance_summary.language.name.as_deref(),
        Some("AppleScript")
    );
    assert!(instance_summary.component_instance_pointer > 0);

    let round_trip_language = component
        .language()
        .expect("component should produce a language");
    assert_eq!(
        round_trip_language
            .name()
            .expect("round-trip language should have a name")
            .as_deref(),
        Some("AppleScript")
    );
    let round_trip_component = component_instance
        .component()
        .expect("component instance should resolve back to a component");
    assert_eq!(
        round_trip_component
            .summary()
            .expect("round-trip component summary should load")
            .language
            .name
            .as_deref(),
        Some("AppleScript")
    );
}
