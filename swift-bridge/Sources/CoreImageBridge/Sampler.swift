import CoreGraphics
import CoreImage
import Foundation

private func ci_sampler_options(
    wrapMode: Int32,
    filterMode: Int32,
    useTransform: Bool,
    a: Double,
    b: Double,
    c: Double,
    d: Double,
    tx: Double,
    ty: Double
) -> [String: Any] {
    var options: [String: Any] = [:]
    options[kCISamplerWrapMode] = wrapMode == 1 ? kCISamplerWrapClamp : kCISamplerWrapBlack
    options[kCISamplerFilterMode] = filterMode == 1 ? kCISamplerFilterNearest : kCISamplerFilterLinear
    if useTransform {
        options[kCISamplerAffineMatrix] = CIVector(cgAffineTransform: CGAffineTransform(a: a, b: b, c: c, d: d, tx: tx, ty: ty))
    }
    return options
}

@_cdecl("ci_sampler_new")
public func ci_sampler_new(
    _ imageHandle: UnsafeMutableRawPointer?,
    _ wrapMode: Int32,
    _ filterMode: Int32,
    _ useTransform: Bool,
    _ a: Double,
    _ b: Double,
    _ c: Double,
    _ d: Double,
    _ tx: Double,
    _ ty: Double
) -> UnsafeMutableRawPointer? {
    guard let image: CIImage = ci_borrow(imageHandle) else { return nil }
    let options = ci_sampler_options(
        wrapMode: wrapMode,
        filterMode: filterMode,
        useTransform: useTransform,
        a: a,
        b: b,
        c: c,
        d: d,
        tx: tx,
        ty: ty
    )
    return ci_retain(CISampler(image: image, options: options))
}

@_cdecl("ci_sampler_extent")
public func ci_sampler_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let sampler: CISampler = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = sampler.extent
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}

@_cdecl("ci_sampler_definition_extent")
public func ci_sampler_definition_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let sampler: CISampler = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = sampler.definition.extent
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}
