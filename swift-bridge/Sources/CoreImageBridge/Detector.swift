import CoreImage
import Foundation

private func ci_detector_type(_ kind: Int32) -> String? {
    switch kind {
    case 0:
        return CIDetectorTypeFace
    case 1:
        return CIDetectorTypeRectangle
    case 2:
        return CIDetectorTypeQRCode
    case 3:
        return CIDetectorTypeText
    default:
        return nil
    }
}

private func ci_detector_options(
    accuracy: Int32,
    tracking: Bool,
    minFeatureSize: Double,
    maxFeatureCount: Int32,
    numberOfAngles: Int32
) -> [String: Any] {
    var options: [String: Any] = [:]
    switch accuracy {
    case 1:
        options[CIDetectorAccuracy] = CIDetectorAccuracyLow
    case 2:
        options[CIDetectorAccuracy] = CIDetectorAccuracyHigh
    default:
        break
    }
    if tracking {
        options[CIDetectorTracking] = true
    }
    if minFeatureSize > 0 {
        options[CIDetectorMinFeatureSize] = minFeatureSize
    }
    if maxFeatureCount > 0 {
        options[CIDetectorMaxFeatureCount] = maxFeatureCount
    }
    if numberOfAngles > 0 {
        options[CIDetectorNumberOfAngles] = numberOfAngles
    }
    return options
}

private func ci_feature_detection_options(
    orientation: Int32,
    eyeBlink: Bool,
    smile: Bool,
    focalLength: Double,
    aspectRatio: Double,
    returnSubFeatures: Bool
) -> [String: Any] {
    var options: [String: Any] = [:]
    if orientation > 0 {
        options[CIDetectorImageOrientation] = orientation
    }
    if eyeBlink {
        options[CIDetectorEyeBlink] = true
    }
    if smile {
        options[CIDetectorSmile] = true
    }
    if focalLength >= 0 {
        options[CIDetectorFocalLength] = focalLength
    }
    if aspectRatio > 0 {
        options[CIDetectorAspectRatio] = aspectRatio
    }
    if returnSubFeatures {
        options[CIDetectorReturnSubFeatures] = true
    }
    return options
}

@_cdecl("ci_detector_new")
public func ci_detector_new(
    _ kind: Int32,
    _ contextHandle: UnsafeMutableRawPointer?,
    _ accuracy: Int32,
    _ tracking: Bool,
    _ minFeatureSize: Double,
    _ maxFeatureCount: Int32,
    _ numberOfAngles: Int32,
    _ outDetector: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let type = ci_detector_type(kind), let outDetector else {
            throw CIBridgeError.invalidArgument("missing detector type or output pointer")
        }
        let context: CIContext? = ci_borrow(contextHandle)
        let options = ci_detector_options(
            accuracy: accuracy,
            tracking: tracking,
            minFeatureSize: minFeatureSize,
            maxFeatureCount: maxFeatureCount,
            numberOfAngles: numberOfAngles
        )
        guard let detector = CIDetector(ofType: type, context: context, options: options.isEmpty ? nil : options) else {
            throw CIBridgeError.nullResult("CIDetector(ofType:context:options:) returned nil")
        }
        outDetector.pointee = ci_retain(detector)
    }
}

@_cdecl("ci_detector_features")
public func ci_detector_features(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ orientation: Int32,
    _ eyeBlink: Bool,
    _ smile: Bool,
    _ focalLength: Double,
    _ aspectRatio: Double,
    _ returnSubFeatures: Bool,
    _ outFeatures: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let detector: CIDetector = ci_borrow(handle),
              let image: CIImage = ci_borrow(imageHandle),
              let outFeatures
        else {
            throw CIBridgeError.invalidArgument("missing detector, image, or output pointer")
        }
        let options = ci_feature_detection_options(
            orientation: orientation,
            eyeBlink: eyeBlink,
            smile: smile,
            focalLength: focalLength,
            aspectRatio: aspectRatio,
            returnSubFeatures: returnSubFeatures
        )
        let features = detector.features(in: image, options: options.isEmpty ? nil : options)
        outFeatures.pointee = ci_retain(features as NSArray)
    }
}
