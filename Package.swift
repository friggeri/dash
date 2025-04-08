// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "Dash",
	products: [
		.library(
			name: "Dash",
			targets: ["Dash"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "Dash.xcframework",
			path: "ios/Dash.xcframework"
		),
		.target(
			name: "Dash",
			dependencies: ["Dash.xcframework"],
			path: "ios"
		)
	]
)
	