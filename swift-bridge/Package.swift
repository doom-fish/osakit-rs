// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "OSAKitBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "OSAKitBridge",
            type: .static,
            targets: ["OSAKitBridge"]
        )
    ],
    targets: [
        .target(
            name: "OSAKitBridge",
            path: "Sources/OSAKitBridge",
            publicHeadersPath: "include"
        )
    ]
)
