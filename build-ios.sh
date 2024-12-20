# build the lib
cargo build
 
# generate bindings
cargo run --bin uniffi-bindgen generate --library ./target/debug/libwallet.dylib --language swift --out-dir ./bindings
 
# add the iOS targets and build
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo build --release --target=aarch64-apple-ios
cargo build --release --target=aarch64-apple-ios-sim

# rename *.modulemap to module.modulemap, because XCode
mv ./bindings/walletFFI.modulemap ./bindings/module.modulemap

# cleanup
rm -rf ios

# rename and move the Swift file to the project
mkdir ios
mv ./bindings/wallet.swift ./ios/Wallet.swift
 
# create XCFramework
xcodebuild -create-xcframework \
        -library ./target/aarch64-apple-ios-sim/release/libwallet.a -headers ./bindings \
        -library ./target/aarch64-apple-ios/release/libwallet.a -headers ./bindings \
        -output "ios/Wallet.xcframework"
 
# cleanup
rm -rf bindings
