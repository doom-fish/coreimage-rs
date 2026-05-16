import CoreImage
import Foundation

extension CIFilter {
    static func gaussianBlur() -> CIFilter? { CIFilter(name: "CIGaussianBlur") }
    static func boxBlur() -> CIFilter? { CIFilter(name: "CIBoxBlur") }
    static func discBlur() -> CIFilter? { CIFilter(name: "CIDiscBlur") }
    static func motionBlur() -> CIFilter? { CIFilter(name: "CIMotionBlur") }
    static func zoomBlur() -> CIFilter? { CIFilter(name: "CIZoomBlur") }
    static func sharpenLuminance() -> CIFilter? { CIFilter(name: "CISharpenLuminance") }
    static func unsharpMask() -> CIFilter? { CIFilter(name: "CIUnsharpMask") }
    static func colorControls() -> CIFilter? { CIFilter(name: "CIColorControls") }
    static func exposureAdjust() -> CIFilter? { CIFilter(name: "CIExposureAdjust") }
    static func gammaAdjust() -> CIFilter? { CIFilter(name: "CIGammaAdjust") }
    static func hueAdjust() -> CIFilter? { CIFilter(name: "CIHueAdjust") }
    static func vibrance() -> CIFilter? { CIFilter(name: "CIVibrance") }
    static func temperatureAndTint() -> CIFilter? { CIFilter(name: "CITemperatureAndTint") }
    static func whitePointAdjust() -> CIFilter? { CIFilter(name: "CIWhitePointAdjust") }
    static func sepiaTone() -> CIFilter? { CIFilter(name: "CISepiaTone") }
    static func colorInvert() -> CIFilter? { CIFilter(name: "CIColorInvert") }
    static func colorMonochrome() -> CIFilter? { CIFilter(name: "CIColorMonochrome") }
    static func falseColor() -> CIFilter? { CIFilter(name: "CIFalseColor") }
    static func vignette() -> CIFilter? { CIFilter(name: "CIVignette") }
    static func vignetteEffect() -> CIFilter? { CIFilter(name: "CIVignetteEffect") }
    static func edges() -> CIFilter? { CIFilter(name: "CIEdges") }
    static func edgeWork() -> CIFilter? { CIFilter(name: "CIEdgeWork") }
    static func bloom() -> CIFilter? { CIFilter(name: "CIBloom") }
    static func pixellate() -> CIFilter? { CIFilter(name: "CIPixellate") }
    static func comicEffect() -> CIFilter? { CIFilter(name: "CIComicEffect") }
    static func crystallize() -> CIFilter? { CIFilter(name: "CICrystallize") }
    static func straighten() -> CIFilter? { CIFilter(name: "CIStraightenFilter") }
    static func lanczosScaleTransform() -> CIFilter? { CIFilter(name: "CILanczosScaleTransform") }
    static func perspectiveCorrection() -> CIFilter? { CIFilter(name: "CIPerspectiveCorrection") }
    static func perspectiveTransform() -> CIFilter? { CIFilter(name: "CIPerspectiveTransform") }
    static func sourceOverCompositing() -> CIFilter? { CIFilter(name: "CISourceOverCompositing") }
    static func multiplyCompositing() -> CIFilter? { CIFilter(name: "CIMultiplyCompositing") }
    static func blendWithMask() -> CIFilter? { CIFilter(name: "CIBlendWithMask") }
    static func constantColor() -> CIFilter? { CIFilter(name: "CIConstantColorGenerator") }
    static func checkerboard() -> CIFilter? { CIFilter(name: "CICheckerboardGenerator") }
    static func linearGradient() -> CIFilter? { CIFilter(name: "CILinearGradient") }
    static func radialGradient() -> CIFilter? { CIFilter(name: "CIRadialGradient") }
    static func qrCode() -> CIFilter? { CIFilter(name: "CIQRCodeGenerator") }
    static func crop() -> CIFilter? { CIFilter(name: "CICrop") }
}

@_cdecl("ci_filter_new")
public func ci_filter_new(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return CIFilter(name: String(cString: name)).map(ci_retain)
}

@_cdecl("ci_filter_names_lines")
public func ci_filter_names_lines(_ category: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    let names: [String]
    if let category {
        names = CIFilter.filterNames(inCategories: [String(cString: category)])
    } else {
        names = CIFilter.filterNames(inCategories: nil)
    }
    return ci_string(names.joined(separator: "\n"))
}

@_cdecl("ci_filter_input_keys_lines")
public func ci_filter_input_keys_lines(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let filter: CIFilter = ci_borrow(handle) else { return ci_string("") }
    return ci_string(filter.inputKeys.joined(separator: "\n"))
}

@_cdecl("ci_filter_output_keys_lines")
public func ci_filter_output_keys_lines(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let filter: CIFilter = ci_borrow(handle) else { return ci_string("") }
    return ci_string(filter.outputKeys.joined(separator: "\n"))
}

@_cdecl("ci_filter_attributes_json")
public func ci_filter_attributes_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let filter: CIFilter = ci_borrow(handle) else { return ci_string("{}") }
    return ci_string(ci_json_string(from: filter.attributes) ?? "{}")
}

@_cdecl("ci_filter_set_image")
public func ci_filter_set_image(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ imageHandle: UnsafeMutableRawPointer?
) {
    guard let filter: CIFilter = ci_borrow(handle),
          let key,
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return
    }
    filter.setValue(image, forKey: String(cString: key))
}

@_cdecl("ci_filter_set_number")
public func ci_filter_set_number(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Double
) {
    guard let filter: CIFilter = ci_borrow(handle), let key else { return }
    filter.setValue(value, forKey: String(cString: key))
}

@_cdecl("ci_filter_set_string")
public func ci_filter_set_string(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?
) {
    guard let filter: CIFilter = ci_borrow(handle), let key, let value else { return }
    filter.setValue(String(cString: value), forKey: String(cString: key))
}

@_cdecl("ci_filter_set_vector")
public func ci_filter_set_vector(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ vectorHandle: UnsafeMutableRawPointer?
) {
    guard let filter: CIFilter = ci_borrow(handle),
          let key,
          let vector: CIVector = ci_borrow(vectorHandle)
    else {
        return
    }
    filter.setValue(vector, forKey: String(cString: key))
}

@_cdecl("ci_filter_set_color")
public func ci_filter_set_color(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ colorHandle: UnsafeMutableRawPointer?
) {
    guard let filter: CIFilter = ci_borrow(handle),
          let key,
          let color: CIColor = ci_borrow(colorHandle)
    else {
        return
    }
    filter.setValue(color, forKey: String(cString: key))
}

@_cdecl("ci_filter_output_image")
public func ci_filter_output_image(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let filter: CIFilter = ci_borrow(handle),
          let image = filter.outputImage
    else {
        return nil
    }
    return ci_retain(image)
}

@_cdecl("ci_vector_new2")
public func ci_vector_new2(_ x: Double, _ y: Double) -> UnsafeMutableRawPointer? {
    ci_retain(CIVector(x: x, y: y))
}

@_cdecl("ci_vector_new3")
public func ci_vector_new3(_ x: Double, _ y: Double, _ z: Double) -> UnsafeMutableRawPointer? {
    ci_retain(CIVector(x: x, y: y, z: z))
}

@_cdecl("ci_vector_new4")
public func ci_vector_new4(
    _ x: Double,
    _ y: Double,
    _ z: Double,
    _ w: Double
) -> UnsafeMutableRawPointer? {
    ci_retain(CIVector(x: x, y: y, z: z, w: w))
}

@_cdecl("ci_color_new_rgba")
public func ci_color_new_rgba(
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) -> UnsafeMutableRawPointer? {
    ci_retain(CIColor(red: red, green: green, blue: blue, alpha: alpha))
}
