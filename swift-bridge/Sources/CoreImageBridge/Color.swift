import CoreGraphics
import CoreImage
import Foundation

@_cdecl("ci_color_new_rgba")
public func ci_color_new_rgba(
    _ red: Double,
    _ green: Double,
    _ blue: Double,
    _ alpha: Double
) -> UnsafeMutableRawPointer? {
    ci_retain(CIColor(red: red, green: green, blue: blue, alpha: alpha))
}

@_cdecl("ci_color_from_string")
public func ci_color_from_string(_ representation: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let representation else { return nil }
    return ci_retain(CIColor(string: String(cString: representation)))
}

@_cdecl("ci_color_named")
public func ci_color_named(_ kind: Int32) -> UnsafeMutableRawPointer? {
    let color: CIColor
    switch kind {
    case 0:
        color = CIColor(red: 0, green: 0, blue: 0, alpha: 1)
    case 1:
        color = CIColor(red: 1, green: 1, blue: 1, alpha: 1)
    case 2:
        color = CIColor(red: 0.5, green: 0.5, blue: 0.5, alpha: 1)
    case 3:
        color = CIColor(red: 1, green: 0, blue: 0, alpha: 1)
    case 4:
        color = CIColor(red: 0, green: 1, blue: 0, alpha: 1)
    case 5:
        color = CIColor(red: 0, green: 0, blue: 1, alpha: 1)
    case 6:
        color = CIColor(red: 0, green: 1, blue: 1, alpha: 1)
    case 7:
        color = CIColor(red: 1, green: 0, blue: 1, alpha: 1)
    case 8:
        color = CIColor(red: 1, green: 1, blue: 0, alpha: 1)
    case 9:
        color = CIColor(red: 0, green: 0, blue: 0, alpha: 0)
    default:
        return nil
    }
    return ci_retain(color)
}

@_cdecl("ci_color_number_of_components")
public func ci_color_number_of_components(_ handle: UnsafeMutableRawPointer?) -> Int {
    guard let color: CIColor = ci_borrow(handle) else { return 0 }
    return Int(color.numberOfComponents)
}

@_cdecl("ci_color_component_at")
public func ci_color_component_at(_ handle: UnsafeMutableRawPointer?, _ index: Int) -> Double {
    guard let color: CIColor = ci_borrow(handle), index >= 0, index < Int(color.numberOfComponents) else {
        return 0
    }
    return Double(color.components[index])
}

@_cdecl("ci_color_alpha")
public func ci_color_alpha(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let color: CIColor = ci_borrow(handle) else { return 0 }
    return Double(color.alpha)
}

@_cdecl("ci_color_red")
public func ci_color_red(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let color: CIColor = ci_borrow(handle) else { return 0 }
    return Double(color.red)
}

@_cdecl("ci_color_green")
public func ci_color_green(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let color: CIColor = ci_borrow(handle) else { return 0 }
    return Double(color.green)
}

@_cdecl("ci_color_blue")
public func ci_color_blue(_ handle: UnsafeMutableRawPointer?) -> Double {
    guard let color: CIColor = ci_borrow(handle) else { return 0 }
    return Double(color.blue)
}

@_cdecl("ci_color_string_representation")
public func ci_color_string_representation(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let color: CIColor = ci_borrow(handle) else { return nil }
    return ci_string(color.stringRepresentation)
}
