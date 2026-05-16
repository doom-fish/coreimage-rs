import CoreGraphics
import CoreImage
import CoreVideo
import Foundation
import IOSurface
import ImageIO
import Metal

func ci_url(from path: UnsafePointer<CChar>?) throws -> URL {
    guard let path else {
        throw CIBridgeError.invalidArgument("missing output path")
    }
    return URL(fileURLWithPath: String(cString: path))
}

func ci_quality_options(_ quality: Double) -> [CIImageRepresentationOption: Any] {
    [CIImageRepresentationOption(rawValue: kCGImageDestinationLossyCompressionQuality as String): quality]
}

@_cdecl("ci_context_new_default")
public func ci_context_new_default() -> UnsafeMutableRawPointer? {
    ci_retain(CIContext(options: nil))
}

@_cdecl("ci_context_new_cpu")
public func ci_context_new_cpu() -> UnsafeMutableRawPointer? {
    ci_retain(CIContext(options: [CIContextOption.useSoftwareRenderer: true]))
}

@_cdecl("ci_context_new_with_options")
public func ci_context_new_with_options(
    _ cacheIntermediates: Bool,
    _ priorityRequestLow: Bool,
    _ allowLowPower: Bool,
    _ outputPremultiplied: Bool,
    _ highQualityDownsample: Bool,
    _ useOutputColorSpace: Bool,
    _ outputColorSpaceCode: Int32,
    _ useWorkingColorSpace: Bool,
    _ workingColorSpaceCode: Int32,
    _ useWorkingFormat: Bool,
    _ workingFormatCode: Int32,
    _ useMemoryLimit: Bool,
    _ memoryLimit: Double,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    var options: [CIContextOption: Any] = [
        CIContextOption.cacheIntermediates: cacheIntermediates,
        CIContextOption.priorityRequestLow: priorityRequestLow,
        CIContextOption.outputPremultiplied: outputPremultiplied,
        CIContextOption.highQualityDownsample: highQualityDownsample,
    ]
    if #available(macOS 10.12, *) {
        options[CIContextOption.allowLowPower] = allowLowPower
    }
    if useOutputColorSpace {
        options[CIContextOption.outputColorSpace] = ci_color_space(from: outputColorSpaceCode) ?? NSNull()
    }
    if useWorkingColorSpace {
        options[CIContextOption.workingColorSpace] = ci_color_space(from: workingColorSpaceCode) ?? NSNull()
    }
    if useWorkingFormat, let workingFormat = ci_image_format(from: workingFormatCode) {
        options[CIContextOption.workingFormat] = workingFormat
    }
    if useMemoryLimit, #available(macOS 14.0, *) {
        options[CIContextOption.memoryTarget] = memoryLimit
    }
    if let name {
        options[CIContextOption.name] = String(cString: name)
    }
    return ci_retain(CIContext(options: options))
}

@_cdecl("ci_context_new_metal_device")
public func ci_context_new_metal_device(_ deviceHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let device: MTLDevice = ci_borrow(deviceHandle) else { return nil }
    return ci_retain(CIContext(mtlDevice: device))
}

@_cdecl("ci_context_new_metal_command_queue")
public func ci_context_new_metal_command_queue(_ queueHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let queue: MTLCommandQueue = ci_borrow(queueHandle) else { return nil }
    return ci_retain(CIContext(mtlCommandQueue: queue))
}

@_cdecl("ci_context_working_format")
public func ci_context_working_format(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let context: CIContext = ci_borrow(handle) else { return 0 }
    return context.workingFormat.rawValue
}

@_cdecl("ci_context_create_cg_image")
public func ci_context_create_cg_image(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let context: CIContext = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle),
          let rendered = context.createCGImage(image, from: image.extent)
    else {
        return nil
    }
    return ci_retain(rendered)
}

@_cdecl("ci_context_render_to_cv_pixel_buffer")
public func ci_context_render_to_cv_pixel_buffer(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ bufferHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle),
              let buffer: CVPixelBuffer = ci_borrow(bufferHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext, CIImage, or CVPixelBuffer handle")
        }
        context.render(image, to: buffer, bounds: image.extent, colorSpace: ci_srgb_color_space())
    }
}

@_cdecl("ci_context_render_to_iosurface")
public func ci_context_render_to_iosurface(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ surfaceHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle),
              let surface: IOSurface = ci_borrow(surfaceHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext, CIImage, or IOSurface handle")
        }
        context.render(image, to: surface, bounds: image.extent, colorSpace: ci_srgb_color_space())
    }
}

@_cdecl("ci_context_reclaim_resources")
public func ci_context_reclaim_resources(_ handle: UnsafeMutableRawPointer?) {
    guard let context: CIContext = ci_borrow(handle) else { return }
    context.reclaimResources()
}

@_cdecl("ci_context_clear_caches")
public func ci_context_clear_caches(_ handle: UnsafeMutableRawPointer?) {
    guard let context: CIContext = ci_borrow(handle) else { return }
    context.clearCaches()
}

@_cdecl("ci_context_input_image_maximum_size")
public func ci_context_input_image_maximum_size(
    _ handle: UnsafeMutableRawPointer?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let _: CIContext = ci_borrow(handle) else {
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    outWidth?.pointee = 0
    outHeight?.pointee = 0
}

@_cdecl("ci_context_output_image_maximum_size")
public func ci_context_output_image_maximum_size(
    _ handle: UnsafeMutableRawPointer?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let _: CIContext = ci_borrow(handle) else {
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    outWidth?.pointee = 0
    outHeight?.pointee = 0
}

@_cdecl("ci_context_write_png")
public func ci_context_write_png(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        try context.writePNGRepresentation(
            of: image,
            to: ci_url(from: path),
            format: .RGBA8,
            colorSpace: ci_srgb_color_space()
        )
    }
}

@_cdecl("ci_context_write_jpeg")
public func ci_context_write_jpeg(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ quality: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        try context.writeJPEGRepresentation(
            of: image,
            to: ci_url(from: path),
            colorSpace: ci_srgb_color_space(),
            options: ci_quality_options(quality)
        )
    }
}

@_cdecl("ci_context_write_heif")
public func ci_context_write_heif(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ quality: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        try context.writeHEIFRepresentation(
            of: image,
            to: ci_url(from: path),
            format: .RGBA8,
            colorSpace: ci_srgb_color_space(),
            options: ci_quality_options(quality)
        )
    }
}

@_cdecl("ci_context_write_heif10")
public func ci_context_write_heif10(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ quality: Double,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        if #available(macOS 12.0, *) {
            try context.writeHEIF10Representation(
                of: image,
                to: ci_url(from: path),
                colorSpace: ci_srgb_color_space(),
                options: ci_quality_options(quality)
            )
        } else {
            throw CIBridgeError.unsupported("HEIF10 output requires macOS 12.0")
        }
    }
}

@_cdecl("ci_context_write_tiff")
public func ci_context_write_tiff(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        try context.writeTIFFRepresentation(
            of: image,
            to: ci_url(from: path),
            format: .RGBA8,
            colorSpace: ci_srgb_color_space(),
            options: [:]
        )
    }
}

@_cdecl("ci_context_write_openexr")
public func ci_context_write_openexr(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing CIContext or CIImage handle")
        }
        if #available(macOS 14.0, *) {
            try context.writeOpenEXRRepresentation(of: image, to: ci_url(from: path), options: [:])
        } else {
            throw CIBridgeError.unsupported("OpenEXR output requires macOS 14.0")
        }
    }
}
