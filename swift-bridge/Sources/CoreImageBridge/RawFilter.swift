import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_raw_filter_supported_camera_models_lines")
public func ci_raw_filter_supported_camera_models_lines() -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 12.0, *) {
        return ci_string(CIRAWFilter.supportedCameraModels.joined(separator: "\n"))
    }
    return ci_string("")
}

@_cdecl("ci_raw_filter_new_from_path")
public func ci_raw_filter_new_from_path(
    _ path: UnsafePointer<CChar>?,
    _ outFilter: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard #available(macOS 12.0, *) else {
            throw CIBridgeError.unsupported("CIRAWFilter requires macOS 12.0")
        }
        guard let path, let outFilter else {
            throw CIBridgeError.invalidArgument("missing RAW image path or output filter pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let filter = CIRAWFilter(imageURL: url) else {
            throw CIBridgeError.nullResult("CIRAWFilter(imageURL:) returned nil for \(url.path)")
        }
        outFilter.pointee = ci_retain(filter)
    }
}

@_cdecl("ci_raw_filter_new_from_data")
public func ci_raw_filter_new_from_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ identifierHint: UnsafePointer<CChar>?,
    _ outFilter: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard #available(macOS 12.0, *) else {
            throw CIBridgeError.unsupported("CIRAWFilter requires macOS 12.0")
        }
        guard let bytes, len > 0, let outFilter else {
            throw CIBridgeError.invalidArgument("missing RAW bytes or output filter pointer")
        }
        let data = Data(bytes: bytes, count: len)
        guard let filter = CIRAWFilter(
            imageData: data,
            identifierHint: identifierHint.map { String(cString: $0) }
        ) else {
            throw CIBridgeError.nullResult("CIRAWFilter(imageData:identifierHint:) returned nil")
        }
        outFilter.pointee = ci_retain(filter)
    }
}

@_cdecl("ci_raw_filter_supported_decoder_versions_lines")
public func ci_raw_filter_supported_decoder_versions_lines(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return ci_string("") }
        return ci_string(filter.supportedDecoderVersions.map(\.rawValue).joined(separator: "\n"))
    }
    return ci_string("")
}

@_cdecl("ci_raw_filter_native_size")
public func ci_raw_filter_native_size(
    _ handle: UnsafeMutableRawPointer?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else {
            outWidth?.pointee = 0
            outHeight?.pointee = 0
            return
        }
        outWidth?.pointee = filter.nativeSize.width
        outHeight?.pointee = filter.nativeSize.height
        return
    }
    outWidth?.pointee = 0
    outHeight?.pointee = 0
}

@_cdecl("ci_raw_filter_properties_json")
public func ci_raw_filter_properties_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return ci_string("{}") }
        return ci_string(ci_json_string(from: filter.properties) ?? "{}")
    }
    return ci_string("{}")
}

@_cdecl("ci_raw_filter_orientation")
public func ci_raw_filter_orientation(_ handle: UnsafeMutableRawPointer?) -> UInt32 {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return 1 }
        return filter.orientation.rawValue
    }
    return 1
}

@_cdecl("ci_raw_filter_set_orientation")
public func ci_raw_filter_set_orientation(_ handle: UnsafeMutableRawPointer?, _ orientation: UInt32) {
    guard #available(macOS 12.0, *) else { return }
    guard let filter: CIRAWFilter = ci_borrow(handle),
          let orientation = CGImagePropertyOrientation(rawValue: orientation)
    else {
        return
    }
    filter.orientation = orientation
}

@_cdecl("ci_raw_filter_is_draft_mode_enabled")
public func ci_raw_filter_is_draft_mode_enabled(_ handle: UnsafeMutableRawPointer?) -> Bool {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return false }
        return filter.isDraftModeEnabled
    }
    return false
}

@_cdecl("ci_raw_filter_set_draft_mode_enabled")
public func ci_raw_filter_set_draft_mode_enabled(_ handle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard #available(macOS 12.0, *) else { return }
    guard let filter: CIRAWFilter = ci_borrow(handle) else { return }
    filter.isDraftModeEnabled = enabled
}

@_cdecl("ci_raw_filter_decoder_version")
public func ci_raw_filter_decoder_version(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return ci_string("") }
        return ci_string(filter.decoderVersion.rawValue)
    }
    return ci_string("")
}

@_cdecl("ci_raw_filter_set_decoder_version")
public func ci_raw_filter_set_decoder_version(_ handle: UnsafeMutableRawPointer?, _ value: UnsafePointer<CChar>?) {
    guard #available(macOS 12.0, *) else { return }
    guard let filter: CIRAWFilter = ci_borrow(handle), let value else { return }
    filter.decoderVersion = CIRAWDecoderVersion(rawValue: String(cString: value))
}

@_cdecl("ci_raw_filter_scale_factor")
public func ci_raw_filter_scale_factor(_ handle: UnsafeMutableRawPointer?) -> Float {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return 0 }
        return filter.scaleFactor
    }
    return 0
}

@_cdecl("ci_raw_filter_set_scale_factor")
public func ci_raw_filter_set_scale_factor(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard #available(macOS 12.0, *) else { return }
    guard let filter: CIRAWFilter = ci_borrow(handle) else { return }
    filter.scaleFactor = value
}

@_cdecl("ci_raw_filter_exposure")
public func ci_raw_filter_exposure(_ handle: UnsafeMutableRawPointer?) -> Float {
    if #available(macOS 12.0, *) {
        guard let filter: CIRAWFilter = ci_borrow(handle) else { return 0 }
        return filter.exposure
    }
    return 0
}

@_cdecl("ci_raw_filter_set_exposure")
public func ci_raw_filter_set_exposure(_ handle: UnsafeMutableRawPointer?, _ value: Float) {
    guard #available(macOS 12.0, *) else { return }
    guard let filter: CIRAWFilter = ci_borrow(handle) else { return }
    filter.exposure = value
}

@_cdecl("ci_raw_filter_preview_image")
public func ci_raw_filter_preview_image(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard #available(macOS 12.0, *) else { return nil }
    guard let filter: CIRAWFilter = ci_borrow(handle), let image = filter.previewImage else { return nil }
    return ci_retain(image)
}
