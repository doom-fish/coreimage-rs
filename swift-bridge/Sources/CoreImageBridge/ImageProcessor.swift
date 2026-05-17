import CoreImage
import Foundation

private struct BridgeImageProcessorOutputSnapshot {
    var region: CGRect = .zero
    var bytesPerRow: Int = 0
    var format: Int32 = 0
    var hasPixelBuffer = false
    var hasMetalTexture = false
    var digest: String?

    init() {}

    init(output: CIImageProcessorOutput) {
        region = output.region
        bytesPerRow = Int(output.bytesPerRow)
        format = Int32(output.format.rawValue)
        hasPixelBuffer = output.pixelBuffer != nil
        hasMetalTexture = output.metalTexture != nil
        if #available(macOS 13.0, *) {
            digest = String(output.digest)
        }
    }

    var jsonObject: [String: Any] {
        var snapshot: [String: Any] = [
            "region": ci_normalize_json(region),
            "bytesPerRow": bytesPerRow,
            "format": format,
            "hasPixelBuffer": hasPixelBuffer,
            "hasMetalTexture": hasMetalTexture,
        ]
        if let digest {
            snapshot["digest"] = digest
        }
        return snapshot
    }
}

private struct BridgeImageProcessorInputSnapshot {
    var region: CGRect = .zero
    var bytesPerRow: Int = 0
    var format: Int32 = 0
    var hasPixelBuffer = false
    var hasMetalTexture = false
    var digest: String?
    var roiTileIndex: Int?
    var roiTileCount: Int?

    init(input: CIImageProcessorInput) {
        region = input.region
        bytesPerRow = Int(input.bytesPerRow)
        format = Int32(input.format.rawValue)
        hasPixelBuffer = input.pixelBuffer != nil
        hasMetalTexture = input.metalTexture != nil
        if #available(macOS 13.0, *) {
            digest = String(input.digest)
        }
        if #available(macOS 14.0, *) {
            roiTileIndex = input.roiTileIndex
            roiTileCount = input.roiTileCount
        }
    }

    var jsonObject: [String: Any] {
        var snapshot: [String: Any] = [
            "region": ci_normalize_json(region),
            "bytesPerRow": bytesPerRow,
            "format": format,
            "hasPixelBuffer": hasPixelBuffer,
            "hasMetalTexture": hasMetalTexture,
        ]
        if let digest {
            snapshot["digest"] = digest
        }
        if let roiTileIndex {
            snapshot["roiTileIndex"] = roiTileIndex
        }
        if let roiTileCount {
            snapshot["roiTileCount"] = roiTileCount
        }
        return snapshot
    }
}

private struct BridgeImageProcessorInvocationSnapshot {
    var inputCount = 0
    var input: BridgeImageProcessorInputSnapshot?
    var output = BridgeImageProcessorOutputSnapshot()

    var jsonObject: [String: Any] {
        var snapshot: [String: Any] = [
            "inputCount": inputCount,
            "output": output.jsonObject,
        ]
        if let input {
            snapshot["input"] = input.jsonObject
        }
        return snapshot
    }
}

private func ci_write_rect(
    _ rect: CGRect,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}

private final class BridgePassthroughImageProcessorKernel: CIImageProcessorKernel {
    private static var lastInvocation = BridgeImageProcessorInvocationSnapshot()

    private class func record(
        input: CIImageProcessorInput?,
        output: CIImageProcessorOutput,
        inputCount: Int
    ) {
        var snapshot = BridgeImageProcessorInvocationSnapshot()
        snapshot.inputCount = inputCount
        snapshot.output = BridgeImageProcessorOutputSnapshot(output: output)
        if let input {
            snapshot.input = BridgeImageProcessorInputSnapshot(input: input)
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
        ci_json_string(from: lastInvocation.jsonObject) ?? "{}"
    }

    class func lastInvocationSnapshot() -> BridgeImageProcessorInvocationSnapshot {
        lastInvocation
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
        let context = CIContext(options: nil)
        guard context.createCGImage(output, from: output.extent) != nil else {
            throw CIBridgeError.nullResult("CIContext.createCGImage returned nil")
        }
        outImage.pointee = ci_retain(output)
    }
}

@_cdecl("ci_image_processor_last_invocation_json")
public func ci_image_processor_last_invocation_json() -> UnsafeMutablePointer<CChar>? {
    ci_string(BridgePassthroughImageProcessorKernel.lastInvocationJSON())
}

@_cdecl("ci_image_processor_last_invocation_input_count")
public func ci_image_processor_last_invocation_input_count() -> Int {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().inputCount
}

@_cdecl("ci_image_processor_last_invocation_has_input")
public func ci_image_processor_last_invocation_has_input() -> Bool {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input != nil
}

@_cdecl("ci_image_processor_last_input_region")
public func ci_image_processor_last_input_region(
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    let rect = BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.region ?? .zero
    ci_write_rect(rect, outX, outY, outWidth, outHeight)
}

@_cdecl("ci_image_processor_last_output_region")
public func ci_image_processor_last_output_region(
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    ci_write_rect(
        BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.region,
        outX,
        outY,
        outWidth,
        outHeight
    )
}

@_cdecl("ci_image_processor_last_input_bytes_per_row")
public func ci_image_processor_last_input_bytes_per_row() -> Int {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.bytesPerRow ?? 0
}

@_cdecl("ci_image_processor_last_output_bytes_per_row")
public func ci_image_processor_last_output_bytes_per_row() -> Int {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.bytesPerRow
}

@_cdecl("ci_image_processor_last_input_format")
public func ci_image_processor_last_input_format() -> Int32 {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.format ?? 0
}

@_cdecl("ci_image_processor_last_output_format")
public func ci_image_processor_last_output_format() -> Int32 {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.format
}

@_cdecl("ci_image_processor_last_input_has_pixel_buffer")
public func ci_image_processor_last_input_has_pixel_buffer() -> Bool {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.hasPixelBuffer ?? false
}

@_cdecl("ci_image_processor_last_output_has_pixel_buffer")
public func ci_image_processor_last_output_has_pixel_buffer() -> Bool {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.hasPixelBuffer
}

@_cdecl("ci_image_processor_last_input_has_metal_texture")
public func ci_image_processor_last_input_has_metal_texture() -> Bool {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.hasMetalTexture ?? false
}

@_cdecl("ci_image_processor_last_output_has_metal_texture")
public func ci_image_processor_last_output_has_metal_texture() -> Bool {
    BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.hasMetalTexture
}

@_cdecl("ci_image_processor_last_input_digest")
public func ci_image_processor_last_input_digest() -> UnsafeMutablePointer<CChar>? {
    ci_string(BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.digest ?? "")
}

@_cdecl("ci_image_processor_last_output_digest")
public func ci_image_processor_last_output_digest() -> UnsafeMutablePointer<CChar>? {
    ci_string(BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().output.digest ?? "")
}

@_cdecl("ci_image_processor_last_input_roi_tile_index")
public func ci_image_processor_last_input_roi_tile_index() -> Int64 {
    Int64(BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.roiTileIndex ?? -1)
}

@_cdecl("ci_image_processor_last_input_roi_tile_count")
public func ci_image_processor_last_input_roi_tile_count() -> Int64 {
    Int64(BridgePassthroughImageProcessorKernel.lastInvocationSnapshot().input?.roiTileCount ?? -1)
}
