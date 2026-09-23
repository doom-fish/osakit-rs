mod common;

#[test]
fn script_controller_runs_editor_actions_headlessly() {
    common::run_example("05_script_controller");
}

#[test]
fn script_controller_off_the_main_thread_fails_instead_of_blocking() {
    let error = std::thread::spawn(|| osakit::ScriptController::new().map(drop))
        .join()
        .expect("worker thread should not panic")
        .expect_err("ScriptController::new must fail off the main thread");
    assert!(matches!(error, osakit::OsaKitError::MainThreadRequired(_)));
    assert_eq!(error.code(), -4);
}
