use std::{env, fs, process::Command, path::Path};

fn run(cmd: &mut Command) {
    println!("▶ Running: {:?}", cmd);
    let status = cmd.status().expect("Failed to run command");
    if !status.success() {
        panic!("Command {:?} failed with {}", cmd, status);
    }
}

fn main() {
    let module_dir = "../modules/rust-module";
    let jni_libs_dir = format!("{}/android/src/main/jniLibs", module_dir);
    let java_bindings_dir = format!("{}/android/src/main/java/uniffi", module_dir);
    let lib_name = "native_rust_lib";
    let min_sdk = "21"; // According to https://claas.dev/posts/expo-with-rust/#step-by-step this will break if set lower

    let abis = vec![
        ("aarch64-linux-android", "arm64-v8a"),
        ("armv7-linux-androideabi", "armeabi-v7a"),
        ("i686-linux-android", "x86"),
        ("x86_64-linux-android", "x86_64"),
    ];

    println!("📦 Building Android Rust libraries...");
    for (rust_target, android_dir) in &abis {
        println!("🛠 Building for {} -> {}", rust_target, android_dir);
        run(Command::new("cargo")
            .arg("ndk")
            .arg("--target").arg(rust_target)
            .arg("--platform").arg(min_sdk)
            .arg("build")
            .arg("--release")
            .arg("--lib"));

        let target_path = format!("target/{}/release/lib{}.so", rust_target, lib_name);
        let out_dir = format!("{}/{}", jni_libs_dir, android_dir);
        fs::create_dir_all(&out_dir).unwrap();
        fs::copy(&target_path, format!("{}/lib{}.so", out_dir, lib_name))
            .expect("Failed to copy .so file");
    }

    println!("📜 Generating Kotlin bindings with UniFFI...");
    run(Command::new("cargo")
        .arg("run")
        .args(["--bin", "uniffi-bindgen", "generate"])
        .arg("--library").arg(format!("target/aarch64-linux-android/release/lib{}.so", lib_name))
        .arg("--language").arg("kotlin")
        .arg("--out-dir").arg("generated/kotlin"));

    println!("📂 Moving Kotlin bindings into Android source tree...");
    fs::create_dir_all(&java_bindings_dir).unwrap();
    let generated_path = Path::new("generated/kotlin/uniffi");
    if generated_path.exists() {
        for entry in walkdir::WalkDir::new(&generated_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                let dest = Path::new(&java_bindings_dir).join(entry.path().strip_prefix(&generated_path).unwrap());
                if let Some(parent) = dest.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                fs::copy(entry.path(), &dest).unwrap();
            }
        }
    }

    println!("✅ Android build complete!");
}
