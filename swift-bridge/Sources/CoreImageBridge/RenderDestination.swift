import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_render_destination_new_bitmap_data")
public func ci_render_destination_new_bitmap_data(
    _ data: UnsafeMutableRawPointer?,
    _ len: Int,
    _ width: Int,
    _ height: Int,
    _ bytesPerRow: Int,
    _ formatCode: Int32,
    _ useColorSpace: Bool,
    _ colorSpaceCode: Int32
) -> UnsafeMutableRawPointer? {
    guard let data, len >= bytesPerRow * height, width > 0, height > 0,
          let format = ci_image_format(from: formatCode)
    else {
        return nil
    }
    let destination = CIRenderDestination(
        bitmapData: data,
        width: width,
        height: height,
        bytesPerRow: bytesPerRow,
        format: format
    )
    if useColorSpace {
        destination.colorSpace = ci_color_space(from: colorSpaceCode)
    }
    return ci_retain(destination)
}

@_cdecl("ci_render_destination_width")
public func ci_render_destination_width(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return 0 }
    return destination.width
}

@_cdecl("ci_render_destination_height")
public func ci_render_destination_height(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return 0 }
    return destination.height
}

@_cdecl("ci_render_destination_alpha_mode")
public func ci_render_destination_alpha_mode(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return 0 }
    return Int32(destination.alphaMode.rawValue)
}

@_cdecl("ci_render_destination_set_alpha_mode")
public func ci_render_destination_set_alpha_mode(_ handle: UnsafeMutableRawPointer?, _ mode: Int32) {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return }
    guard let alphaMode = CIRenderDestinationAlphaMode(rawValue: UInt(mode)) else { return }
    destination.alphaMode = alphaMode
}

@_cdecl("ci_render_destination_is_flipped")
public func ci_render_destination_is_flipped(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return false }
    return destination.isFlipped
}

@_cdecl("ci_render_destination_set_flipped")
public func ci_render_destination_set_flipped(_ handle: UnsafeMutableRawPointer?, _ flipped: Bool) {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return }
    destination.isFlipped = flipped
}

@_cdecl("ci_render_destination_is_dithered")
public func ci_render_destination_is_dithered(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return false }
    return destination.isDithered
}

@_cdecl("ci_render_destination_set_dithered")
public func ci_render_destination_set_dithered(_ handle: UnsafeMutableRawPointer?, _ dithered: Bool) {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return }
    destination.isDithered = dithered
}

@_cdecl("ci_render_destination_is_clamped")
public func ci_render_destination_is_clamped(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return false }
    return destination.isClamped
}

@_cdecl("ci_render_destination_set_clamped")
public func ci_render_destination_set_clamped(_ handle: UnsafeMutableRawPointer?, _ clamped: Bool) {
    guard let destination: CIRenderDestination = ci_borrow(handle) else { return }
    destination.isClamped = clamped
}

@_cdecl("ci_context_start_render_task")
public func ci_context_start_render_task(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle),
              let destination: CIRenderDestination = ci_borrow(destinationHandle),
              let outTask
        else {
            throw CIBridgeError.invalidArgument("missing context, image, destination, or task output")
        }
        outTask.pointee = ci_retain(try context.startTask(toRender: image, to: destination))
    }
}

@_cdecl("ci_context_prepare_render")
public func ci_context_prepare_render(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle),
              let destination: CIRenderDestination = ci_borrow(destinationHandle)
        else {
            throw CIBridgeError.invalidArgument("missing context, image, or destination")
        }
        try context.prepareRender(image, from: image.extent, to: destination, at: .zero)
    }
}

@_cdecl("ci_context_start_clear_task")
public func ci_context_start_clear_task(
    _ handle: UnsafeMutableRawPointer?,
    _ destinationHandle: UnsafeMutableRawPointer?,
    _ outTask: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let context: CIContext = ci_borrow(handle),
              let destination: CIRenderDestination = ci_borrow(destinationHandle),
              let outTask
        else {
            throw CIBridgeError.invalidArgument("missing context, destination, or task output")
        }
        outTask.pointee = ci_retain(try context.startTask(toClear: destination))
    }
}

@_cdecl("ci_render_task_wait_until_completed")
public func ci_render_task_wait_until_completed(
    _ handle: UnsafeMutableRawPointer?,
    _ outInfo: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let task: CIRenderTask = ci_borrow(handle), let outInfo else {
            throw CIBridgeError.invalidArgument("missing render task or info output")
        }
        outInfo.pointee = ci_retain(try task.waitUntilCompleted())
    }
}

@_cdecl("ci_render_info_kernel_execution_time")
public func ci_render_info_kernel_execution_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let info: CIRenderInfo = ci_borrow(handle) else { return 0 }
    return info.kernelExecutionTime
}

@_cdecl("ci_render_info_kernel_compile_time")
public func ci_render_info_kernel_compile_time(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let info: CIRenderInfo = ci_borrow(handle) else { return 0 }
    if #available(macOS 14.0, *) {
        return info.kernelCompileTime
    }
    return 0
}

@_cdecl("ci_render_info_pass_count")
public func ci_render_info_pass_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let info: CIRenderInfo = ci_borrow(handle) else { return 0 }
    return info.passCount
}

@_cdecl("ci_render_info_pixels_processed")
public func ci_render_info_pixels_processed(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let info: CIRenderInfo = ci_borrow(handle) else { return 0 }
    return info.pixelsProcessed
}
