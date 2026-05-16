import Cocoa
import OSAKit

@_cdecl("osa_script_new")
public func osa_script_new(
    _ source: UnsafePointer<CChar>?,
    _ languagePtr: UnsafeMutableRawPointer?,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let source else {
        osaWriteError(errorOut, "missing script source")
        return OSA_INVALID_ARGUMENT
    }

    let language = languagePtr.map { ptr in
        let language: OSALanguage = osaBorrow(ptr)
        return language
    }
    let script = OSAScript(source: String(cString: source), language: language)
    outScript.pointee = osaRetain(script)
    _ = errorOut
    return OSA_OK
}

@_cdecl("osa_script_from_file")
public func osa_script_from_file(
    _ path: UnsafePointer<CChar>?,
    _ languagePtr: UnsafeMutableRawPointer?,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let path else {
        osaWriteError(errorOut, "missing script path")
        return OSA_INVALID_ARGUMENT
    }

    let url = URL(fileURLWithPath: String(cString: path))
    let script: OSAScript?
    if let languagePtr {
        let language: OSALanguage = osaBorrow(languagePtr)
        let instance = language.sharedLanguageInstance()
        do {
            script = try OSAScript(contentsOf: url, languageInstance: instance, using: [])
        } catch {
            osaWriteError(errorOut, osaNSError(error as NSError))
            return OSA_SCRIPT_ERROR
        }
    } else {
        var errorInfo: NSDictionary?
        script = OSAScript(contentsOf: url, error: &errorInfo)
        if script == nil {
            osaWriteError(errorOut, osaErrorInfo(errorInfo))
            return OSA_SCRIPT_ERROR
        }
    }

    if let script {
        outScript.pointee = osaRetain(script)
        return OSA_OK
    }

    osaWriteError(errorOut, "OSAKit returned a null script")
    return OSA_FRAMEWORK_ERROR
}

@_cdecl("osa_script_source")
public func osa_script_source(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return osaCString(script.source)
}

@_cdecl("osa_script_is_compiled")
public func osa_script_is_compiled(_ scriptPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let scriptPtr else { return false }
    let script: OSAScript = osaBorrow(scriptPtr)
    return script.isCompiled
}

@_cdecl("osa_script_language")
public func osa_script_language(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return osaRetain(script.language)
}

@_cdecl("osa_script_language_instance")
public func osa_script_language_instance(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return osaRetain(script.languageInstance)
}

@_cdecl("osa_script_compile")
public func osa_script_compile(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    var errorInfo: NSDictionary?
    guard script.compileAndReturnError(&errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    return OSA_OK
}

@_cdecl("osa_script_execute")
public func osa_script_execute(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outDescriptor.pointee = nil
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    var errorInfo: NSDictionary?
    guard let descriptor = script.executeAndReturnError(&errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    outDescriptor.pointee = osaRetain(descriptor)
    return OSA_OK
}

@_cdecl("osa_script_execute_apple_event")
public func osa_script_execute_apple_event(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ eventPtr: UnsafeMutableRawPointer?,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outDescriptor.pointee = nil
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    guard let eventPtr else {
        osaWriteError(errorOut, "missing Apple event descriptor")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    let event: NSAppleEventDescriptor = osaBorrow(eventPtr)
    var errorInfo: NSDictionary?
    guard let descriptor = script.executeAppleEvent(event, error: &errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    outDescriptor.pointee = osaRetain(descriptor)
    return OSA_OK
}
