import Cocoa
import OSAKit

private func osaRangeInfo(_ rawValue: Any?) -> Any {
    guard let value = rawValue as? NSValue else {
        return NSNull()
    }
    let range = value.rangeValue
    return [
        "location": range.location,
        "length": range.length,
    ]
}

func osaErrorInfo(_ errorInfo: NSDictionary?) -> String {
    let foundationRaw = errorInfo as? [AnyHashable: Any] ?? [:]
    let raw = foundationRaw.reduce(into: [String: Any]()) { partialResult, entry in
        partialResult[String(describing: entry.key)] = osaJSONSafe(entry.value)
    }
    let normalized: [String: Any] = [
        "message": foundationRaw[OSAScriptErrorMessageKey] ?? foundationRaw[NSLocalizedDescriptionKey] ?? NSNull(),
        "briefMessage": foundationRaw[OSAScriptErrorBriefMessageKey] ?? foundationRaw[NSLocalizedFailureReasonErrorKey] ?? NSNull(),
        "number": foundationRaw[OSAScriptErrorNumberKey] ?? NSNull(),
        "partialResult": foundationRaw[OSAScriptErrorPartialResultKey].map(osaJSONSafe) ?? NSNull(),
        "offendingObject": foundationRaw[OSAScriptErrorOffendingObjectKey].map(osaJSONSafe) ?? NSNull(),
        "expectedType": foundationRaw[OSAScriptErrorExpectedTypeKey].map(osaJSONSafe) ?? NSNull(),
        "appAddress": foundationRaw[OSAScriptErrorAppAddressKey].map(osaJSONSafe) ?? NSNull(),
        "appName": foundationRaw[OSAScriptErrorAppNameKey] ?? foundationRaw[OSAScriptErrorAppName] ?? NSNull(),
        "range": osaRangeInfo(foundationRaw[OSAScriptErrorRangeKey] ?? foundationRaw[OSAScriptErrorRange]),
        "raw": raw,
    ]
    return osaJSONString(normalized)
}

func osaNSError(_ error: NSError?) -> String {
    guard let error else {
        return osaErrorInfo(nil)
    }

    let foundationRaw = error.userInfo as [AnyHashable: Any]
    let rawUserInfo = foundationRaw.reduce(into: [String: Any]()) { partialResult, entry in
        partialResult[String(describing: entry.key)] = osaJSONSafe(entry.value)
    }
    let normalized: [String: Any] = [
        "message": foundationRaw[OSAScriptErrorMessageKey] ?? foundationRaw[NSLocalizedDescriptionKey] ?? error.localizedDescription,
        "briefMessage": foundationRaw[OSAScriptErrorBriefMessageKey] ?? foundationRaw[NSLocalizedFailureReasonErrorKey] ?? NSNull(),
        "number": foundationRaw[OSAScriptErrorNumberKey] ?? error.code,
        "partialResult": foundationRaw[OSAScriptErrorPartialResultKey].map(osaJSONSafe) ?? NSNull(),
        "offendingObject": foundationRaw[OSAScriptErrorOffendingObjectKey].map(osaJSONSafe) ?? NSNull(),
        "expectedType": foundationRaw[OSAScriptErrorExpectedTypeKey].map(osaJSONSafe) ?? NSNull(),
        "appAddress": foundationRaw[OSAScriptErrorAppAddressKey].map(osaJSONSafe) ?? NSNull(),
        "appName": foundationRaw[OSAScriptErrorAppNameKey] ?? foundationRaw[OSAScriptErrorAppName] ?? NSNull(),
        "range": osaRangeInfo(foundationRaw[OSAScriptErrorRangeKey] ?? foundationRaw[OSAScriptErrorRange]),
        "raw": [
            "domain": error.domain,
            "code": error.code,
            "message": error.localizedDescription,
            "userInfo": rawUserInfo,
        ],
    ]
    return osaJSONString(normalized)
}

@_cdecl("osa_script_error_constants_json")
public func osa_script_error_constants_json() -> UnsafeMutablePointer<CChar>? {
    osaCString(osaJSONString([
        "messageKey": OSAScriptErrorMessageKey,
        "briefMessageKey": OSAScriptErrorBriefMessageKey,
        "numberKey": OSAScriptErrorNumberKey,
        "partialResultKey": OSAScriptErrorPartialResultKey,
        "offendingObjectKey": OSAScriptErrorOffendingObjectKey,
        "expectedTypeKey": OSAScriptErrorExpectedTypeKey,
        "appAddressKey": OSAScriptErrorAppAddressKey,
        "appNameKey": OSAScriptErrorAppNameKey,
        "rangeKey": OSAScriptErrorRangeKey,
        "message": OSAScriptErrorMessage,
        "number": OSAScriptErrorNumber,
        "appName": OSAScriptErrorAppName,
        "briefMessage": OSAScriptErrorBriefMessage,
        "range": OSAScriptErrorRange,
    ]))
}
