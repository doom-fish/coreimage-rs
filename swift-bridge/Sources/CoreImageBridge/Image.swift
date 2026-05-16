import CoreGraphics
import CoreImage
import CoreVideo
import Foundation
import IOSurface

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

@_cdecl("ci_image_from_bitmap_rgba8")
public func ci_image_from_bitmap_rgba8(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ width: Int,
    _ height: Int,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len > 0, width > 0, height > 0, let outImage else {
            throw CIBridgeError.invalidArgument("missing bitmap bytes or output image pointer")
        }
        let data = Data(bytes: bytes, count: len)
        let image = CIImage(
            bitmapData: data,
            bytesPerRow: width * 4,
            size: CGSize(width: width, height: height),
            format: .RGBA8,
            colorSpace: ci_srgb_color_space()
        )
        outImage.pointee = ci_retain(image)
    }
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
