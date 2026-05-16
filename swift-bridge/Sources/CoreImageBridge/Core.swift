import CoreGraphics
import CoreImage
import Dispatch
import Foundation
import Metal

public let CIX_OK: Int32 = 0
public let CIX_INVALID_ARGUMENT: Int32 = -1
public let CIX_NULL_RESULT: Int32 = -2
public let CIX_FRAMEWORK: Int32 = -3
public let CIX_IO: Int32 = -4
public let CIX_UNSUPPORTED: Int32 = -5
public let CIX_UNKNOWN: Int32 = -99

@inline(__always)
public func ci_string(_ string: String) -> UnsafeMutablePointer<CChar>? {
    string.withCString { strdup($0) }
}

@inline(__always)
public func ci_retain<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(object).toOpaque()
}

@inline(__always)
public func ci_release(_ handle: UnsafeMutableRawPointer?) {
    guard let handle else { return }
    Unmanaged<AnyObject>.fromOpaque(handle).release()
}

@inline(__always)
public func ci_borrow<T: AnyObject>(_ handle: UnsafeMutableRawPointer?) -> T? {
    guard let handle else { return nil }
    return Unmanaged<T>.fromOpaque(handle).takeUnretainedValue()
}

public enum CIBridgeError: Error, CustomStringConvertible {
    case invalidArgument(String)
    case nullResult(String)
    case unsupported(String)
    case io(String)
    case framework(Error)
    case unknown(String)

    public var description: String {
        switch self {
        case .invalidArgument(let message):
            return message
        case .nullResult(let message):
            return message
        case .unsupported(let message):
            return message
        case .io(let message):
            return message
        case .framework(let error):
            return error.localizedDescription
        case .unknown(let message):
            return message
        }
    }

    public var statusCode: Int32 {
        switch self {
        case .invalidArgument:
            return CIX_INVALID_ARGUMENT
        case .nullResult:
            return CIX_NULL_RESULT
        case .unsupported:
            return CIX_UNSUPPORTED
        case .io:
            return CIX_IO
        case .framework:
            return CIX_FRAMEWORK
        case .unknown:
            return CIX_UNKNOWN
        }
    }
}

@inline(__always)
public func ci_status(from error: Error) -> Int32 {
    if let error = error as? CIBridgeError {
        return error.statusCode
    }
    return CIX_FRAMEWORK
}

@inline(__always)
public func ci_message(from error: Error) -> String {
    if let error = error as? CIBridgeError {
        return error.description
    }
    return (error as NSError).localizedDescription
}

@inline(__always)
public func ci_fail(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ error: Error
) -> Int32 {
    if let outError {
        outError.pointee = ci_string(ci_message(from: error))
    }
    return ci_status(from: error)
}

@inline(__always)
public func ci_run(
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    _ work: () throws -> Void
) -> Int32 {
    do {
        try work()
        if let outError {
            outError.pointee = nil
        }
        return CIX_OK
    } catch {
        return ci_fail(outError, error)
    }
}

public func ci_block_on_async<T>(
    timeoutSeconds: Int = 30,
    work: @escaping () async throws -> T,
    onSuccess: @escaping (T) -> Void
) -> Int32 {
    let semaphore = DispatchSemaphore(value: 0)
    var status = CIX_OK
    Task {
        do {
            let result = try await work()
            onSuccess(result)
        } catch {
            status = ci_status(from: error)
        }
        semaphore.signal()
    }

    let waitResult = semaphore.wait(timeout: .now() + .seconds(timeoutSeconds))
    if waitResult == .timedOut {
        return CIX_UNKNOWN
    }
    return status
}

public func ci_srgb_color_space() -> CGColorSpace {
    CGColorSpace(name: CGColorSpace.sRGB)!
}

public func ci_normalize_json(_ value: Any) -> Any {
    switch value {
    case let value as NSString:
        return String(value)
    case let value as NSNumber:
        return value
    case let value as NSNull:
        return value
    case let value as Data:
        return value.base64EncodedString()
    case let value as URL:
        return value.absoluteString
    case let value as CIColor:
        return [
            "red": value.red,
            "green": value.green,
            "blue": value.blue,
            "alpha": value.alpha,
        ]
    case let value as CIVector:
        return (0..<value.count).map { Double(value.value(at: $0)) }
    case let value as CGRect:
        return [
            "x": value.origin.x,
            "y": value.origin.y,
            "width": value.size.width,
            "height": value.size.height,
        ]
    case let value as CGPoint:
        return [
            "x": value.x,
            "y": value.y,
        ]
    case let value as CGSize:
        return [
            "width": value.width,
            "height": value.height,
        ]
    case let value as CGAffineTransform:
        return [
            "a": value.a,
            "b": value.b,
            "c": value.c,
            "d": value.d,
            "tx": value.tx,
            "ty": value.ty,
        ]
    case let value as [Any]:
        return value.map(ci_normalize_json)
    case let value as [AnyHashable: Any]:
        var normalized: [String: Any] = [:]
        for (key, nestedValue) in value {
            normalized[String(describing: key)] = ci_normalize_json(nestedValue)
        }
        return normalized
    default:
        return String(describing: value)
    }
}

public func ci_json_string(from value: Any) -> String? {
    let normalized = ci_normalize_json(value)
    guard JSONSerialization.isValidJSONObject(normalized) else {
        return nil
    }
    do {
        let data = try JSONSerialization.data(withJSONObject: normalized, options: [.sortedKeys])
        return String(data: data, encoding: .utf8)
    } catch {
        return nil
    }
}

@_cdecl("ci_object_retain")
public func ci_object_retain(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let handle else { return nil }
    let object = Unmanaged<AnyObject>.fromOpaque(handle).takeUnretainedValue()
    return ci_retain(object)
}

@_cdecl("ci_object_release")
public func ci_object_release(_ handle: UnsafeMutableRawPointer?) {
    ci_release(handle)
}
