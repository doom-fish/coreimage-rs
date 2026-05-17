import CoreImage
import Foundation

public typealias CIXPlugInRegistrationCallback = @convention(c) (
    UnsafeMutableRawPointer?,
    UnsafeMutableRawPointer?
) -> Bool
public typealias CIXPlugInRegistrationReleaseCallback = @convention(c) (UnsafeMutableRawPointer?) -> Void

private final class BridgePlugInRegistration: NSObject, CIPlugInRegistration {
    private let context: UnsafeMutableRawPointer?
    private let callback: CIXPlugInRegistrationCallback
    private let releaseCallback: CIXPlugInRegistrationReleaseCallback?

    init(
        context: UnsafeMutableRawPointer?,
        callback: @escaping CIXPlugInRegistrationCallback,
        releaseCallback: CIXPlugInRegistrationReleaseCallback?
    ) {
        self.context = context
        self.callback = callback
        self.releaseCallback = releaseCallback
    }

    deinit {
        releaseCallback?(context)
    }

    func load(_ host: UnsafeMutableRawPointer?) -> Bool {
        callback(context, host)
    }
}

@_cdecl("ci_plugin_registration_new")
public func ci_plugin_registration_new(
    _ context: UnsafeMutableRawPointer?,
    _ callback: CIXPlugInRegistrationCallback?,
    _ releaseCallback: CIXPlugInRegistrationReleaseCallback?
) -> UnsafeMutableRawPointer? {
    guard let callback else { return nil }
    return ci_retain(
        BridgePlugInRegistration(
            context: context,
            callback: callback,
            releaseCallback: releaseCallback
        )
    )
}

@_cdecl("ci_plugin_registration_load")
public func ci_plugin_registration_load(
    _ handle: UnsafeMutableRawPointer?,
    _ host: UnsafeMutableRawPointer?
) -> Bool {
    guard let registration: BridgePlugInRegistration = ci_borrow(handle) else {
        return false
    }
    return registration.load(host)
}

@_cdecl("ci_plugin_load_all_plugins")
public func ci_plugin_load_all_plugins() {
    CIPlugIn.loadAllPlugIns()
}

@_cdecl("ci_plugin_load_non_executable_plugins")
public func ci_plugin_load_non_executable_plugins() {
    CIPlugIn.loadNonExecutablePlugIns()
}

@_cdecl("ci_plugin_load_plugin")
public func ci_plugin_load_plugin(
    _ path: UnsafePointer<CChar>?,
    _ allowExecutableCode: Bool
) {
    guard let path else { return }
    let url = URL(fileURLWithPath: String(cString: path))
    CIPlugIn.load(url, allowExecutableCode: allowExecutableCode)
}

@_cdecl("ci_plugin_load_non_executable_plugin")
public func ci_plugin_load_non_executable_plugin(_ path: UnsafePointer<CChar>?) {
    guard let path else { return }
    let url = URL(fileURLWithPath: String(cString: path))
    CIPlugIn.loadNonExecutablePlugIn(url)
}
