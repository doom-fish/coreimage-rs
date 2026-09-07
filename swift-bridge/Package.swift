// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "CoreImageBridge",
    platforms: [
        .macOS(.v11)
    ],
    products: [
        .library(
            name: "CoreImageBridge",
            type: .static,
            targets: ["CoreImageBridge"])
    ],
    targets: [
        .target(
            name: "CoreImageObjCBridge",
            path: "Sources/CoreImageObjCBridge",
            publicHeadersPath: "include"),
        .target(
            name: "CoreImageBridge",
            dependencies: ["CoreImageObjCBridge"],
            path: "Sources/CoreImageBridge",
            publicHeadersPath: "include"),
        .testTarget(
            name: "CoreImageObjCBridgeTests",
            dependencies: ["CoreImageObjCBridge"],
            path: "Tests/CoreImageObjCBridgeTests")
    ]
)
