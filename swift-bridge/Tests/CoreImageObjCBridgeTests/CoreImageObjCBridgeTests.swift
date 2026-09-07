import CoreImageObjCBridge
import Foundation
import XCTest

final class CoreImageObjCBridgeTests: XCTestCase {
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
}
