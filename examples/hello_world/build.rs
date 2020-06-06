fn main() {
    // Watch the following files for override attrs
    overrider_build::watch_files(vec!["src/*.rs"]);

    /*
     * NOTE:
     * watch_files should only be called once
     */
}
