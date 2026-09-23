import CoreImage
import CoreImageObjCBridge
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

private final class BridgeImageProcessorInvocationSnapshotBox {
    let value: BridgeImageProcessorInvocationSnapshot

    init(_ value: BridgeImageProcessorInvocationSnapshot) {
        self.value = value
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
    private static let invocationLock = NSLock()
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
        invocationLock.lock()
        defer { invocationLock.unlock() }
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

    class func lastInvocationSnapshot() -> BridgeImageProcessorInvocationSnapshot {
        invocationLock.lock()
        defer { invocationLock.unlock() }
        return lastInvocation
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

@_cdecl("ci_image_processor_invocation_snapshot_new")
public func ci_image_processor_invocation_snapshot_new() -> UnsafeMutableRawPointer? {
    ci_retain(
        BridgeImageProcessorInvocationSnapshotBox(
            BridgePassthroughImageProcessorKernel.lastInvocationSnapshot()
        )
    )
}

@_cdecl("ci_image_processor_invocation_snapshot_json")
public func ci_image_processor_invocation_snapshot_json(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return nil
    }
    return ci_string(ci_json_string(from: snapshot.value.jsonObject) ?? "{}")
}

@_cdecl("ci_image_processor_invocation_snapshot_input_count")
public func ci_image_processor_invocation_snapshot_input_count(
    _ handle: UnsafeMutableRawPointer?
) -> Int {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return 0
    }
    return snapshot.value.inputCount
}

@_cdecl("ci_image_processor_invocation_snapshot_has_input")
public func ci_image_processor_invocation_snapshot_has_input(
    _ handle: UnsafeMutableRawPointer?
) -> Bool {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return false
    }
    return snapshot.value.input != nil
}

@_cdecl("ci_image_processor_invocation_snapshot_input_region")
public func ci_image_processor_invocation_snapshot_input_region(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        ci_write_rect(.zero, outX, outY, outWidth, outHeight)
        return
    }
    let rect = snapshot.value.input?.region ?? .zero
    ci_write_rect(rect, outX, outY, outWidth, outHeight)
}

@_cdecl("ci_image_processor_invocation_snapshot_output_region")
public func ci_image_processor_invocation_snapshot_output_region(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        ci_write_rect(.zero, outX, outY, outWidth, outHeight)
        return
    }
    ci_write_rect(snapshot.value.output.region, outX, outY, outWidth, outHeight)
}

@_cdecl("ci_image_processor_invocation_snapshot_input_bytes_per_row")
public func ci_image_processor_invocation_snapshot_input_bytes_per_row(
    _ handle: UnsafeMutableRawPointer?
) -> Int {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return 0
    }
    return snapshot.value.input?.bytesPerRow ?? 0
}

@_cdecl("ci_image_processor_invocation_snapshot_output_bytes_per_row")
public func ci_image_processor_invocation_snapshot_output_bytes_per_row(
    _ handle: UnsafeMutableRawPointer?
) -> Int {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return 0
    }
    return snapshot.value.output.bytesPerRow
}

@_cdecl("ci_image_processor_invocation_snapshot_input_format")
public func ci_image_processor_invocation_snapshot_input_format(
    _ handle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return 0
    }
    return snapshot.value.input?.format ?? 0
}

@_cdecl("ci_image_processor_invocation_snapshot_output_format")
public func ci_image_processor_invocation_snapshot_output_format(
    _ handle: UnsafeMutableRawPointer?
) -> Int32 {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return 0
    }
    return snapshot.value.output.format
}

@_cdecl("ci_image_processor_invocation_snapshot_input_has_pixel_buffer")
public func ci_image_processor_invocation_snapshot_input_has_pixel_buffer(
    _ handle: UnsafeMutableRawPointer?
) -> Bool {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return false
    }
    return snapshot.value.input?.hasPixelBuffer ?? false
}

@_cdecl("ci_image_processor_invocation_snapshot_output_has_pixel_buffer")
public func ci_image_processor_invocation_snapshot_output_has_pixel_buffer(
    _ handle: UnsafeMutableRawPointer?
) -> Bool {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return false
    }
    return snapshot.value.output.hasPixelBuffer
}

@_cdecl("ci_image_processor_invocation_snapshot_input_has_metal_texture")
public func ci_image_processor_invocation_snapshot_input_has_metal_texture(
    _ handle: UnsafeMutableRawPointer?
) -> Bool {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return false
    }
    return snapshot.value.input?.hasMetalTexture ?? false
}

@_cdecl("ci_image_processor_invocation_snapshot_output_has_metal_texture")
public func ci_image_processor_invocation_snapshot_output_has_metal_texture(
    _ handle: UnsafeMutableRawPointer?
) -> Bool {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return false
    }
    return snapshot.value.output.hasMetalTexture
}

@_cdecl("ci_image_processor_invocation_snapshot_input_digest")
public func ci_image_processor_invocation_snapshot_input_digest(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return nil
    }
    return ci_string(snapshot.value.input?.digest ?? "")
}

@_cdecl("ci_image_processor_invocation_snapshot_output_digest")
public func ci_image_processor_invocation_snapshot_output_digest(
    _ handle: UnsafeMutableRawPointer?
) -> UnsafeMutablePointer<CChar>? {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return nil
    }
    return ci_string(snapshot.value.output.digest ?? "")
}

@_cdecl("ci_image_processor_invocation_snapshot_input_roi_tile_index")
public func ci_image_processor_invocation_snapshot_input_roi_tile_index(
    _ handle: UnsafeMutableRawPointer?
) -> Int64 {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return -1
    }
    return Int64(snapshot.value.input?.roiTileIndex ?? -1)
}

@_cdecl("ci_image_processor_invocation_snapshot_input_roi_tile_count")
public func ci_image_processor_invocation_snapshot_input_roi_tile_count(
    _ handle: UnsafeMutableRawPointer?
) -> Int64 {
    guard let snapshot: BridgeImageProcessorInvocationSnapshotBox = ci_borrow(handle) else {
        return -1
    }
    return Int64(snapshot.value.input?.roiTileCount ?? -1)
}

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
