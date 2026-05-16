import CoreImage
import Foundation

private func ci_feature_type_code(_ feature: CIFeature) -> Int32 {
    switch feature.type {
    case CIFeatureTypeFace:
        return 0
    case CIFeatureTypeRectangle:
        return 1
    case CIFeatureTypeQRCode:
        return 2
    case CIFeatureTypeText:
        return 3
    default:
        return -1
    }
}

private func ci_barcode_descriptor_type_code(_ descriptor: CIBarcodeDescriptor) -> Int32 {
    switch descriptor {
    case is CIQRCodeDescriptor:
        return 0
    case is CIAztecCodeDescriptor:
        return 1
    case is CIPDF417CodeDescriptor:
        return 2
    case is CIDataMatrixCodeDescriptor:
        return 3
    default:
        return -1
    }
}

private func ci_feature_details(_ feature: CIFeature) -> [String: Any] {
    var details: [String: Any] = [
        "type": feature.type,
        "bounds": ci_normalize_json(feature.bounds),
    ]

    if let face = feature as? CIFaceFeature {
        details["hasLeftEyePosition"] = face.hasLeftEyePosition
        details["leftEyePosition"] = ci_normalize_json(face.leftEyePosition)
        details["hasRightEyePosition"] = face.hasRightEyePosition
        details["rightEyePosition"] = ci_normalize_json(face.rightEyePosition)
        details["hasMouthPosition"] = face.hasMouthPosition
        details["mouthPosition"] = ci_normalize_json(face.mouthPosition)
        details["hasTrackingID"] = face.hasTrackingID
        details["trackingID"] = face.trackingID
        details["hasTrackingFrameCount"] = face.hasTrackingFrameCount
        details["trackingFrameCount"] = face.trackingFrameCount
        details["hasFaceAngle"] = face.hasFaceAngle
        details["faceAngle"] = face.faceAngle
        details["hasSmile"] = face.hasSmile
        details["leftEyeClosed"] = face.leftEyeClosed
        details["rightEyeClosed"] = face.rightEyeClosed
    }

    if let rectangle = feature as? CIRectangleFeature {
        details["topLeft"] = ci_normalize_json(rectangle.topLeft)
        details["topRight"] = ci_normalize_json(rectangle.topRight)
        details["bottomLeft"] = ci_normalize_json(rectangle.bottomLeft)
        details["bottomRight"] = ci_normalize_json(rectangle.bottomRight)
    }

    if let qr = feature as? CIQRCodeFeature {
        details["topLeft"] = ci_normalize_json(qr.topLeft)
        details["topRight"] = ci_normalize_json(qr.topRight)
        details["bottomLeft"] = ci_normalize_json(qr.bottomLeft)
        details["bottomRight"] = ci_normalize_json(qr.bottomRight)
        details["messageString"] = qr.messageString ?? ""
        if let descriptor = qr.symbolDescriptor {
            details["symbolDescriptor"] = ci_normalize_json(["kind": ci_barcode_descriptor_type_code(descriptor)])
        }
    }

    if let text = feature as? CITextFeature {
        details["topLeft"] = ci_normalize_json(text.topLeft)
        details["topRight"] = ci_normalize_json(text.topRight)
        details["bottomLeft"] = ci_normalize_json(text.bottomLeft)
        details["bottomRight"] = ci_normalize_json(text.bottomRight)
        details["subFeatureCount"] = text.subFeatures?.count ?? 0
    }

    return details
}

@_cdecl("ci_feature_type_code")
public func ci_feature_type_code_c(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let feature: CIFeature = ci_borrow(handle) else { return -1 }
    return ci_feature_type_code(feature)
}

@_cdecl("ci_feature_bounds")
public func ci_feature_bounds(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let feature: CIFeature = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let bounds = feature.bounds
    outX?.pointee = bounds.origin.x
    outY?.pointee = bounds.origin.y
    outWidth?.pointee = bounds.size.width
    outHeight?.pointee = bounds.size.height
}

@_cdecl("ci_feature_details_json")
public func ci_feature_details_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let feature: CIFeature = ci_borrow(handle) else { return ci_string("{}") }
    return ci_string(ci_json_string(from: ci_feature_details(feature)) ?? "{}")
}

@_cdecl("ci_feature_message_string")
public func ci_feature_message_string(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let feature: CIFeature = ci_borrow(handle), let qr = feature as? CIQRCodeFeature else {
        return nil
    }
    return ci_string(qr.messageString ?? "")
}

@_cdecl("ci_feature_symbol_descriptor")
public func ci_feature_symbol_descriptor(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let feature: CIFeature = ci_borrow(handle), let qr = feature as? CIQRCodeFeature, let descriptor = qr.symbolDescriptor else {
        return nil
    }
    return ci_retain(descriptor)
}

@_cdecl("ci_feature_subfeatures")
public func ci_feature_subfeatures(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let feature: CIFeature = ci_borrow(handle), let text = feature as? CITextFeature, let subFeatures = text.subFeatures else {
        return nil
    }
    return ci_retain(subFeatures as NSArray)
}
