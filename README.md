## Running Rust code directly in React Native

This project demonstrates how to integrate **Rust** into a React Native application and run Rust code natively on both iOS (Swift) and Android (Kotlin). It uses [UniFFI](https://github.com/mozilla/uniffi-rs) to generate language bindings automatically and leverages modern tooling such as `.xcframework` for iOS (so you don’t need to manually manage simulator/device builds).

The goal is to provide the **simplest possible example** of wiring Rust into a React Native project while keeping the setup clean and future-proof.

## 🚀 Getting Started

Clone the repo and install dependencies:

```bash
npm install

Then you can run the example on iOS or Android:

npm run ios
npm run android
```

## ✨ Features

- 🔧 **UniFFI support**: Simplifies exposing Rust functions to Swift and Kotlin.
- 📱 **Cross-platform bindings**: Works with iOS and Android out of the box.
- ⚡ **React Native integration**: Call Rust code directly from your JS/TS code.
- 📦 **Modern iOS build system**: Uses `.xcframework` for easy simulator/device handling.
- 🛠 **Minimal example**: Focused on clarity, this is a pure minimalistic example so you can see the bare minimum required to run your rust code directly in native code.

## ⚙️ How It Works

1. native_rust_lib library
   The business logic lives in a separate Rust library, native_rust_lib. Functions you want to expose must use #[uniffi::record], which ensures they are included in the generated bindings via [procmacros](https://mozilla.github.io/uniffi-rs/0.27/proc_macro/index.html)

2. UniFFI code generation
   UniFFI automatically generates Swift and Kotlin bindings from the annotated Rust code using ProcMacros.

3. Native module integration
   React Native bridges the generated Swift/Kotlin code so you can call it from JavaScript/TypeScript.

4. React Native usage
   In your JS/TS files, you can import and use the exposed functions as if they were ordinary native modules.

## Building rust code

I have also included instructions on how you can build your own library and use that in your React-Native project. The scripts build-ios and build-android will:

- Compile your Rust code for the proper mobile targets
- Package the output into an .xcframework (for iOS) and .aar (for Android).
- Move the files to the correct folders in your React Native project.

```bash
cargo run --release --bin build-ios
cargo run --release --bin build-android

```
