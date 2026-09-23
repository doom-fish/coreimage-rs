import CoreGraphics
import CoreImage
import CoreImageObjCBridge
import Foundation

public typealias CIXRegionOfInterestCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int32,
    Double,
    Double,
    Double,
    Double,
    UnsafeMutablePointer<Double>?,
    UnsafeMutablePointer<Double>?,
    UnsafeMutablePointer<Double>?,
    UnsafeMutablePointer<Double>?
) -> Void
public typealias CIXContextReleaseCallback = @convention(c) (
    UnsafeMutableRawPointer?
) -> Void

final class BridgeRegionOfInterestCallback {
    private let context: UnsafeMutableRawPointer?
    private let callback: CIXRegionOfInterestCallback?
    private let releaseCallback: CIXContextReleaseCallback?

    init(
        context: UnsafeMutableRawPointer?,
        callback: CIXRegionOfInterestCallback?,
        releaseCallback: CIXContextReleaseCallback?
    ) {
        self.context = context
        self.callback = callback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback?(context)
    }

    var isComplete: Bool {
        context != nil && callback != nil && releaseCallback != nil
    }

    func region(inputIndex: Int32, destination: CGRect) -> CGRect {
        guard let callback else { return destination }
        var x = Double(destination.origin.x)
        var y = Double(destination.origin.y)
        var width = Double(destination.size.width)
        var height = Double(destination.size.height)
        callback(
            context,
            inputIndex,
            destination.origin.x,
            destination.origin.y,
            destination.size.width,
            destination.size.height,
            &x,
            &y,
            &width,
            &height
        )
        return CGRect(x: x, y: y, width: width, height: height)
    }
}

private func ci_kernel_result(_ image: CIImage?, _ kind: String) -> UnsafeMutableRawPointer? {
    guard let image else { return nil }
    return ci_retain(image)
}

private func ci_kernel_class(_ kind: Int32) -> CIKernel.Type? {
    switch kind {
    case 0: return CIKernel.self
    case 1: return CIColorKernel.self
    case 2: return CIWarpKernel.self
    case 3: return CIBlendKernel.self
    default: return nil
    }
}

private func ci_kernel_arguments(
    _ kinds: UnsafePointer<Int32>?,
    _ scalars: UnsafePointer<Double>?,
    _ objects: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ count: Int
) throws -> [Any] {
    guard count > 0 else { return [] }
    guard let kinds, let scalars, let objects else {
        throw CIBridgeError.invalidArgument("missing kernel argument arrays")
    }
    var arguments: [Any] = []
    arguments.reserveCapacity(count)
    for index in 0 ..< count {
        let object: AnyObject? = ci_borrow(objects[index])
        switch kinds[index] {
        case 0:
            guard let image = object as? CIImage else {
                throw CIBridgeError.invalidArgument("kernel argument \(index) is not a CIImage")
            }
            arguments.append(image)
        case 1:
            arguments.append(NSNumber(value: scalars[index]))
        case 2:
            guard let vector = object as? CIVector else {
                throw CIBridgeError.invalidArgument("kernel argument \(index) is not a CIVector")
            }
            arguments.append(vector)
        case 3:
            guard let color = object as? CIColor else {
                throw CIBridgeError.invalidArgument("kernel argument \(index) is not a CIColor")
            }
            arguments.append(color)
        default:
            throw CIBridgeError.invalidArgument("kernel argument \(index) has unknown kind \(kinds[index])")
        }
    }
    return arguments
}

private func ci_kernel_output(
    _ image: CIImage?,
    _ error: NSError?,
    _ method: String,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>
) throws {
    if let error {
        throw CIBridgeError.framework(error)
    }
    guard let image else {
        throw CIBridgeError.nullResult(
            "\(method) returned nil; the arguments do not match the kernel's parameters"
        )
    }
    outImage.pointee = ci_retain(image)
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
    _ height: Double,
    _ useDestinationRect: Bool
) -> UnsafeMutableRawPointer? {
    guard let kernel: CIWarpKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    let output = kernel.apply(
        extent: extent,
        roiCallback: { _, destination in
            useDestinationRect ? destination : image.extent
        },
        image: image,
        arguments: [value]
    )
    return ci_kernel_result(output, "warp")
}

@_cdecl("ci_warp_kernel_apply_image_scalar_with_roi")
public func ci_warp_kernel_apply_image_scalar_with_roi(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ value: Double,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ context: UnsafeMutableRawPointer?,
    _ callback: CIXRegionOfInterestCallback?,
    _ releaseCallback: CIXContextReleaseCallback?
) -> UnsafeMutableRawPointer? {
    let callbackHolder = BridgeRegionOfInterestCallback(
        context: context,
        callback: callback,
        releaseCallback: releaseCallback
    )
    guard callbackHolder.isComplete,
          let kernel: CIWarpKernel = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return nil
    }
    let extent = CGRect(x: x, y: y, width: width, height: height)
    let output = kernel.apply(
        extent: extent,
        roiCallback: { inputIndex, destination in
            callbackHolder.region(inputIndex: inputIndex, destination: destination)
        },
        image: image,
        arguments: [value]
    )
    return ci_kernel_result(output, "warp")
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

@_cdecl("ci_kernel_new_metal_library")
public func ci_kernel_new_metal_library(
    _ kind: Int32,
    _ functionName: UnsafePointer<CChar>?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ hasOutputFormat: Bool,
    _ outputFormat: Int32,
    _ outKernel: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let functionName, let bytes, len > 0, let outKernel,
              let kernelClass = ci_kernel_class(kind)
        else {
            throw CIBridgeError.invalidArgument("missing kernel function name, library data, or output pointer")
        }
        let name = String(cString: functionName)
        let data = Data(bytes: bytes, count: len)
        let kernel: CIKernel
        do {
            if hasOutputFormat {
                kernel = try kernelClass.init(
                    functionName: name,
                    fromMetalLibraryData: data,
                    outputPixelFormat: CIFormat(rawValue: outputFormat)
                )
            } else {
                kernel = try kernelClass.init(functionName: name, fromMetalLibraryData: data)
            }
        } catch {
            throw CIBridgeError.framework(error)
        }
        outKernel.pointee = ci_retain(kernel)
    }
}

@_cdecl("ci_kernel_names_metal_library")
public func ci_kernel_names_metal_library(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int
) -> UnsafeMutablePointer<CChar>? {
    guard let bytes, len > 0 else { return ci_string("") }
    let names = CIKernel.kernelNames(fromMetalLibraryData: Data(bytes: bytes, count: len))
    return ci_string(names.joined(separator: "\n"))
}

@_cdecl("ci_kernels_new_metal_source")
public func ci_kernels_new_metal_source(
    _ source: UnsafePointer<CChar>?,
    _ outKernels: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let source, let outKernels else {
            throw CIBridgeError.invalidArgument("missing Metal source or output pointer")
        }
        guard #available(macOS 12.0, *) else {
            throw CIBridgeError.unsupported("CIKernel.kernels(withMetalString:) requires macOS 12.0 or later")
        }
        let kernels: [CIKernel]
        do {
            kernels = try CIKernel.kernels(withMetalString: String(cString: source))
        } catch {
            throw CIBridgeError.framework(error)
        }
        outKernels.pointee = ci_retain(kernels as NSArray)
    }
}

@_cdecl("ci_kernel_is_kind")
public func ci_kernel_is_kind(_ handle: UnsafeMutableRawPointer?, _ kind: Int32) -> Bool {
    let object: AnyObject? = ci_borrow(handle)
    switch kind {
    case 0: return object is CIKernel
    case 1: return object is CIColorKernel
    case 2: return object is CIWarpKernel
    case 3: return object is CIBlendKernel
    default: return false
    }
}

@_cdecl("ci_kernel_apply_arguments")
public func ci_kernel_apply_arguments(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ argumentKinds: UnsafePointer<Int32>?,
    _ argumentScalars: UnsafePointer<Double>?,
    _ argumentObjects: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ argumentCount: Int,
    _ context: UnsafeMutableRawPointer?,
    _ callback: CIXRegionOfInterestCallback?,
    _ releaseCallback: CIXContextReleaseCallback?,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let callbackHolder = BridgeRegionOfInterestCallback(
        context: context,
        callback: callback,
        releaseCallback: releaseCallback
    )
    return ci_run(outError) {
        guard callbackHolder.isComplete, let outImage,
              let kernel: CIKernel = ci_borrow(handle)
        else {
            throw CIBridgeError.invalidArgument("missing kernel, region-of-interest callback, or output pointer")
        }
        let arguments = try ci_kernel_arguments(argumentKinds, argumentScalars, argumentObjects, argumentCount)
        var error: NSError?
        let output = CIXTryApplyKernel(
            kernel,
            CGRect(x: x, y: y, width: width, height: height),
            { inputIndex, destination in
                callbackHolder.region(inputIndex: inputIndex, destination: destination)
            },
            arguments,
            &error
        )
        try ci_kernel_output(output, error, "CIKernel.apply(extent:roiCallback:arguments:)", outImage)
    }
}

@_cdecl("ci_color_kernel_apply_arguments")
public func ci_color_kernel_apply_arguments(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ argumentKinds: UnsafePointer<Int32>?,
    _ argumentScalars: UnsafePointer<Double>?,
    _ argumentObjects: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ argumentCount: Int,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let outImage, let kernel: CIColorKernel = ci_borrow(handle) else {
            throw CIBridgeError.invalidArgument("missing color kernel or output pointer")
        }
        let arguments = try ci_kernel_arguments(argumentKinds, argumentScalars, argumentObjects, argumentCount)
        var error: NSError?
        let output = CIXTryApplyColorKernel(
            kernel,
            CGRect(x: x, y: y, width: width, height: height),
            arguments,
            &error
        )
        try ci_kernel_output(output, error, "CIColorKernel.apply(extent:arguments:)", outImage)
    }
}

@_cdecl("ci_warp_kernel_apply_arguments")
public func ci_warp_kernel_apply_arguments(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ argumentKinds: UnsafePointer<Int32>?,
    _ argumentScalars: UnsafePointer<Double>?,
    _ argumentObjects: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ argumentCount: Int,
    _ context: UnsafeMutableRawPointer?,
    _ callback: CIXRegionOfInterestCallback?,
    _ releaseCallback: CIXContextReleaseCallback?,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let callbackHolder = BridgeRegionOfInterestCallback(
        context: context,
        callback: callback,
        releaseCallback: releaseCallback
    )
    return ci_run(outError) {
        guard callbackHolder.isComplete, let outImage,
              let kernel: CIWarpKernel = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle)
        else {
            throw CIBridgeError.invalidArgument("missing warp kernel, input image, region-of-interest callback, or output pointer")
        }
        let arguments = try ci_kernel_arguments(argumentKinds, argumentScalars, argumentObjects, argumentCount)
        var error: NSError?
        let output = CIXTryApplyWarpKernel(
            kernel,
            CGRect(x: x, y: y, width: width, height: height),
            { inputIndex, destination in
                callbackHolder.region(inputIndex: inputIndex, destination: destination)
            },
            image,
            arguments,
            &error
        )
        try ci_kernel_output(output, error, "CIWarpKernel.apply(extent:roiCallback:image:arguments:)", outImage)
    }
}
