import Cocoa
import OSAKit

@_cdecl("osa_descriptor_int32")
public func osa_descriptor_int32(_ value: Int32) -> UnsafeMutableRawPointer? {
    osaRetain(NSAppleEventDescriptor(int32: value))
}

@_cdecl("osa_descriptor_string")
public func osa_descriptor_string(_ value: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let value else { return nil }
    return osaRetain(NSAppleEventDescriptor(string: String(cString: value)))
}

@_cdecl("osa_descriptor_null")
public func osa_descriptor_null() -> UnsafeMutableRawPointer? {
    osaRetain(NSAppleEventDescriptor.null())
}

@_cdecl("osa_descriptor_descriptor_type")
public func osa_descriptor_descriptor_type(_ descriptorPtr: UnsafeMutableRawPointer?) -> OSType {
    guard let descriptorPtr else { return 0 }
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    return descriptor.descriptorType
}

@_cdecl("osa_descriptor_int32_value")
public func osa_descriptor_int32_value(_ descriptorPtr: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptorPtr else { return 0 }
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    return descriptor.int32Value
}

@_cdecl("osa_descriptor_boolean_value")
public func osa_descriptor_boolean_value(_ descriptorPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let descriptorPtr else { return false }
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    return descriptor.booleanValue
}

@_cdecl("osa_descriptor_string_value")
public func osa_descriptor_string_value(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptorPtr else { return nil }
    let descriptor: NSAppleEventDescriptor = osaBorrow(descriptorPtr)
    guard let stringValue = descriptor.stringValue else {
        return nil
    }
    return osaCString(stringValue)
}
