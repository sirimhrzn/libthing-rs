// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let binaryTarget: Target

binaryTarget = .binaryTarget(
    name: "CalendarCoreRs",
        // As stated by Ferrostar
        // IMPORTANT: Swift packages importing this locally will not be able to
        // import Ferrostar core unless you specify this as a relative path!
    path: "./common/target/ios/libcalendarcore-rs.xcframework"
)

let package = Package(
    name: "CalendarCore",
    defaultLocalization: "en",
    platforms: [
        .iOS(.v16),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "CalendarCore", //
            targets: ["CalendarCoreLib"]
        ),
        // .library(
        //     name: "CalendarUtils",
        //     targets: ["CalendarUtils"]
        // ),
    ],

   targets: [
       binaryTarget,
        .target(
            name: "CalendarCoreLib", // Here we expose helper functions
            path: "apple/calendarcore"
        ),
    ]
)
