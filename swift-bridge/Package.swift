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
            name: "CoreImageBridge",
            path: "Sources/CoreImageBridge",
            publicHeadersPath: "include")
    ]
)
