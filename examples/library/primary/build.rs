fn main() {
    let libpath = "../secondary/src/lib.rs";
    overrider_build::watch_files(vec!["src/main.rs", libpath]);

    // Because the library file is in a seperate crate, by default cargo does
    // not check for changes on it, so neither does overrider. This fixes that
    println!("cargo:rerun-if-changed={}", libpath);
}
