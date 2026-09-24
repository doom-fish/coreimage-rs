import CoreImage
import CoreImageObjCBridge
import Foundation

public typealias CIXProcessorCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    Int,
    UnsafePointer<UnsafeRawPointer?>?,
    UnsafePointer<Int>?,
    UnsafePointer<Int32>?,
    UnsafePointer<Double>?,
    UnsafeMutableRawPointer?,
    Int,
    Int32,
    Double,
    Double,
    Double,
    Double,
    UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool

private let ciProcessorInvocationKey = "coreimage-rs.processor-invocation"

private final class BridgeProcessorInvocation: NSObject {
    let owner: BridgeRegionOfInterestCallback
    let process: CIXProcessorCallback

    init(owner: BridgeRegionOfInterestCallback, process: @escaping CIXProcessorCallback) {
        self.owner = owner
        self.process = process
    }
}

private class BridgeClosureImageProcessorKernel: CIImageProcessorKernel {
    class var bridgeFormat: CIFormat { CIFormat(rawValue: 0) }

    override class func formatForInput(at input: Int32) -> CIFormat {
        bridgeFormat
    }

    override class var outputFormat: CIFormat {
        bridgeFormat
    }

    override class func roi(
        forInput input: Int32,
        arguments: [String: Any]?,
        outputRect: CGRect
    ) -> CGRect {
        guard let invocation = arguments?[ciProcessorInvocationKey] as? BridgeProcessorInvocation else {
            return outputRect
        }
        return invocation.owner.region(inputIndex: input, destination: outputRect)
    }

    override class func process(
        with inputs: [CIImageProcessorInput]?,
        arguments: [String: Any]?,
        output: CIImageProcessorOutput
    ) throws {
        guard let invocation = arguments?[ciProcessorInvocationKey] as? BridgeProcessorInvocation else {
            throw CIBridgeError.invalidArgument("the image processor invocation is missing")
        }
        let inputs = inputs ?? []
        let bases: [UnsafeRawPointer?] = inputs.map { $0.baseAddress }
        let bytesPerRow: [Int] = inputs.map { $0.bytesPerRow }
        let formats: [Int32] = inputs.map { $0.format.rawValue }
        let regions: [Double] = inputs.flatMap { input in
            [
                Double(input.region.origin.x),
                Double(input.region.origin.y),
                Double(input.region.size.width),
                Double(input.region.size.height),
            ]
        }
        var message: UnsafeMutablePointer<CChar>?
        let succeeded = invocation.process(
            invocation.owner.context,
            inputs.count,
            bases,
            bytesPerRow,
            formats,
            regions,
            output.baseAddress,
            output.bytesPerRow,
            output.format.rawValue,
            Double(output.region.origin.x),
            Double(output.region.origin.y),
            Double(output.region.size.width),
            Double(output.region.size.height),
            &message
        )
        guard succeeded else {
            let text = message.map { String(cString: $0) } ?? "the image processor closure failed"
            free(message)
            throw NSError(
                domain: "coreimage-rs",
                code: 1,
                userInfo: [NSLocalizedDescriptionKey: text]
            )
        }
    }
}

private final class BridgeClosureImageProcessorKernelBGRA8: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .BGRA8 }
}

private final class BridgeClosureImageProcessorKernelRGBAh: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .RGBAh }
}

private final class BridgeClosureImageProcessorKernelRGBAf: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .RGBAf }
}

private final class BridgeClosureImageProcessorKernelR8: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .R8 }
}

private final class BridgeClosureImageProcessorKernelRh: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .Rh }
}

private final class BridgeClosureImageProcessorKernelRf: BridgeClosureImageProcessorKernel {
    override class var bridgeFormat: CIFormat { .Rf }
}

private func ci_closure_processor_class(_ format: Int32) -> BridgeClosureImageProcessorKernel.Type? {
    switch format {
    case 0: return BridgeClosureImageProcessorKernel.self
    case CIFormat.BGRA8.rawValue: return BridgeClosureImageProcessorKernelBGRA8.self
    case CIFormat.RGBAh.rawValue: return BridgeClosureImageProcessorKernelRGBAh.self
    case CIFormat.RGBAf.rawValue: return BridgeClosureImageProcessorKernelRGBAf.self
    case CIFormat.R8.rawValue: return BridgeClosureImageProcessorKernelR8.self
    case CIFormat.Rh.rawValue: return BridgeClosureImageProcessorKernelRh.self
    case CIFormat.Rf.rawValue: return BridgeClosureImageProcessorKernelRf.self
    default: return nil
    }
}

@_cdecl("ci_image_processor_apply_closure")
public func ci_image_processor_apply_closure(
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ inputHandles: UnsafePointer<UnsafeMutableRawPointer?>?,
    _ inputCount: Int,
    _ format: Int32,
    _ context: UnsafeMutableRawPointer?,
    _ process: CIXProcessorCallback?,
    _ regionOfInterest: CIXRegionOfInterestCallback?,
    _ releaseCallback: CIXContextReleaseCallback?,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    let owner = BridgeRegionOfInterestCallback(
        context: context,
        callback: regionOfInterest,
        releaseCallback: releaseCallback
    )
    return ci_run(outError) {
        guard owner.isComplete, let process, let outImage else {
            throw CIBridgeError.invalidArgument("missing image processor callbacks or output pointer")
        }
        guard let kernelClass = ci_closure_processor_class(format) else {
            throw CIBridgeError.invalidArgument("unsupported image processor format \(format)")
        }
        var inputs: [CIImage] = []
        if inputCount > 0 {
            guard let inputHandles else {
                throw CIBridgeError.invalidArgument("missing image processor inputs")
            }
            for index in 0 ..< inputCount {
                let object: AnyObject? = ci_borrow(inputHandles[index])
                guard let image = object as? CIImage else {
                    throw CIBridgeError.invalidArgument("image processor input \(index) is not a CIImage")
                }
                inputs.append(image)
            }
        }
        let invocation = BridgeProcessorInvocation(owner: owner, process: process)
        var error: NSError?
        let image = CIXTryApplyImageProcessor(
            kernelClass,
            CGRect(x: x, y: y, width: width, height: height),
            inputs,
            [ciProcessorInvocationKey: invocation],
            &error
        )
        if let error {
            throw CIBridgeError.framework(error)
        }
        guard let image else {
            throw CIBridgeError.nullResult("CIImageProcessorKernel.apply(withExtent:inputs:arguments:) returned nil")
        }
        outImage.pointee = ci_retain(image)
    }
}
