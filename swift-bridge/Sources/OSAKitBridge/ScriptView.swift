import Cocoa
import OSAKit

@_cdecl("osa_script_view_new")
public func osa_script_view_new(
    _ outView: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outView.pointee = nil
    let view = osaOnMain {
        OSAScriptView(frame: .zero)
    }
    outView.pointee = osaRetain(view)
    _ = errorOut
    return OSA_OK
}

@_cdecl("osa_script_view_source")
public func osa_script_view_source(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let viewPtr else { return nil }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain {
        view.source.map(osaCString) ?? nil
    }
}

@_cdecl("osa_script_view_set_source")
public func osa_script_view_set_source(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ source: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain {
        view.source = source.map { String(cString: $0) }
    }
    return OSA_OK
}

@_cdecl("osa_script_view_uses_script_assistant")
public func osa_script_view_uses_script_assistant(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let viewPtr else { return false }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { view.usesScriptAssistant }
}

@_cdecl("osa_script_view_set_uses_script_assistant")
public func osa_script_view_set_uses_script_assistant(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.usesScriptAssistant = value }
    return OSA_OK
}

@_cdecl("osa_script_view_uses_tabs")
public func osa_script_view_uses_tabs(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let viewPtr else { return false }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { view.usesTabs }
}

@_cdecl("osa_script_view_set_uses_tabs")
public func osa_script_view_set_uses_tabs(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.usesTabs = value }
    return OSA_OK
}

@_cdecl("osa_script_view_tab_width")
public func osa_script_view_tab_width(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let viewPtr else { return 0 }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { UInt64(view.tabWidth) }
}

@_cdecl("osa_script_view_set_tab_width")
public func osa_script_view_set_tab_width(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ width: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.tabWidth = Int(width) }
    return OSA_OK
}

@_cdecl("osa_script_view_wraps_lines")
public func osa_script_view_wraps_lines(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let viewPtr else { return false }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { view.wrapsLines }
}

@_cdecl("osa_script_view_set_wraps_lines")
public func osa_script_view_set_wraps_lines(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.wrapsLines = value }
    return OSA_OK
}

@_cdecl("osa_script_view_indents_wrapped_lines")
public func osa_script_view_indents_wrapped_lines(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    guard let viewPtr else { return false }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { view.indentsWrappedLines }
}

@_cdecl("osa_script_view_set_indents_wrapped_lines")
public func osa_script_view_set_indents_wrapped_lines(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.indentsWrappedLines = value }
    return OSA_OK
}

@_cdecl("osa_script_view_indent_width")
public func osa_script_view_indent_width(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    guard let viewPtr else { return 0 }
    let view: OSAScriptView = osaBorrow(viewPtr)
    return osaOnMain { UInt64(view.indentWidth) }
}

@_cdecl("osa_script_view_set_indent_width")
public func osa_script_view_set_indent_width(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ width: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    let view: OSAScriptView = osaBorrow(viewPtr)
    osaOnMain { view.indentWidth = Int(width) }
    return OSA_OK
}
