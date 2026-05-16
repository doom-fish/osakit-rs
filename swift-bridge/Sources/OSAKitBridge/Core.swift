import Cocoa
import Dispatch
import OSAKit

public let OSA_OK: Int32 = 0
public let OSA_INVALID_ARGUMENT: Int32 = -1
public let OSA_SCRIPT_ERROR: Int32 = -2
public let OSA_FRAMEWORK_ERROR: Int32 = -3

final class OSAComponentHandle {
    let language: OSALanguage

    init(_ language: OSALanguage) {
        self.language = language
    }
}

final class OSAComponentInstanceHandle {
    let componentInstancePointer: UInt64
    let language: OSALanguage

    init(componentInstancePointer: UInt64, language: OSALanguage) {
        self.componentInstancePointer = componentInstancePointer
        self.language = language
    }
}

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

@inline(__always)
func osaOnMain<T>(_ body: () -> T) -> T {
    if Thread.isMainThread {
        return body()
    }
    return DispatchQueue.main.sync(execute: body)
}

func osaDescriptorInfo(_ descriptor: NSAppleEventDescriptor) -> [String: Any] {
    [
        "descriptorType": descriptor.descriptorType,
        "int32Value": descriptor.int32Value,
        "booleanValue": descriptor.booleanValue,
        "stringValue": descriptor.stringValue ?? NSNull(),
    ]
}

func osaJSONSafe(_ value: Any) -> Any {
    switch value {
    case let dict as [AnyHashable: Any]:
        var result: [String: Any] = [:]
        for (key, nestedValue) in dict {
            result[String(describing: key)] = osaJSONSafe(nestedValue)
        }
        return result
    case let array as [Any]:
        return array.map(osaJSONSafe)
    case let number as NSNumber:
        return number
    case let string as String:
        return string
    case let descriptor as NSAppleEventDescriptor:
        return osaDescriptorInfo(descriptor)
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

func osaStorageOptions(_ rawValue: UInt64) -> OSAStorageOptions {
    OSAStorageOptions(rawValue: UInt(rawValue))
}

func osaPath(_ path: UnsafePointer<CChar>?) -> URL? {
    guard let path else { return nil }
    return URL(fileURLWithPath: String(cString: path))
}

func osaCopyData(_ data: Data) -> UnsafeMutableRawPointer? {
    let count = data.count
    guard count > 0 else {
        return nil
    }
    guard let raw = malloc(count) else {
        return nil
    }
    data.withUnsafeBytes { buffer in
        memcpy(raw, buffer.baseAddress, count)
    }
    return raw
}
