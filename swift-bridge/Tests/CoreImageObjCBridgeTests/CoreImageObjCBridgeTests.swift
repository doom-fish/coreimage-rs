import CoreImage
import CoreImageObjCBridge
import Foundation
import XCTest

final class CoreImageObjCBridgeTests: XCTestCase {
    private let extent = CGRect(x: 0, y: 0, width: 4, height: 4)

    func testKVCExceptionIsReturnedAsNSError() {
        var error: NSError?
        let succeeded = CIXTrySetValueForKey(
            NSObject(),
            NSNumber(value: 1),
            "missingKey",
            &error
        )

        XCTAssertFalse(succeeded)
        XCTAssertNotNil(error)
    }

    func testKernelApplyExceptionsAreReturnedAsNSError() {
        let image = CIImage(color: .red).cropped(to: extent)
        var generalError: NSError?
        let general = CIXTryApplyKernel(
            unsafeBitCast(NSObject(), to: CIKernel.self),
            extent,
            { _, rect in rect },
            [image],
            &generalError
        )
        var colorError: NSError?
        let color = CIXTryApplyColorKernel(
            unsafeBitCast(NSObject(), to: CIColorKernel.self),
            extent,
            [image],
            &colorError
        )
        var warpError: NSError?
        let warp = CIXTryApplyWarpKernel(
            unsafeBitCast(NSObject(), to: CIWarpKernel.self),
            extent,
            { _, rect in rect },
            image,
            [NSNumber(value: 1)],
            &warpError
        )

        XCTAssertNil(general)
        XCTAssertNil(color)
        XCTAssertNil(warp)
        for error in [generalError, colorError, warpError] {
            XCTAssertEqual(error?.domain, "CoreImageObjCBridge")
            XCTAssertTrue(error?.localizedDescription.contains("unrecognized selector") ?? false)
        }
    }

    func testMismatchedKernelArgumentsReturnNilWithoutAnError() {
        var error: NSError?
        let image = CIXTryApplyColorKernel(
            CIBlendKernel.sourceOver,
            extent,
            [NSNumber(value: 1)],
            &error
        )

        XCTAssertNil(image)
        XCTAssertNil(error)
    }

    func testImageProcessorApplyRejectsOtherClasses() {
        var error: NSError?
        let image = CIXTryApplyImageProcessor(
            NSObject.self,
            extent,
            [],
            [:],
            &error
        )

        XCTAssertNil(image)
        XCTAssertEqual(error?.code, 2)
    }
}
