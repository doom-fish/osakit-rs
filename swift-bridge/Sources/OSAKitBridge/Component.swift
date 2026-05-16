import Cocoa
import OSAKit

private func osaComponentInstancePointer(_ language: OSALanguage) -> UInt64 {
    UInt64(UInt(bitPattern: UnsafeMutableRawPointer(language.componentInstance)))
}

private func osaComponentSummary(_ language: OSALanguage) -> [String: Any] {
    [
        "componentInstancePointer": osaComponentInstancePointer(language),
        "language": osaLanguageInfo(language),
    ]
}

private func osaComponentInstanceSummary(_ handle: OSAComponentInstanceHandle) -> [String: Any] {
    [
        "componentInstancePointer": handle.componentInstancePointer,
        "language": osaLanguageInfo(handle.language),
    ]
}

@_cdecl("osa_component_from_language")
public func osa_component_from_language(_ languagePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let languagePtr else { return nil }
    let language: OSALanguage = osaBorrow(languagePtr)
    return osaRetain(OSAComponentHandle(language))
}

@_cdecl("osa_component_from_language_instance")
public func osa_component_from_language_instance(_ instancePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let instancePtr else { return nil }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    return osaRetain(OSAComponentHandle(instance.language))
}

@_cdecl("osa_component_language")
public func osa_component_language(_ componentPtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let componentPtr else { return nil }
    let handle: OSAComponentHandle = osaBorrow(componentPtr)
    return osaRetain(handle.language)
}

@_cdecl("osa_component_summary_json")
public func osa_component_summary_json(
    _ componentPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let componentPtr else {
        osaWriteError(errorOut, "missing OSA component handle")
        return nil
    }
    let handle: OSAComponentHandle = osaBorrow(componentPtr)
    return osaCString(osaJSONString(osaComponentSummary(handle.language)))
}

@_cdecl("osa_component_instance_from_language")
public func osa_component_instance_from_language(_ languagePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let languagePtr else { return nil }
    let language: OSALanguage = osaBorrow(languagePtr)
    return osaRetain(
        OSAComponentInstanceHandle(
            componentInstancePointer: osaComponentInstancePointer(language),
            language: language
        )
    )
}

@_cdecl("osa_component_instance_from_language_instance")
public func osa_component_instance_from_language_instance(_ instancePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let instancePtr else { return nil }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    return osaRetain(
        OSAComponentInstanceHandle(
            componentInstancePointer: UInt64(UInt(bitPattern: UnsafeMutableRawPointer(instance.componentInstance))),
            language: instance.language
        )
    )
}

@_cdecl("osa_component_instance_component")
public func osa_component_instance_component(_ instancePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let instancePtr else { return nil }
    let handle: OSAComponentInstanceHandle = osaBorrow(instancePtr)
    return osaRetain(OSAComponentHandle(handle.language))
}

@_cdecl("osa_component_instance_summary_json")
public func osa_component_instance_summary_json(
    _ instancePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let instancePtr else {
        osaWriteError(errorOut, "missing OSA component instance handle")
        return nil
    }
    let handle: OSAComponentInstanceHandle = osaBorrow(instancePtr)
    return osaCString(osaJSONString(osaComponentInstanceSummary(handle)))
}
