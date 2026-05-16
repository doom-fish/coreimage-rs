import CoreImage
import Foundation

@_cdecl("ci_filter_generator_new")
public func ci_filter_generator_new() -> UnsafeMutableRawPointer? {
    ci_retain(CIFilterGenerator())
}

@_cdecl("ci_filter_generator_from_path")
public func ci_filter_generator_from_path(
    _ path: UnsafePointer<CChar>?,
    _ outGenerator: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let path, let outGenerator else {
            throw CIBridgeError.invalidArgument("missing path or output pointer")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        guard let generator = CIFilterGenerator(contentsOf: url) else {
            throw CIBridgeError.nullResult("CIFilterGenerator(contentsOf:) returned nil")
        }
        outGenerator.pointee = ci_retain(generator)
    }
}

@_cdecl("ci_filter_generator_connect_filter_output")
public func ci_filter_generator_connect_filter_output(
    _ handle: UnsafeMutableRawPointer?,
    _ sourceFilterHandle: UnsafeMutableRawPointer?,
    _ sourceKey: UnsafePointer<CChar>?,
    _ targetFilterHandle: UnsafeMutableRawPointer?,
    _ targetKey: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle),
          let sourceFilter: CIFilter = ci_borrow(sourceFilterHandle),
          let targetFilter: CIFilter = ci_borrow(targetFilterHandle),
          let targetKey
    else {
        return
    }
    let source = sourceKey.map { String(cString: $0) }
    generator.connect(sourceFilter, withKey: source, to: targetFilter, withKey: String(cString: targetKey))
}

@_cdecl("ci_filter_generator_disconnect_filter_output")
public func ci_filter_generator_disconnect_filter_output(
    _ handle: UnsafeMutableRawPointer?,
    _ sourceFilterHandle: UnsafeMutableRawPointer?,
    _ sourceKey: UnsafePointer<CChar>?,
    _ targetFilterHandle: UnsafeMutableRawPointer?,
    _ targetKey: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle),
          let sourceFilter: CIFilter = ci_borrow(sourceFilterHandle),
          let targetFilter: CIFilter = ci_borrow(targetFilterHandle),
          let sourceKey,
          let targetKey
    else {
        return
    }
    generator.disconnectObject(
        sourceFilter,
        withKey: String(cString: sourceKey),
        to: targetFilter,
        withKey: String(cString: targetKey)
    )
}

@_cdecl("ci_filter_generator_connect_image")
public func ci_filter_generator_connect_image(
    _ handle: UnsafeMutableRawPointer?,
    _ imageHandle: UnsafeMutableRawPointer?,
    _ targetFilterHandle: UnsafeMutableRawPointer?,
    _ targetKey: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle),
          let image: CIImage = ci_borrow(imageHandle),
          let targetFilter: CIFilter = ci_borrow(targetFilterHandle),
          let targetKey
    else {
        return
    }
    generator.connect(image, withKey: nil, to: targetFilter, withKey: String(cString: targetKey))
}

@_cdecl("ci_filter_generator_export_key")
public func ci_filter_generator_export_key(
    _ handle: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ targetFilterHandle: UnsafeMutableRawPointer?,
    _ exportedName: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle),
          let key,
          let targetFilter: CIFilter = ci_borrow(targetFilterHandle)
    else {
        return
    }
    generator.exportKey(
        String(cString: key),
        from: targetFilter,
        withName: exportedName.map { String(cString: $0) }
    )
}

@_cdecl("ci_filter_generator_remove_exported_key")
public func ci_filter_generator_remove_exported_key(
    _ handle: UnsafeMutableRawPointer?,
    _ exportedName: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle), let exportedName else { return }
    generator.removeExportedKey(String(cString: exportedName))
}

@_cdecl("ci_filter_generator_exported_keys_json")
public func ci_filter_generator_exported_keys_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let generator: CIFilterGenerator = ci_borrow(handle) else { return ci_string("{}") }
    return ci_string(ci_json_string(from: generator.exportedKeys) ?? "{}")
}

@_cdecl("ci_filter_generator_filter")
public func ci_filter_generator_filter(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let generator: CIFilterGenerator = ci_borrow(handle) else { return nil }
    return ci_retain(generator.filter())
}

@_cdecl("ci_filter_generator_register_filter_name")
public func ci_filter_generator_register_filter_name(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?,
    _ displayName: UnsafePointer<CChar>?
) {
    guard let generator: CIFilterGenerator = ci_borrow(handle), let name else { return }
    let nameString = String(cString: name)
    generator.classAttributes = [
        kCIAttributeFilterDisplayName: displayName.map { String(cString: $0) } ?? nameString,
        kCIAttributeFilterCategories: [kCICategoryGenerator, kCICategoryStillImage, kCICategoryBuiltIn],
    ]
    generator.registerFilterName(nameString)
}

@_cdecl("ci_filter_generator_write_to_path")
public func ci_filter_generator_write_to_path(
    _ handle: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ atomically: Bool,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let generator: CIFilterGenerator = ci_borrow(handle), let path else {
            throw CIBridgeError.invalidArgument("missing generator or path")
        }
        let url = URL(fileURLWithPath: String(cString: path))
        if !generator.write(to: url, atomically: atomically) {
            throw CIBridgeError.io("CIFilterGenerator failed to write to \(url.path)")
        }
    }
}
