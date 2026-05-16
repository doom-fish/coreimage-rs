import CoreImage
import Foundation

private final class BridgePassthroughImageProcessorKernel: CIImageProcessorKernel {
    private static var lastInvocation: [String: Any] = [:]

    private class func record(
        input: CIImageProcessorInput?,
        output: CIImageProcessorOutput,
        inputCount: Int
    ) {
        var snapshot: [String: Any] = [
            "inputCount": inputCount,
            "outputRegion": ci_normalize_json(output.region),
            "outputBytesPerRow": output.bytesPerRow,
            "outputFormat": output.format,
            "hasOutputPixelBuffer": output.pixelBuffer != nil,
            "hasOutputMetalTexture": output.metalTexture != nil,
        ]
        if #available(macOS 13.0, *) {
            snapshot["outputDigest"] = output.digest
        }
        if let input {
            snapshot["inputRegion"] = ci_normalize_json(input.region)
            snapshot["inputBytesPerRow"] = input.bytesPerRow
            snapshot["inputFormat"] = input.format
            snapshot["hasInputPixelBuffer"] = input.pixelBuffer != nil
            snapshot["hasInputMetalTexture"] = input.metalTexture != nil
            if #available(macOS 13.0, *) {
                snapshot["inputDigest"] = input.digest
            }
            if #available(macOS 14.0, *) {
                snapshot["roiTileIndex"] = input.roiTileIndex
                snapshot["roiTileCount"] = input.roiTileCount
            }
        }
        lastInvocation = snapshot
    }

    override class func process(
        with inputs: [CIImageProcessorInput]?,
        arguments _: [String: Any]?,
        output: CIImageProcessorOutput
    ) throws {
        let input = inputs?.first
        record(input: input, output: output, inputCount: inputs?.count ?? 0)

        guard let input else {
            return
        }

        let rowBytes = min(Int(input.bytesPerRow), Int(output.bytesPerRow))
        let rows = max(Int(output.region.integral.height), 0)
        if rowBytes == 0 || rows == 0 {
            return
        }
        memcpy(output.baseAddress, input.baseAddress, rowBytes * rows)
    }

    class func lastInvocationJSON() -> String {
        ci_json_string(from: lastInvocation) ?? "{}"
    }
}

@_cdecl("ci_image_processor_apply_passthrough")
public func ci_image_processor_apply_passthrough(
    _ imageHandle: UnsafeMutableRawPointer?,
    _ outImage: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let image: CIImage = ci_borrow(imageHandle), let outImage else {
            throw CIBridgeError.invalidArgument("missing input image or output pointer")
        }
        let output = try BridgePassthroughImageProcessorKernel.apply(
            withExtent: image.extent,
            inputs: [image],
            arguments: nil
        )
        outImage.pointee = ci_retain(output)
    }
}

@_cdecl("ci_image_processor_last_invocation_json")
public func ci_image_processor_last_invocation_json() -> UnsafeMutablePointer<CChar>? {
    ci_string(BridgePassthroughImageProcessorKernel.lastInvocationJSON())
}
