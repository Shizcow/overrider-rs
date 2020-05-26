fn main() {
    // Watch the following files for override attrs
    overrider_build::watch_files!("src/main.rs");

    /*
     * NOTE:
     * Only need to watch files with #[override_default]
     * Files with #[override] only do not need to be watched
     * In this example, everything is in the same file
     */
}
