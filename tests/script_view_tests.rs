mod common;

#[test]
fn script_view_properties_round_trip() {
    common::run_example("06_script_view");
}

#[test]
fn script_view_off_the_main_thread_fails_instead_of_blocking() {
    let error = std::thread::spawn(|| osakit::ScriptView::new().map(drop))
        .join()
        .expect("worker thread should not panic")
        .expect_err("ScriptView::new must fail off the main thread");
    assert!(matches!(error, osakit::OsaKitError::MainThreadRequired(_)));
    assert_eq!(error.code(), -4);
    assert!(error.message().contains("main thread"));
}
