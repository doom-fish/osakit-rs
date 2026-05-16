import Cocoa
import OSAKit

@_cdecl("osa_script_controller_new")
public func osa_script_controller_new(
    _ outController: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outController.pointee = nil
    let controller = osaOnMain {
        let created = OSAScriptController()
        created.scriptView = OSAScriptView(frame: .zero)
        created.resultView = NSTextView(frame: .zero)
        return created
    }
    outController.pointee = osaRetain(controller)
    _ = errorOut
    return OSA_OK
}

@_cdecl("osa_script_controller_script_view")
public func osa_script_controller_script_view(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else { return nil }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        controller.scriptView.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_script_view")
public func osa_script_controller_set_script_view(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ scriptViewPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let controllerPtr else {
        osaWriteError(errorOut, "missing OSA script controller handle")
        return OSA_INVALID_ARGUMENT
    }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    let view = scriptViewPtr.map { ptr in
        let resolved: OSAScriptView = osaBorrow(ptr)
        return resolved
    }
    osaOnMain {
        controller.scriptView = view
    }
    return OSA_OK
}

@_cdecl("osa_script_controller_script")
public func osa_script_controller_script(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else { return nil }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        controller.script.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_script")
public func osa_script_controller_set_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let controllerPtr else {
        osaWriteError(errorOut, "missing OSA script controller handle")
        return OSA_INVALID_ARGUMENT
    }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    let script = scriptPtr.map { ptr in
        let resolved: OSAScript = osaBorrow(ptr)
        return resolved
    }
    osaOnMain {
        controller.script = script
    }
    return OSA_OK
}

@_cdecl("osa_script_controller_language")
public func osa_script_controller_language(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let controllerPtr else { return nil }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        controller.language.map(osaRetain)
    }
}

@_cdecl("osa_script_controller_set_language")
public func osa_script_controller_set_language(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ languagePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let controllerPtr else {
        osaWriteError(errorOut, "missing OSA script controller handle")
        return OSA_INVALID_ARGUMENT
    }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    let language = languagePtr.map { ptr in
        let resolved: OSALanguage = osaBorrow(ptr)
        return resolved
    }
    osaOnMain {
        controller.language = language
    }
    return OSA_OK
}

@_cdecl("osa_script_controller_result_text")
public func osa_script_controller_result_text(_ controllerPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let controllerPtr else { return nil }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        controller.resultView.map { osaCString($0.string) } ?? nil
    }
}

@_cdecl("osa_script_controller_script_state")
public func osa_script_controller_script_state(_ controllerPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let controllerPtr else { return Int32(OSAScriptState.stopped.rawValue) }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        Int32(controller.scriptState.rawValue)
    }
}

@_cdecl("osa_script_controller_is_compiling")
public func osa_script_controller_is_compiling(_ controllerPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let controllerPtr else { return false }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    return osaOnMain {
        controller.isCompiling
    }
}

private func osaRunControllerAction(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    action: @escaping (OSAScriptController) -> Void
) -> Int32 {
    guard let controllerPtr else {
        osaWriteError(errorOut, "missing OSA script controller handle")
        return OSA_INVALID_ARGUMENT
    }
    let controller: OSAScriptController = osaBorrow(controllerPtr)
    osaOnMain {
        action(controller)
    }
    return OSA_OK
}

@_cdecl("osa_script_controller_compile_script")
public func osa_script_controller_compile_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaRunControllerAction(controllerPtr, errorOut) { controller in
        controller.compileScript(nil)
    }
}

@_cdecl("osa_script_controller_record_script")
public func osa_script_controller_record_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaRunControllerAction(controllerPtr, errorOut) { controller in
        controller.recordScript(nil)
    }
}

@_cdecl("osa_script_controller_run_script")
public func osa_script_controller_run_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaRunControllerAction(controllerPtr, errorOut) { controller in
        controller.runScript(nil)
    }
}

@_cdecl("osa_script_controller_stop_script")
public func osa_script_controller_stop_script(
    _ controllerPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaRunControllerAction(controllerPtr, errorOut) { controller in
        controller.stopScript(nil)
    }
}
