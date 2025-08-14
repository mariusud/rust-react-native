use std::process::{Command, exit};
use std::path::{Path, PathBuf};
use std::fs;

const CRATE_NAME: &str = "native_rust_lib";
const SWIFT_OUT: &str = "build/swift";
const HEADERS_OUT: &str = "build/swift/Headers";
const MODULES_OUT: &str = "build/swift/Modules";
const XCFRAMEWORK_OUT: &str = "native_rust_lib.xcframework";

// Path to your Expo module's ios folder
const EXPO_IOS_DIR: &str = "../modules/rust-module/ios";

fn main() {
    // Step 1: Build Rust for device + simulator
    run("cargo", &["build", "--release", "--target", "aarch64-apple-ios"]);
    run("cargo", &["build", "--release", "--target", "aarch64-apple-ios-sim"]);

    let device_lib = format!("target/aarch64-apple-ios/release/lib{}.a", CRATE_NAME);
    let sim_lib    = format!("target/aarch64-apple-ios-sim/release/lib{}.a", CRATE_NAME);

    // Step 2: Ensure uniffi-bindgen-swift binary is built
    let bindgen_bin = PathBuf::from("target/release/uniffi-bindgen-swift");
    run("cargo", &["build", "--release", "--bin", "uniffi-bindgen-swift"]);

    // Step 3: Generate Swift sources
    run_path(&bindgen_bin, &[
        "--swift-sources", &sim_lib, SWIFT_OUT
    ]);

    // Step 4: Generate headers
    run_path(&bindgen_bin, &[
        "--headers", &sim_lib, HEADERS_OUT
    ]);

    // Step 5: Generate XCFramework-compatible modulemap
    run_path(&bindgen_bin, &[
        "--xcframework", "--modulemap",
        "--modulemap-filename", &format!("{}.modulemap", CRATE_NAME),
        &sim_lib, MODULES_OUT
    ]);

    // Step 6: Create XCFramework
    run("xcodebuild", &[
        "-create-xcframework",
        "-library", &device_lib, "-headers", HEADERS_OUT,
        "-library", &sim_lib, "-headers", HEADERS_OUT,
        "-output", XCFRAMEWORK_OUT
    ]);

    // Step 7: Copy outputs into Expo module ios folder
    let dest_framework = Path::new(EXPO_IOS_DIR).join(format!("{}.xcframework", CRATE_NAME));
    let dest_swift = Path::new(EXPO_IOS_DIR).join("swift");

    println!("▶ Copying XCFramework to {:?}", dest_framework);
    if dest_framework.exists() {
        fs::remove_dir_all(&dest_framework).unwrap();
    }
    fs::create_dir_all(EXPO_IOS_DIR).unwrap();
    copy_dir_all(XCFRAMEWORK_OUT, &dest_framework);

    println!("▶ Copying Swift bindings to {:?}", dest_swift);
    if dest_swift.exists() {
        fs::remove_dir_all(&dest_swift).unwrap();
    }
    copy_dir_all(SWIFT_OUT, &dest_swift);

    println!("✅ All done! Podspec can now use:");
    println!("   s.vendored_frameworks = '{}'", dest_framework.file_name().unwrap().to_string_lossy());
    println!("   s.source_files = 'swift/**/*.swift'");
}

fn run(cmd: &str, args: &[&str]) {
    println!("▶ Running: {} {}", cmd, args.join(" "));
    let status = Command::new(cmd)
        .args(args)
        .status()
        .expect("failed to run command");
    if !status.success() {
        eprintln!("❌ Command failed: {} {}", cmd, args.join(" "));
        exit(1);
    }
}

fn run_path(bin: &PathBuf, args: &[&str]) {
    println!("▶ Running: {:?} {}", bin, args.join(" "));
    let status = Command::new(bin)
        .args(args)
        .status()
        .expect("failed to run binary");
    if !status.success() {
        eprintln!("❌ Binary failed: {:?} {}", bin, args.join(" "));
        exit(1);
    }
}

fn copy_dir_all<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) {
    fs::create_dir_all(&dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let ty = entry.file_type().unwrap();
        let dest_path = dst.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), &dest_path);
        } else {
            fs::copy(entry.path(), &dest_path).unwrap();
        }
    }
}
