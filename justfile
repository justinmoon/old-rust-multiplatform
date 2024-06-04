ndk:
	cargo ndk -o ./app/src/main/jniLibs \
        --manifest-path ./Cargo.toml \
        -t armeabi-v7a \
        -t arm64-v8a \
        -t x86 \
        -t x86_64 \
        build --release

kotlin:
        cargo run --bin uniffi-bindgen \
        generate --library ./target/debug/libmobile.dylib \
        --language kotlin \
        --out-dir ./app/src/main/java/tech/forgen/todolist/rust

