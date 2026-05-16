import Cocoa
import OSAKit

func osaLanguageInfo(_ language: OSALanguage) -> [String: Any] {
    [
        "name": language.name ?? NSNull(),
        "info": language.info ?? NSNull(),
        "version": language.version ?? NSNull(),
        "typeCode": language.type,
        "subType": language.subType,
        "manufacturer": language.manufacturer,
        "features": language.features.rawValue,
        "threadSafe": language.isThreadSafe,
    ]
}

@_cdecl("osa_language_available_languages_json")
public func osa_language_available_languages_json(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    _ = errorOut
    return osaCString(osaJSONString(OSALanguage.availableLanguages().map(osaLanguageInfo)))
}

@_cdecl("osa_language_for_name")
public func osa_language_for_name(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    guard let language = OSALanguage(forName: String(cString: name)) else {
        return nil
    }
    return osaRetain(language)
}

@_cdecl("osa_language_default")
public func osa_language_default() -> UnsafeMutableRawPointer? {
    guard let language = OSALanguage.default() else {
        return nil
    }
    return osaRetain(language)
}

@_cdecl("osa_language_name")
public func osa_language_name(_ languagePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let languagePtr else { return nil }
    let language: OSALanguage = osaBorrow(languagePtr)
    guard let name = language.name else {
        return nil
    }
    return osaCString(name)
}

@_cdecl("osa_language_info_json")
public func osa_language_info_json(
    _ languagePtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
    guard let languagePtr else {
        osaWriteError(errorOut, "missing OSA language handle")
        return nil
    }
    let language: OSALanguage = osaBorrow(languagePtr)
    return osaCString(osaJSONString(osaLanguageInfo(language)))
}

@_cdecl("osa_language_shared_instance")
public func osa_language_shared_instance(_ languagePtr: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let languagePtr else { return nil }
    let language: OSALanguage = osaBorrow(languagePtr)
    return osaRetain(language.sharedLanguageInstance())
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
