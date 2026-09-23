import Cocoa
import OSAKit

@_cdecl("osa_script_controller_new")
public func osa_script_controller_new(
    _ outController: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outController.pointee = nil
    guard osaRequireMainThread(errorOut) else {
        return OSA_MAIN_THREAD_REQUIRED
    }
    let controller = OSAScriptController()
    controller.scriptView = OSAScriptView(frame: .zero)
    controller.resultView = NSTextView(frame: .zero)
    outController.pointee = osaRetain(controller)
    return OSA_OK
}

private func osaWithController(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ body: (OSAScriptController) -> Void
) -> Int32 {
    guard let controllerPtr else {
        osaWriteError(errorOut, "missing OSA script controller handle")
        return OSA_INVALID_ARGUMENT
    }
    guard osaRequireMainThread(errorOut) else {
        return OSA_MAIN_THREAD_REQUIRED
    }
    body(osaBorrow(controllerPtr))
    return OSA_OK
}

private func osaReadController<T>(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ fallback: T,
    _ body: (OSAScriptController) -> T
) -> T {
    guard let controllerPtr, Thread.isMainThread else { return fallback }
    return body(osaBorrow(controllerPtr))
}

@_cdecl("osa_script_controller_script_view")
public func osa_script_controller_script_view(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    osaReadController(controllerPtr, nil) { controller in
        controller.scriptView.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_script_view")
public func osa_script_controller_set_script_view(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ scriptViewPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let view = scriptViewPtr.map { ptr -> OSAScriptView in osaBorrow(ptr) }
    return osaWithController(controllerPtr, errorOut) { $0.scriptView = view }
}

@_cdecl("osa_script_controller_script")
public func osa_script_controller_script(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    osaReadController(controllerPtr, nil) { controller in
        controller.script.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_script")
public func osa_script_controller_set_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let script = scriptPtr.map { ptr -> OSAScript in osaBorrow(ptr) }
    return osaWithController(controllerPtr, errorOut) { $0.script = script }
}

@_cdecl("osa_script_controller_language")
public func osa_script_controller_language(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    osaReadController(controllerPtr, nil) { controller in
        controller.language.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_language")
public func osa_script_controller_set_language(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ languagePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let language = languagePtr.map { ptr -> OSALanguage in osaBorrow(ptr) }
    return osaWithController(controllerPtr, errorOut) { $0.language = language }
}

@_cdecl("osa_script_controller_result_text")
public func osa_script_controller_result_text(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    osaReadController(controllerPtr, nil) { controller in
        controller.resultView.map { osaCString($0.string) } ?? nil
    }
}

@_cdecl("osa_script_controller_script_state")
public func osa_script_controller_script_state(_ controllerPtr: UnsafeMutableRawPointer?) -> Int32 {
    osaReadController(controllerPtr, -1) { controller in
        Int32(clamping: controller.scriptState.rawValue)
    }
}

@_cdecl("osa_script_controller_is_compiling")
public func osa_script_controller_is_compiling(_ controllerPtr: UnsafeMutableRawPointer?) -> Bool {
    osaReadController(controllerPtr, false) { $0.isCompiling }
}

@_cdecl("osa_script_controller_compile_script")
public func osa_script_controller_compile_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithController(controllerPtr, errorOut) { $0.compileScript(nil) }
}

@_cdecl("osa_script_controller_record_script")
public func osa_script_controller_record_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithController(controllerPtr, errorOut) { $0.recordScript(nil) }
}

@_cdecl("osa_script_controller_run_script")
public func osa_script_controller_run_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithController(controllerPtr, errorOut) { $0.runScript(nil) }
}

@_cdecl("osa_script_controller_stop_script")
public func osa_script_controller_stop_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithController(controllerPtr, errorOut) { $0.stopScript(nil) }
}
