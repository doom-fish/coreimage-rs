import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_vector_new1")
public func ci_vector_new1(_ x: Double) -> UnsafeMutableRawPointer? {
    ci_retain(CIVector(x: x))
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

@_cdecl("ci_vector_from_rect")
public func ci_vector_from_rect(_ x: Double, _ y: Double, _ width: Double, _ height: Double) -> UnsafeMutableRawPointer? {
    ci_retain(CIVector(cgRect: CGRect(x: x, y: y, width: width, height: height)))
}

@_cdecl("ci_vector_from_transform")
public func ci_vector_from_transform(
    _ a: Double,
    _ b: Double,
    _ c: Double,
    _ d: Double,
    _ tx: Double,
    _ ty: Double
) -> UnsafeMutableRawPointer? {
    let transform = CGAffineTransform(a: a, b: b, c: c, d: d, tx: tx, ty: ty)
    return ci_retain(CIVector(cgAffineTransform: transform))
}

@_cdecl("ci_vector_from_string")
public func ci_vector_from_string(_ representation: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let representation else { return nil }
    return ci_retain(CIVector(string: String(cString: representation)))
}

@_cdecl("ci_vector_count")
public func ci_vector_count(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let vector: CIVector = ci_borrow(handle) else { return 0 }
    return Int(vector.count)
}

@_cdecl("ci_vector_value_at")
public func ci_vector_value_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let vector: CIVector = ci_borrow(handle), index >= 0 else { return 0 }
    return Double(vector.value(at: index))
}

@_cdecl("ci_vector_x")
public func ci_vector_x(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let vector: CIVector = ci_borrow(handle) else { return 0 }
    return Double(vector.x)
}

@_cdecl("ci_vector_y")
public func ci_vector_y(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let vector: CIVector = ci_borrow(handle) else { return 0 }
    return Double(vector.y)
}

@_cdecl("ci_vector_z")
public func ci_vector_z(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let vector: CIVector = ci_borrow(handle) else { return 0 }
    return Double(vector.z)
}

@_cdecl("ci_vector_w")
public func ci_vector_w(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let vector: CIVector = ci_borrow(handle) else { return 0 }
    return Double(vector.w)
}

@_cdecl("ci_vector_point")
public func ci_vector_point(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?
) {
    guard let vector: CIVector = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        return
    }
    let point = vector.cgPointValue
    outX?.pointee = point.x
    outY?.pointee = point.y
}

@_cdecl("ci_vector_rect")
public func ci_vector_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let vector: CIVector = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = vector.cgRectValue
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}

@_cdecl("ci_vector_transform")
public func ci_vector_transform(
    _ handle: UnsafeMutableRawPointer?,
    _ outA: UnsafeMutablePointer<Double>?,
    _ outB: UnsafeMutablePointer<Double>?,
    _ outC: UnsafeMutablePointer<Double>?,
    _ outD: UnsafeMutablePointer<Double>?,
    _ outTx: UnsafeMutablePointer<Double>?,
    _ outTy: UnsafeMutablePointer<Double>?
) {
    guard let vector: CIVector = ci_borrow(handle) else {
        outA?.pointee = 0
        outB?.pointee = 0
        outC?.pointee = 0
        outD?.pointee = 0
        outTx?.pointee = 0
        outTy?.pointee = 0
        return
    }
    let transform = vector.cgAffineTransformValue
    outA?.pointee = transform.a
    outB?.pointee = transform.b
    outC?.pointee = transform.c
    outD?.pointee = transform.d
    outTx?.pointee = transform.tx
    outTy?.pointee = transform.ty
}

@_cdecl("ci_vector_string_representation")
public func ci_vector_string_representation(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let vector: CIVector = ci_borrow(handle) else { return nil }
    return ci_string(vector.stringRepresentation)
}
