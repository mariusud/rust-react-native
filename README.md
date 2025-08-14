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
cargo run --release --bin build-ios
cargo run --release --bin build-android

```
