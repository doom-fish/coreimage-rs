import CoreImage
import Foundation

@_cdecl("ci_filter_new")
public func ci_filter_new(_ name: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let name else { return nil }
    return CIFilter(name: String(cString: name)).map(ci_retain)
}

@_cdecl("ci_filter_name")
public func ci_filter_name(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let filter: CIFilter = ci_borrow(handle) else { return nil }
    return ci_string(filter.name)
}

@_cdecl("ci_filter_set_name")
public func ci_filter_set_name(_ handle: UnsafeMutableRawPointer?, _ name: UnsafePointer<CChar>?) {
    guard let filter: CIFilter = ci_borrow(handle), let name else { return }
    filter.name = String(cString: name)
}

@_cdecl("ci_filter_is_enabled")
public func ci_filter_is_enabled(_ handle: UnsafeMutableRawPointer?) -> Bool {
    guard let filter: CIFilter = ci_borrow(handle) else { return false }
    return filter.isEnabled
}

@_cdecl("ci_filter_set_enabled")
public func ci_filter_set_enabled(_ handle: UnsafeMutableRawPointer?, _ enabled: Bool) {
    guard let filter: CIFilter = ci_borrow(handle) else { return }
    filter.isEnabled = enabled
}

@_cdecl("ci_filter_set_defaults")
public func ci_filter_set_defaults(_ handle: UnsafeMutableRawPointer?) {
    guard let filter: CIFilter = ci_borrow(handle) else { return }
    filter.setDefaults()
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

@_cdecl("ci_filter_localized_name")
public func ci_filter_localized_name(_ name: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let name else { return nil }
    return ci_string(CIFilter.localizedName(forFilterName: String(cString: name)) ?? "")
}

@_cdecl("ci_filter_localized_description")
public func ci_filter_localized_description(_ name: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let name else { return nil }
    return ci_string(CIFilter.localizedDescription(forFilterName: String(cString: name)) ?? "")
}

@_cdecl("ci_filter_localized_reference_url")
public func ci_filter_localized_reference_url(_ name: UnsafePointer<CChar>?) -> UnsafeMutablePointer<CChar>? {
    guard let name else { return nil }
    return ci_string(CIFilter.localizedReferenceDocumentation(forFilterName: String(cString: name))?.absoluteString ?? "")
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

@_cdecl("ci_filter_set_bytes")
public func ci_filter_set_bytes(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int
) {
    guard let filter: CIFilter = ci_borrow(handle), let key, let bytes, len >= 0 else { return }
    filter.setValue(Data(bytes: bytes, count: len), forKey: String(cString: key))
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

@_cdecl("ci_filter_set_barcode_descriptor")
public func ci_filter_set_barcode_descriptor(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ descriptorHandle: UnsafeMutableRawPointer?
) {
    guard let filter: CIFilter = ci_borrow(handle),
          let key,
          let descriptor: CIBarcodeDescriptor = ci_borrow(descriptorHandle)
    else {
        return
    }
    filter.setValue(descriptor, forKey: String(cString: key))
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
