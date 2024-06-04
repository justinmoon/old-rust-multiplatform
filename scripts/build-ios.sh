#!/bin/bash
set -ex
cd rust

# Build the dylib
cargo build
 
# Generate bindings
cargo run --bin uniffi-bindgen generate --library ./target/debug/libcounter.dylib --language swift --out-dir ./bindings
 
        # aarch64-apple-darwin \
        # aarch64-apple-ios \
        # aarch64-apple-ios-sim \
        # x86_64-apple-darwin \
        # x86_64-apple-ios
# Add the iOS targets and build
for TARGET in \
        aarch64-apple-ios-sim
do
    rustup target add $TARGET
    cargo build --release --target=$TARGET
done
 
# Rename *.modulemap to module.modulemap
mv ./bindings/counterFFI.modulemap ./bindings/module.modulemap
 
# Move the Swift file to the project
rm ./ios/TodoList/Counter.swift || true
mv ./bindings/counter.swift ./ios/TodoList/Counter.swift
 
# Recreate XCFramework
rm -rf "ios/Counter.xcframework" || true
        # -library ./target/aarch64-apple-ios/release/libcounter.a -headers ./bindings \
xcodebuild -create-xcframework \
        -library ./target/aarch64-apple-ios-sim/release/libcounter.a -headers ./bindings \
        -output "ios/Counter.xcframework"
 
# Cleanup
rm -rf bindings