import Cocoa
import OSAKit

public let OSA_OK: Int32 = 0
public let OSA_INVALID_ARGUMENT: Int32 = -1
public let OSA_SCRIPT_ERROR: Int32 = -2
public let OSA_FRAMEWORK_ERROR: Int32 = -3

@inline(__always)
public func osaRetain(_ object: some AnyObject) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func osaBorrow<T: AnyObject>(_ ptr: UnsafeMutableRawPointer, as _: T.Type = T.self) -> T {
    Unmanaged<T>.fromOpaque(ptr).takeUnretainedValue()
}

@_cdecl("osa_object_release")
public func osa_object_release(_ ptr: UnsafeMutableRawPointer?) {
    guard let ptr else { return }
    Unmanaged<AnyObject>.fromOpaque(ptr).release()
}

@inline(__always)
public func osaCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func osaWriteError(
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ message: String
) {
    errorOut?.pointee = osaCString(message)
}

func osaJSONSafe(_ value: Any) -> Any {
    switch value {
    case let dict as [AnyHashable: Any]:
        var result: [String: Any] = [:]
        for (key, value) in dict {
            result[String(describing: key)] = osaJSONSafe(value)
        }
        return result
    case let array as [Any]:
        return array.map(osaJSONSafe)
    case let number as NSNumber:
        return number
    case let string as String:
        return string
    case let descriptor as NSAppleEventDescriptor:
        return descriptor.stringValue ?? "<descriptor type=\(descriptor.descriptorType)>"
    case let value as NSValue:
        return value.description
    case _ as NSNull:
        return NSNull()
    default:
        return String(describing: value)
    }
}

func osaJSONString(_ value: Any) -> String {
    do {
        let data = try JSONSerialization.data(withJSONObject: osaJSONSafe(value), options: [.sortedKeys])
        return String(data: data, encoding: .utf8) ?? "{}"
    } catch {
        return "{}"
    }
}

func osaErrorInfo(_ errorInfo: NSDictionary?) -> String {
    let raw = (errorInfo as? [AnyHashable: Any]).map(osaJSONSafe) as? [String: Any] ?? [:]
    let normalized: [String: Any] = [
        "message": raw["OSAScriptErrorMessage"] ?? raw["NSLocalizedDescription"] ?? NSNull(),
        "briefMessage": raw["OSAScriptErrorBriefMessage"] ?? NSNull(),
        "number": raw["OSAScriptErrorNumber"] ?? NSNull(),
        "appName": raw["OSAScriptErrorAppName"] ?? NSNull(),
        "range": raw["OSAScriptErrorRange"] ?? NSNull(),
        "raw": raw,
    ]
    return osaJSONString(normalized)
}

func osaNSError(_ error: NSError?) -> String {
    let raw = error.map { error in
        [
            "domain": error.domain,
            "code": error.code,
            "message": error.localizedDescription,
            "userInfo": osaJSONSafe(error.userInfo),
        ] as [String: Any]
    } ?? [:]
    let normalized: [String: Any] = [
        "message": raw["message"] ?? NSNull(),
        "briefMessage": NSNull(),
        "number": raw["code"] ?? NSNull(),
        "appName": NSNull(),
        "range": NSNull(),
        "raw": raw,
    ]
    return osaJSONString(normalized)
}
