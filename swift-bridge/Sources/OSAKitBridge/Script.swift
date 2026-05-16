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
        let resolved: OSALanguage = osaBorrow(ptr)
        return resolved
    }
    outScript.pointee = osaRetain(OSAScript(source: String(cString: source), language: language))
    return OSA_OK
}

@_cdecl("osa_script_new_with_options")
public func osa_script_new_with_options(
    _ source: UnsafePointer<CChar>?,
    _ urlPath: UnsafePointer<CChar>?,
    _ languageInstancePtr: UnsafeMutableRawPointer?,
    _ storageOptions: UInt64,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let source else {
        osaWriteError(errorOut, "missing script source")
        return OSA_INVALID_ARGUMENT
    }

    let instance = languageInstancePtr.map { ptr in
        let resolved: OSALanguageInstance = osaBorrow(ptr)
        return resolved
    }
    let script = OSAScript(
        source: String(cString: source),
        from: osaPath(urlPath),
        languageInstance: instance,
        using: osaStorageOptions(storageOptions)
    )
    outScript.pointee = osaRetain(script)
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
    guard let url = osaPath(path) else {
        osaWriteError(errorOut, "missing script path")
        return OSA_INVALID_ARGUMENT
    }

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

    guard let script else {
        osaWriteError(errorOut, "OSAKit returned a null script")
        return OSA_FRAMEWORK_ERROR
    }
    outScript.pointee = osaRetain(script)
    return OSA_OK
}

@_cdecl("osa_script_from_file_with_options")
public func osa_script_from_file_with_options(
    _ path: UnsafePointer<CChar>?,
    _ languageInstancePtr: UnsafeMutableRawPointer?,
    _ storageOptions: UInt64,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let url = osaPath(path) else {
        osaWriteError(errorOut, "missing script path")
        return OSA_INVALID_ARGUMENT
    }

    let instance = languageInstancePtr.map { ptr in
        let resolved: OSALanguageInstance = osaBorrow(ptr)
        return resolved
    }
    do {
        let script = try OSAScript(contentsOf: url, languageInstance: instance, using: osaStorageOptions(storageOptions))
        outScript.pointee = osaRetain(script)
        return OSA_OK
    } catch {
        osaWriteError(errorOut, osaNSError(error as NSError))
        return OSA_SCRIPT_ERROR
    }
}

@_cdecl("osa_script_from_compiled_data")
public func osa_script_from_compiled_data(
    _ bytes: UnsafeRawPointer?,
    _ length: Int,
    _ urlPath: UnsafePointer<CChar>?,
    _ storageOptions: UInt64,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let bytes, length > 0 else {
        osaWriteError(errorOut, "missing compiled script bytes")
        return OSA_INVALID_ARGUMENT
    }

    let data = Data(bytes: bytes, count: length)
    do {
        let script = try OSAScript(compiledData: data, from: osaPath(urlPath), using: osaStorageOptions(storageOptions))
        outScript.pointee = osaRetain(script)
        return OSA_OK
    } catch {
        osaWriteError(errorOut, osaNSError(error as NSError))
        return OSA_SCRIPT_ERROR
    }
}

@_cdecl("osa_script_from_script_data_descriptor")
public func osa_script_from_script_data_descriptor(
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ urlPath: UnsafePointer<CChar>?,
    _ languageInstancePtr: UnsafeMutableRawPointer?,
    _ storageOptions: UInt64,
    _ outScript: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outScript.pointee = nil
    guard let descriptorPtr else {
        osaWriteError(errorOut, "missing script data descriptor")
        return OSA_INVALID_ARGUMENT
    }

    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    let instance = languageInstancePtr.map { ptr in
        let resolved: OSALanguageInstance = osaBorrow(ptr)
        return resolved
    }
    do {
        let script = try OSAScript(
            scriptDataDescriptor: descriptor,
            from: osaPath(urlPath),
            languageInstance: instance,
            using: osaStorageOptions(storageOptions)
        )
        outScript.pointee = osaRetain(script)
        return OSA_OK
    } catch {
        osaWriteError(errorOut, osaNSError(error as NSError))
        return OSA_SCRIPT_ERROR
    }
}

@_cdecl("osa_script_data_descriptor_from_file")
public func osa_script_data_descriptor_from_file(
    _ path: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    guard let url = osaPath(path) else {
        osaWriteError(errorOut, "missing script path")
        return nil
    }
    guard let descriptor = OSAScript.scriptDataDescriptor(withContentsOf: url) else {
        osaWriteError(errorOut, "OSAKit failed to produce a script data descriptor")
        return nil
    }
    return osaRetain(descriptor)
}

@_cdecl("osa_script_source")
public func osa_script_source(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return osaCString(script.source)
}

@_cdecl("osa_script_rich_text_source")
public func osa_script_rich_text_source(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return script.richTextSource.map { osaCString($0.string) } ?? nil
}

@_cdecl("osa_script_url")
public func osa_script_url(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return script.url.map { osaCString($0.path) } ?? nil
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

@_cdecl("osa_script_set_language")
public func osa_script_set_language(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ languagePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    guard let languagePtr else {
        osaWriteError(errorOut, "missing OSA language handle")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    let language: OSALanguage = osaBorrow(languagePtr)
    script.language = language
    return OSA_OK
}

@_cdecl("osa_script_language_instance")
public func osa_script_language_instance(_ scriptPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let scriptPtr else { return nil }
    let script: OSAScript = osaBorrow(scriptPtr)
    return osaRetain(script.languageInstance)
}

@_cdecl("osa_script_set_language_instance")
public func osa_script_set_language_instance(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ instancePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    guard let instancePtr else {
        osaWriteError(errorOut, "missing OSA language instance handle")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    script.languageInstance = instance
    return OSA_OK
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

@_cdecl("osa_script_execute_and_return_display_value")
public func osa_script_execute_and_return_display_value(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outDisplayValue: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outDescriptor.pointee = nil
    outDisplayValue?.pointee = nil
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    var displayValue: NSAttributedString?
    var errorInfo: NSDictionary?
    guard let descriptor = script.executeAndReturnDisplayValue(&displayValue, error: &errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    outDescriptor.pointee = osaRetain(descriptor)
    outDisplayValue?.pointee = displayValue.flatMap { osaCString($0.string) }
    return OSA_OK
}

@_cdecl("osa_script_execute_handler")
public func osa_script_execute_handler(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ arguments: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ argumentCount: Int,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outDescriptor.pointee = nil
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    guard let name else {
        osaWriteError(errorOut, "missing handler name")
        return OSA_INVALID_ARGUMENT
    }

    let script: OSAScript = osaBorrow(scriptPtr)
    let resolvedArguments: [Any] = if let arguments, argumentCount > 0 {
        UnsafeBufferPointer(start: arguments, count: argumentCount).compactMap { raw in
            raw.map { ptr in
                let descriptor: NSAppleEventDescriptor = osaBorrow(ptr)
                return descriptor as Any
            }
        }
    } else {
        []
    }

    var errorInfo: NSDictionary?
    guard let descriptor = script.executeHandler(withName: String(cString: name), arguments: resolvedArguments, error: &errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    outDescriptor.pointee = osaRetain(descriptor)
    return OSA_OK
}

@_cdecl("osa_script_rich_text_from_descriptor")
public func osa_script_rich_text_from_descriptor(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return nil
    }
    guard let descriptorPtr else {
        osaWriteError(errorOut, "missing Apple event descriptor")
        return nil
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    return script.richText(from: descriptor).map { osaCString($0.string) } ?? nil
}

@_cdecl("osa_script_write_to_url")
public func osa_script_write_to_url(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ type: UnsafePointer<CChar>?,
    _ storageOptions: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return OSA_INVALID_ARGUMENT
    }
    guard let url = osaPath(path) else {
        osaWriteError(errorOut, "missing script path")
        return OSA_INVALID_ARGUMENT
    }
    guard let type else {
        osaWriteError(errorOut, "missing storage type")
        return OSA_INVALID_ARGUMENT
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    var errorInfo: NSDictionary?
    guard script.write(to: url, ofType: String(cString: type), using: osaStorageOptions(storageOptions), error: &errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return OSA_SCRIPT_ERROR
    }
    return OSA_OK
}

@_cdecl("osa_script_compiled_data")
public func osa_script_compiled_data(
    _ scriptPtr: UnsafeMutableRawPointer?,
    _ type: UnsafePointer<CChar>?,
    _ storageOptions: UInt64,
    _ outLength: UnsafeMutablePointer<Int>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
    outLength?.pointee = 0
    guard let scriptPtr else {
        osaWriteError(errorOut, "missing OSA script handle")
        return nil
    }
    guard let type else {
        osaWriteError(errorOut, "missing storage type")
        return nil
    }
    let script: OSAScript = osaBorrow(scriptPtr)
    var errorInfo: NSDictionary?
    guard let data = script.compiledData(forType: String(cString: type), using: osaStorageOptions(storageOptions), error: &errorInfo) else {
        osaWriteError(errorOut, osaErrorInfo(errorInfo))
        return nil
    }
    outLength?.pointee = data.count
    guard data.count > 0 else {
        return nil
    }
    guard let raw = osaCopyData(data) else {
        osaWriteError(errorOut, "failed to allocate compiled data buffer")
        return nil
    }
    return raw
}
