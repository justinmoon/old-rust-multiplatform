build-android:
    bash scripts/build-android.sh

run-android:
    bash scripts/run-android.sh

build-ios:
    bash scripts/build-ios.sh

run-ios:
    bash scripts/run-ios.sh

build-wasm:
    wasm-pack build --target web --out-dir build rust/
