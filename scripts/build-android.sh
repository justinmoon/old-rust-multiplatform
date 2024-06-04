#!/bin/bash

cd rust
 
# Set up cargo-ndk and add the Android targets
cargo install cargo-ndk
rustup target add aarch64-linux-android \
    armv7-linux-androideabi \
    i686-linux-android \
    x86_64-linux-android
 
# Build the dylib
cargo build
 
# Build the Android libraries in jniLibs
cargo ndk -o ./app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release
 
# Create Kotlin bindings
cargo run --bin uniffi-bindgen generate \
    --library ./target/debug/libcounter.dylib \
    --language kotlin \
    --out-dir ./app/src/main/java/tech/forgen/todolist/rust