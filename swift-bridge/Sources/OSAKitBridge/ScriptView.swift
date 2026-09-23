import Cocoa
import OSAKit

@_cdecl("osa_script_view_new")
public func osa_script_view_new(
    _ outView: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    outView.pointee = nil
    guard osaRequireMainThread(errorOut) else {
        return OSA_MAIN_THREAD_REQUIRED
    }
    outView.pointee = osaRetain(OSAScriptView(frame: .zero))
    return OSA_OK
}

private func osaWithScriptView(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ body: (OSAScriptView) -> Void
) -> Int32 {
    guard let viewPtr else {
        osaWriteError(errorOut, "missing OSA script view handle")
        return OSA_INVALID_ARGUMENT
    }
    guard osaRequireMainThread(errorOut) else {
        return OSA_MAIN_THREAD_REQUIRED
    }
    body(osaBorrow(viewPtr))
    return OSA_OK
}

private func osaReadScriptView<T>(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ fallback: T,
    _ body: (OSAScriptView) -> T
) -> T {
    guard let viewPtr, Thread.isMainThread else { return fallback }
    return body(osaBorrow(viewPtr))
}

@_cdecl("osa_script_view_source")
public func osa_script_view_source(_ viewPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    osaReadScriptView(viewPtr, nil) { view in
        view.source.map(osaCString) ?? nil
    }
}

@_cdecl("osa_script_view_set_source")
public func osa_script_view_set_source(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ source: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { view in
        view.source = source.map { String(cString: $0) }
    }
}

@_cdecl("osa_script_view_uses_script_assistant")
public func osa_script_view_uses_script_assistant(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    osaReadScriptView(viewPtr, false) { $0.usesScriptAssistant }
}

@_cdecl("osa_script_view_set_uses_script_assistant")
public func osa_script_view_set_uses_script_assistant(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.usesScriptAssistant = value }
}

@_cdecl("osa_script_view_uses_tabs")
public func osa_script_view_uses_tabs(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    osaReadScriptView(viewPtr, false) { $0.usesTabs }
}

@_cdecl("osa_script_view_set_uses_tabs")
public func osa_script_view_set_uses_tabs(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.usesTabs = value }
}

@_cdecl("osa_script_view_tab_width")
public func osa_script_view_tab_width(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    osaReadScriptView(viewPtr, 0) { UInt64(UInt(bitPattern: $0.tabWidth)) }
}

@_cdecl("osa_script_view_set_tab_width")
public func osa_script_view_set_tab_width(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ width: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.tabWidth = Int(bitPattern: UInt(width)) }
}

@_cdecl("osa_script_view_wraps_lines")
public func osa_script_view_wraps_lines(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    osaReadScriptView(viewPtr, false) { $0.wrapsLines }
}

@_cdecl("osa_script_view_set_wraps_lines")
public func osa_script_view_set_wraps_lines(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.wrapsLines = value }
}

@_cdecl("osa_script_view_indents_wrapped_lines")
public func osa_script_view_indents_wrapped_lines(_ viewPtr: UnsafeMutableRawPointer?) -> Bool {
    osaReadScriptView(viewPtr, false) { $0.indentsWrappedLines }
}

@_cdecl("osa_script_view_set_indents_wrapped_lines")
public func osa_script_view_set_indents_wrapped_lines(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ value: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.indentsWrappedLines = value }
}

@_cdecl("osa_script_view_indent_width")
public func osa_script_view_indent_width(_ viewPtr: UnsafeMutableRawPointer?) -> UInt64 {
    osaReadScriptView(viewPtr, 0) { UInt64(UInt(bitPattern: $0.indentWidth)) }
}

@_cdecl("osa_script_view_set_indent_width")
public func osa_script_view_set_indent_width(
    _ viewPtr: UnsafeMutableRawPointer?,
    _ width: UInt64,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    osaWithScriptView(viewPtr, errorOut) { $0.indentWidth = Int(bitPattern: UInt(width)) }
}
