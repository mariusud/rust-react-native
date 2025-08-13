## Running rust code directly in React Native

I tried following https://github.com/dgca/expo-rust-demo/tree/main, but it had some slightly outdated code and I wanted to make it simpler and use the more modern .xcframework for iOS which doesn't require you to specify whether you're running on simulator or not.

npx expo prebuild

```bash
rm -rf ~/Library/Developer/Xcode/DerivedData/
cd ios
pod deintegrate
pod install
cd ..
```

## Android

## Building rust code

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
cargo install cargo-ndk
```

##### iOS

```bash
./build.sh
```

##### Android

```bash
cargo ndk --target aarch64-linux-android --platform 31 -- build --release
cargo ndk --target armv7-linux-androideabi --platform 31 -- build --release
cargo ndk --target i686-linux-android --platform 31 -- build --release
cargo ndk --target x86_64-linux-android --platform 31 -- build --release
```
