import CoreGraphics
import CoreImage
import Foundation

private func ci_kernel_result(_ image: CIImage?, _ kind: String) -> UnsafeMutableRawPointer? {
    guard let image else { return nil }
    return ci_retain(image)
}

private func ci_builtin_blend_kernel(_ kind: Int32) -> CIBlendKernel? {
    switch kind {
    case 0: return .componentAdd
    case 1: return .componentMultiply
    case 2: return .componentMin
    case 3: return .componentMax
    case 4: return .clear
    case 5: return .source
    case 6: return .destination
    case 7: return .sourceOver
    case 8: return .destinationOver
    case 9: return .sourceIn
    case 10: return .destinationIn
    case 11: return .sourceOut
    case 12: return .destinationOut
    case 13: return .sourceAtop
    case 14: return .destinationAtop
    case 15: return .exclusiveOr
    case 16: return .multiply
    case 17: return .screen
    case 18: return .overlay
    case 19: return .darken
    case 20: return .lighten
    case 21: return .colorDodge
    case 22: return .colorBurn
    case 23: return .hardLight
    case 24: return .softLight
    case 25: return .difference
    case 26: return .exclusion
    case 27: return .hue
    case 28: return .saturation
    case 29: return .color
    case 30: return .luminosity
    case 31: return .subtract
    case 32: return .divide
    case 33: return .linearBurn
    case 34: return .linearDodge
    case 35: return .vividLight
    case 36: return .linearLight
    case 37: return .pinLight
    case 38: return .hardMix
    case 39: return .darkerColor
    case 40: return .lighterColor
    default: return nil
    }
}

@_cdecl("ci_color_kernel_new_source")
public func ci_color_kernel_new_source(
    _ source: UnsafePointer<CChar>?,
    _ outKernel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let source, let outKernel else {
            throw CIBridgeError.invalidArgument("missing color kernel source or output pointer")
        }
        guard let kernel = CIColorKernel(source: String(cString: source)) else {
            throw CIBridgeError.nullResult("CIColorKernel(source:) returned nil")
        }
        outKernel.pointee = ci_retain(kernel)
    }
}

@_cdecl("ci_warp_kernel_new_source")
public func ci_warp_kernel_new_source(
    _ source: UnsafePointer<CChar>?,
    _ outKernel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let source, let outKernel else {
            throw CIBridgeError.invalidArgument("missing warp kernel source or output pointer")
        }
        guard let kernel = CIWarpKernel(source: String(cString: source)) else {
            throw CIBridgeError.nullResult("CIWarpKernel(source:) returned nil")
        }
        outKernel.pointee = ci_retain(kernel)
    }
}

@_cdecl("ci_blend_kernel_builtin")
public func ci_blend_kernel_builtin(_ kind: Int32) -> UnsafeMutableRawPointer? {
    guard let kernel = ci_builtin_blend_kernel(kind) else { return nil }
    return ci_retain(kernel)
}

@_cdecl("ci_kernel_name")
public func ci_kernel_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let kernel: CIKernel = ci_borrow(handle) else { return nil }
    return ci_string(kernel.name)
}

@_cdecl("ci_color_kernel_apply_image_scalar")
public func ci_color_kernel_apply_image_scalar(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ value: Double,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIColorKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    return ci_kernel_result(kernel.apply(extent: extent, arguments: [image, value]), "color")
}

@_cdecl("ci_color_kernel_apply_image_color")
public func ci_color_kernel_apply_image_color(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ colorHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIColorKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle),
          let color: CIColor = ci_borrow(colorHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    return ci_kernel_result(kernel.apply(extent: extent, arguments: [image, color]), "color")
}

@_cdecl("ci_color_kernel_apply_image_vector")
public func ci_color_kernel_apply_image_vector(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ vectorHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIColorKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle),
          let vector: CIVector = ci_borrow(vectorHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    return ci_kernel_result(kernel.apply(extent: extent, arguments: [image, vector]), "color")
}

@_cdecl("ci_warp_kernel_apply_image_scalar")
public func ci_warp_kernel_apply_image_scalar(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ value: Double,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIWarpKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    return ci_kernel_result(kernel.apply(extent: extent, roiCallback: { _, rect in rect }, image: image, arguments: [value]), "warp")
}

@_cdecl("ci_blend_kernel_apply")
public func ci_blend_kernel_apply(
    _ handle: UnsafeMutableRawPointer?,
    _ foregroundHandle: UnsafeMutableRawPointer?,
    _ backgroundHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIBlendKernel = ci_borrow(handle),
          let foreground: CIImage = ci_borrow(foregroundHandle),
          let background: CIImage = ci_borrow(backgroundHandle)
    else {
        return nil
    }
    return ci_kernel_result(kernel.apply(foreground: foreground, background: background), "blend")
}
