import CoreImage
import Foundation

public typealias CIXFilterConstructorCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer?
public typealias CIXFilterConstructorReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class BridgeFilterConstructor: NSObject, CIFilterConstructor {
    private let context: UnsafeMutableRawPointer?
    private let callback: CIXFilterConstructorCallback
    private let releaseCallback: CIXFilterConstructorReleaseCallback?

    init(
        context: UnsafeMutableRawPointer?,
        callback: @escaping CIXFilterConstructorCallback,
        releaseCallback: CIXFilterConstructorReleaseCallback?
    ) {
        self.context = context
        self.callback = callback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback?(context)
    }

    func filter(withName name: String) -> CIFilter? {
        guard let handle = name.withCString({ callback(context, $0) }) else {
            return nil
        }
        return Unmanaged<CIFilter>.fromOpaque(handle).takeRetainedValue()
    }
}

@_cdecl("ci_filter_constructor_new")
public func ci_filter_constructor_new(
    _ context: UnsafeMutableRawPointer?,
    _ callback: CIXFilterConstructorCallback?,
    _ releaseCallback: CIXFilterConstructorReleaseCallback?
) -> UnsafeMutableRawPointer? {
    guard let callback else { return nil }
    return ci_retain(
        BridgeFilterConstructor(
            context: context,
            callback: callback,
            releaseCallback: releaseCallback
        )
    )
}

@_cdecl("ci_filter_constructor_filter")
public func ci_filter_constructor_filter(
    _ handle: UnsafeMutableRawPointer?,
    _ name: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let constructor: BridgeFilterConstructor = ci_borrow(handle), let name else {
        return nil
    }
    return constructor.filter(withName: String(cString: name)).map(ci_retain)
}

@_cdecl("ci_filter_register_name")
public func ci_filter_register_name(
    _ name: UnsafePointer<CChar>?,
    _ constructorHandle: UnsafeMutableRawPointer?,
    _ displayName: UnsafePointer<CChar>?
) {
    guard let name, let constructor: BridgeFilterConstructor = ci_borrow(constructorHandle) else {
        return
    }

    let nameString = String(cString: name)
    var classAttributes: [String: Any] = [
        kCIAttributeFilterDisplayName: displayName.map { String(cString: $0) } ?? nameString,
        kCIAttributeFilterCategories: [kCICategoryStillImage],
    ]
    classAttributes[kCIAttributeFilterName] = nameString
    CIFilter.registerName(nameString, constructor: constructor, classAttributes: classAttributes)
}
