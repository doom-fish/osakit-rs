import Cocoa
import OSAKit

@_cdecl("osa_language_instance_new")
public func osa_language_instance_new(
    _ languagePtr: UnsafeMutableRawPointer?,
    _ outInstance: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outInstance.pointee = nil
    guard let languagePtr else {
        osaWriteError(errorOut, "missing OSA language handle")
        return OSA_INVALID_ARGUMENT
    }
    let language: OSALanguage = osaBorrow(languagePtr)
    outInstance.pointee = osaRetain(OSALanguageInstance(language: language))
    return OSA_OK
}

@_cdecl("osa_language_instance_language")
public func osa_language_instance_language(_ instancePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let instancePtr else { return nil }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    return osaRetain(instance.language)
}

@_cdecl("osa_language_instance_info_json")
public func osa_language_instance_info_json(
    _ instancePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let instancePtr else {
        osaWriteError(errorOut, "missing OSA language instance handle")
        return nil
    }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    return osaCString(osaJSONString(osaLanguageInfo(instance.language)))
}

@_cdecl("osa_language_instance_default_target")
public func osa_language_instance_default_target(_ instancePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let instancePtr else { return nil }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    guard let descriptor = instance.defaultTarget else {
        return nil
    }
    return osaRetain(descriptor)
}

@_cdecl("osa_language_instance_set_default_target")
public func osa_language_instance_set_default_target(
    _ instancePtr: UnsafeMutableRawPointer?,
    _ targetPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let instancePtr else {
        osaWriteError(errorOut, "missing OSA language instance handle")
        return OSA_INVALID_ARGUMENT
    }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    let target = targetPtr.map { ptr in
        let descriptor: NSAppleEventDescriptor = osaBorrow(ptr)
        return descriptor
    }
    instance.defaultTarget = target
    return OSA_OK
}

@_cdecl("osa_language_instance_rich_text_from_descriptor")
public func osa_language_instance_rich_text_from_descriptor(
    _ instancePtr: UnsafeMutableRawPointer?,
    _ descriptorPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let instancePtr else {
        osaWriteError(errorOut, "missing OSA language instance handle")
        return nil
    }
    guard let descriptorPtr else {
        osaWriteError(errorOut, "missing Apple event descriptor")
        return nil
    }
    let instance: OSALanguageInstance = osaBorrow(instancePtr)
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    return instance.richText(from: descriptor).map { osaCString($0.string) } ?? nil
}
