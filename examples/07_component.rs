use osakit::{Language, OsaComponent, OsaComponentInstance};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let component = OsaComponent::from_language(&apple_script)?;
    let component_summary = component.summary()?;
    let component_instance = OsaComponentInstance::from_language(&apple_script)?;
    let instance_summary = component_instance.summary()?;

    assert_eq!(
        component.language()?.name()?.as_deref(),
        Some("AppleScript")
    );
    assert_eq!(
        component_summary.language.name.as_deref(),
        Some("AppleScript")
    );
    assert!(instance_summary.component_instance_pointer > 0);

    println!(
        "component instance pointer = {}",
        instance_summary.component_instance_pointer
    );
    Ok(())
}
