## hello_world
This is the simplest example of overriding, and a good start for
getting the environment set up. There are two important things to
do before overriding:  
- Add the following dependencies in `Cargo.toml`:
  ```toml
  [dependencies]
  overrider = "~0.3"

  [build-dependencies]
  overrider_build = "~0.3"
  ```
- Add a `build.rs` script to watch all files with overrides.  

And that's it. See `src/main.rs` for usage.
