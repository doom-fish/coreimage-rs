import CoreGraphics
import CoreImage
import CoreVideo
import Foundation
import IOSurface
import Metal

@_cdecl("ci_image_from_path")
public func ci_image_from_path(
    _ path: UnsafePointer<CChar>?,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let path, let outImage else {
            throw CIBridgeError.invalidArgument("missing path or output image pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let image = CIImage(contentsOf: url) else {
            throw CIBridgeError.nullResult("CIImage(contentsOf:) returned nil for \(url.path)")
        }
        outImage.pointee = ci_retain(image)
    }
}

@_cdecl("ci_image_from_encoded_data")
public func ci_image_from_encoded_data(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len > 0, let outImage else {
            throw CIBridgeError.invalidArgument("missing encoded bytes or output image pointer")
        }
        let data = Data(bytes: bytes, count: len)
        guard let image = CIImage(data: data) else {
            throw CIBridgeError.nullResult("CIImage(data:) returned nil")
        }
        outImage.pointee = ci_retain(image)
    }
}

@_cdecl("ci_image_from_cg_image")
public func ci_image_from_cg_image(_ imageHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CGImage = ci_borrow(imageHandle) else { return nil }
    return ci_retain(CIImage(cgImage: image))
}

@_cdecl("ci_image_from_cv_pixel_buffer")
public func ci_image_from_cv_pixel_buffer(_ bufferHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let buffer: CVPixelBuffer = ci_borrow(bufferHandle) else { return nil }
    return ci_retain(CIImage(cvPixelBuffer: buffer))
}

@_cdecl("ci_image_from_iosurface")
public func ci_image_from_iosurface(_ surfaceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let surface: IOSurface = ci_borrow(surfaceHandle) else { return nil }
    return ci_retain(CIImage(ioSurface: surface))
}

@_cdecl("ci_image_from_color")
public func ci_image_from_color(_ colorHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let color: CIColor = ci_borrow(colorHandle) else { return nil }
    return ci_retain(CIImage(color: color))
}

@_cdecl("ci_image_empty")
public func ci_image_empty() -> UnsafeMutableRawPointer? {
    let image = CIImage(color: CIColor(red: 0, green: 0, blue: 0, alpha: 0)).cropped(to: .zero)
    return ci_retain(image)
}

@_cdecl("ci_image_named_constant")
public func ci_image_named_constant(_ kind: Int32) -> UnsafeMutableRawPointer? {
    guard let color = ci_color_named(kind) else { return nil }
    defer { ci_release(color) }
    guard let ciColor: CIColor = ci_borrow(color) else { return nil }
    return ci_retain(CIImage(color: ciColor))
}

@_cdecl("ci_image_from_bitmap")
public func ci_image_from_bitmap(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ width: Int,
    _ height: Int,
    _ bytesPerRow: Int,
    _ formatCode: Int32,
    _ useColorSpace: Bool,
    _ colorSpaceCode: Int32,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len > 0, width > 0, height > 0, bytesPerRow > 0, let outImage,
              let format = ci_image_format(from: formatCode)
        else {
            throw CIBridgeError.invalidArgument("missing bitmap bytes, format, or output image pointer")
        }
        let data = Data(bytes: bytes, count: len)
        let colorSpace = useColorSpace ? ci_color_space(from: colorSpaceCode) : nil
        let image = CIImage(
            bitmapData: data,
            bytesPerRow: bytesPerRow,
            size: CGSize(width: width, height: height),
            format: format,
            colorSpace: colorSpace
        )
        outImage.pointee = ci_retain(image)
    }
}

@_cdecl("ci_image_from_bitmap_rgba8")
public func ci_image_from_bitmap_rgba8(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ width: Int,
    _ height: Int,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_image_from_bitmap(
        bytes,
        len,
        width,
        height,
        width * 4,
        2,
        true,
        1,
        outImage,
        outError
    )
}

@_cdecl("ci_image_extent")
public func ci_image_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let image: CIImage = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }

    let extent = image.extent
    outX?.pointee = extent.origin.x
    outY?.pointee = extent.origin.y
    outWidth?.pointee = extent.size.width
    outHeight?.pointee = extent.size.height
}

@_cdecl("ci_image_is_opaque")
public func ci_image_is_opaque(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let image: CIImage = ci_borrow(handle) else { return false }
    return image.isOpaque
}

@_cdecl("ci_image_properties_json")
public func ci_image_properties_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let image: CIImage = ci_borrow(handle) else {
        return ci_string("{}")
    }
    return ci_string(ci_json_string(from: image.properties) ?? "{}")
}

@_cdecl("ci_image_cropped")
public func ci_image_cropped(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.cropped(to: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_image_clamped_to_extent")
public func ci_image_clamped_to_extent(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.clampedToExtent())
}

@_cdecl("ci_image_clamped_to_rect")
public func ci_image_clamped_to_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.clamped(to: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_image_applying_orientation")
public func ci_image_applying_orientation(
    _ handle: UnsafeMutableRawPointer?,
    _ exifOrientation: UInt32
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.oriented(forExifOrientation: Int32(exifOrientation)))
}

@_cdecl("ci_image_composited_over")
public func ci_image_composited_over(
    _ handle: UnsafeMutableRawPointer?,
    _ backgroundHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle),
          let background: CIImage = ci_borrow(backgroundHandle)
    else {
        return nil
    }
    return ci_retain(image.composited(over: background))
}

@_cdecl("ci_image_transformed")
public func ci_image_transformed(
    _ handle: UnsafeMutableRawPointer?,
    _ a: Double,
    _ b: Double,
    _ c: Double,
    _ d: Double,
    _ tx: Double,
    _ ty: Double
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    let transform = CGAffineTransform(a: a, b: b, c: c, d: d, tx: tx, ty: ty)
    return ci_retain(image.transformed(by: transform))
}

@_cdecl("ci_image_apply_filter_name")
public func ci_image_apply_filter_name(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle), let name else { return nil }
    return ci_retain(image.applyingFilter(String(cString: name)))
}

@_cdecl("ci_image_premultiplying_alpha")
public func ci_image_premultiplying_alpha(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.premultiplyingAlpha())
}

@_cdecl("ci_image_unpremultiplying_alpha")
public func ci_image_unpremultiplying_alpha(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.unpremultiplyingAlpha())
}

@_cdecl("ci_image_setting_alpha_one_in_extent")
public func ci_image_setting_alpha_one_in_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.settingAlphaOne(in: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_image_gaussian_blur")
public func ci_image_gaussian_blur(_ handle: UnsafeMutableRawPointer?, _ sigma: Double) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.applyingGaussianBlur(sigma: sigma))
}

@_cdecl("ci_image_sampling_linear")
public func ci_image_sampling_linear(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.samplingLinear())
}

@_cdecl("ci_image_sampling_nearest")
public func ci_image_sampling_nearest(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.samplingNearest())
}

@_cdecl("ci_image_insert_intermediate")
public func ci_image_insert_intermediate(_ handle: UnsafeMutableRawPointer?, _ cache: Bool) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    return ci_retain(image.insertingIntermediate(cache: cache))
}

@_cdecl("ci_image_apply_gain_map")
public func ci_image_apply_gain_map(
    _ handle: UnsafeMutableRawPointer?,
    _ gainMapHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle),
          let gainMap: CIImage = ci_borrow(gainMapHandle)
    else {
        return nil
    }
    if #available(macOS 15.0, *) {
        return ci_retain(image.applyingGainMap(gainMap))
    }
    return ci_retain(image)
}

@_cdecl("ci_image_apply_gain_map_headroom")
public func ci_image_apply_gain_map_headroom(
    _ handle: UnsafeMutableRawPointer?,
    _ gainMapHandle: UnsafeMutableRawPointer?,
    _ headroom: Float
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle),
          let gainMap: CIImage = ci_borrow(gainMapHandle)
    else {
        return nil
    }
    if #available(macOS 15.0, *) {
        return ci_retain(image.applyingGainMap(gainMap, headroom: headroom))
    }
    return ci_retain(image)
}

@_cdecl("ci_image_content_headroom")
public func ci_image_content_headroom(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let image: CIImage = ci_borrow(handle) else { return 0 }
    if #available(macOS 15.0, *) {
        return image.contentHeadroom
    }
    return 0
}

@_cdecl("ci_image_content_average_light_level")
public func ci_image_content_average_light_level(_ handle: UnsafeMutableRawPointer?) -> Float {
    guard let image: CIImage = ci_borrow(handle) else { return 0 }
    if #available(macOS 26.0, *) {
        return image.contentAverageLightLevel
    }
    return 0
}

@_cdecl("ci_image_setting_content_headroom")
public func ci_image_setting_content_headroom(
    _ handle: UnsafeMutableRawPointer?,
    _ headroom: Float
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    if #available(macOS 26.0, *) {
        return ci_retain(image.settingContentHeadroom(headroom))
    }
    return ci_retain(image)
}

@_cdecl("ci_image_setting_content_average_light_level")
public func ci_image_setting_content_average_light_level(
    _ handle: UnsafeMutableRawPointer?,
    _ average: Float
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(handle) else { return nil }
    if #available(macOS 26.0, *) {
        return ci_retain(image.settingContentAverageLightLevel(average))
    }
    return ci_retain(image)
}

@_cdecl("ci_image_region_of_interest_for_image")
public func ci_image_region_of_interest_for_image(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let image: CIImage = ci_borrow(handle),
          let other: CIImage = ci_borrow(imageHandle)
    else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = image.regionOfInterest(for: other, in: CGRect(x: x, y: y, width: width, height: height))
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}
