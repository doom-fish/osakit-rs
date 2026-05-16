use osakit::ScriptView;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let view = ScriptView::new()?;
    view.set_source(Some("return 1 + 2"))?;
    view.set_uses_script_assistant(true)?;
    view.set_uses_tabs(true)?;
    view.set_tab_width(4)?;
    view.set_wraps_lines(true)?;
    view.set_indents_wrapped_lines(true)?;
    view.set_indent_width(2)?;

    assert_eq!(view.source()?.as_deref(), Some("return 1 + 2"));
    println!(
        "script view configured: tabs={} indent={}",
        view.uses_tabs(),
        view.indent_width()
    );
    Ok(())
}
