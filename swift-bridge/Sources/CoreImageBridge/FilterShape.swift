import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_filter_shape_new")
public func ci_filter_shape_new(
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    ci_retain(CIFilterShape(rect: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_filter_shape_transform")
public func ci_filter_shape_transform(
    _ handle: UnsafeMutableRawPointer?,
    _ a: Double,
    _ b: Double,
    _ c: Double,
    _ d: Double,
    _ tx: Double,
    _ ty: Double,
    _ interior: Bool
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle) else { return nil }
    let transform = CGAffineTransform(a: a, b: b, c: c, d: d, tx: tx, ty: ty)
    return ci_retain(shape.transform(by: transform, interior: interior))
}

@_cdecl("ci_filter_shape_inset")
public func ci_filter_shape_inset(
    _ handle: UnsafeMutableRawPointer?,
    _ dx: Int32,
    _ dy: Int32
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle) else { return nil }
    return ci_retain(shape.insetBy(x: dx, y: dy))
}

@_cdecl("ci_filter_shape_union")
public func ci_filter_shape_union(
    _ handle: UnsafeMutableRawPointer?,
    _ otherHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle),
          let other: CIFilterShape = ci_borrow(otherHandle)
    else {
        return nil
    }
    return ci_retain(shape.union(with: other))
}

@_cdecl("ci_filter_shape_union_rect")
public func ci_filter_shape_union_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle) else { return nil }
    return ci_retain(shape.union(with: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_filter_shape_intersect")
public func ci_filter_shape_intersect(
    _ handle: UnsafeMutableRawPointer?,
    _ otherHandle: UnsafeMutableRawPointer?
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle),
          let other: CIFilterShape = ci_borrow(otherHandle)
    else {
        return nil
    }
    return ci_retain(shape.intersect(with: other))
}

@_cdecl("ci_filter_shape_intersect_rect")
public func ci_filter_shape_intersect_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) -> UnsafeMutableRawPointer? {
    guard let shape: CIFilterShape = ci_borrow(handle) else { return nil }
    return ci_retain(shape.intersect(with: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_filter_shape_extent")
public func ci_filter_shape_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let shape: CIFilterShape = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = shape.extent
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}
