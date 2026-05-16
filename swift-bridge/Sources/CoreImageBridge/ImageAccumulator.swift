import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_image_accumulator_new")
public func ci_image_accumulator_new(
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double,
    _ formatCode: Int32,
    _ useColorSpace: Bool,
    _ colorSpaceCode: Int32
) -> UnsafeMutableRawPointer? {
    let extent = CGRect(x: x, y: y, width: width, height: height)
    guard let format = ci_image_format(from: formatCode) else { return nil }
    let accumulator: CIImageAccumulator?
    if useColorSpace {
        guard let colorSpace = ci_color_space(from: colorSpaceCode) else { return nil }
        accumulator = CIImageAccumulator(extent: extent, format: format, colorSpace: colorSpace)
    } else {
        accumulator = CIImageAccumulator(extent: extent, format: format)
    }
    guard let accumulator else { return nil }
    return ci_retain(accumulator)
}

@_cdecl("ci_image_accumulator_extent")
public func ci_image_accumulator_extent(
    _ handle: UnsafeMutableRawPointer?,
    _ outX: UnsafeMutablePointer<Double>?,
    _ outY: UnsafeMutablePointer<Double>?,
    _ outWidth: UnsafeMutablePointer<Double>?,
    _ outHeight: UnsafeMutablePointer<Double>?
) {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle) else {
        outX?.pointee = 0
        outY?.pointee = 0
        outWidth?.pointee = 0
        outHeight?.pointee = 0
        return
    }
    let rect = accumulator.extent
    outX?.pointee = rect.origin.x
    outY?.pointee = rect.origin.y
    outWidth?.pointee = rect.size.width
    outHeight?.pointee = rect.size.height
}

@_cdecl("ci_image_accumulator_format")
public func ci_image_accumulator_format(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle) else { return 0 }
    return accumulator.format.rawValue
}

@_cdecl("ci_image_accumulator_image")
public func ci_image_accumulator_image(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle) else { return nil }
    return ci_retain(accumulator.image())
}

@_cdecl("ci_image_accumulator_set_image")
public func ci_image_accumulator_set_image(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?
) {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return
    }
    accumulator.setImage(image)
}

@_cdecl("ci_image_accumulator_set_image_dirty_rect")
public func ci_image_accumulator_set_image_dirty_rect(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ x: Double,
    _ y: Double,
    _ width: Double,
    _ height: Double
) {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle)
    else {
        return
    }
    accumulator.setImage(image, dirtyRect: CGRect(x: x, y: y, width: width, height: height))
}

@_cdecl("ci_image_accumulator_clear")
public func ci_image_accumulator_clear(_ handle: UnsafeMutableRawPointer?) {
    guard let accumulator: CIImageAccumulator = ci_borrow(handle) else { return }
    accumulator.clear()
}
