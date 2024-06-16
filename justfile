default:
    just --list

# env:
#     export ANDROID_HOME=/Users/justin/Library/Android/sdk
#     alias emulator=$ANDROID_HOME/emulator/emulator

run-android-emulator:
    # FIRST_AVD=$(/Users/justin/Library/Android/sdk/emulator/emulator -list-avds | head -n 1)
    # echo "Starting emulator $FIRST_AVD"
    /Users/justin/Library/Android/sdk/emulator/emulator -avd Pixel_3a_API_34_extension_level_7_arm64-v8a &
    # TODO: wait for emulator to boot
    # TODO: skip emulator boot if already booted

build-android:
    bash scripts/build-android.sh

run-android: build-android run-android-emulator
    bash scripts/run-android.sh

build-ios profile="debug":
    bash scripts/build-ios.sh {{profile}}

run-ios: build-ios
    bash scripts/run-ios.sh
