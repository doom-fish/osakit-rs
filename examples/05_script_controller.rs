use osakit::{Language, ScriptController, ScriptState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let apple_script = Language::for_name("AppleScript")?
        .ok_or_else(|| "AppleScript language missing".to_string())?;
    let controller = ScriptController::new()?;
    let script_view = controller
        .script_view()?
        .ok_or_else(|| "script view missing".to_string())?;
    script_view.set_source(Some("return 1 + 2"))?;
    controller.set_language(Some(&apple_script))?;
    controller.compile_script()?;
    controller.run_script()?;
    assert_eq!(controller.result_text()?.as_deref(), Some("3"));
    controller.record_script()?;
    assert_eq!(controller.script_state()?, ScriptState::Recording);
    controller.stop_script()?;
    assert_eq!(controller.script_state()?, ScriptState::Stopped);

    println!(
        "script controller result = {}",
        controller.result_text()?.unwrap_or_default()
    );
    Ok(())
}
