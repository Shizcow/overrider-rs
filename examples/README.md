# examples

This is a collection of examples for the `overrider` crates.  
They are listed below in order of increasingly advanced usage.

### hello_world
A simple hello world program, showing the basics of overriding.  
The important parts are in `src/main.rs` and `build.rs`.

### methods
A demo showing that methods (and indeed most things in an `impl`
block) can be overriden.

### priorities
An example of how to gracefully handle overriding more than once.

### Invert
How to make something default, but turned off on a flag.

### finals
Say you're working on a large project, with functions and overrides
spread out over several files. How do you know that the function
you're currently working on will actually be ran? `#[override_final]`
is your friend.

### library
This is not a simple example. To run this, envoke `cargo run -p primary`.

Now that you've been working on a project for a while, you may find yourself
semi-dynamically loading entire files or libraries of overriden objects.
Those dependencies may get pretty complicated, eventually requiring their own
build scripts and sub-dependencies, which may not be the same version as the
crate root. Solution? Seperate `#[override]` definitions into an entire seperate
crate. The only care that needs to be taken here is additional flags in `build.rs`
and `Cargo.toml` to make sure all functions are included.

## What's next?
If you've made it through all these exercies and like this crate, check out
[proc_use](https://github.com/Shizcow/proc_use/). Combining these two crates
together allows for some seriously powerful solutions, where entire sets of
plugins can automatically be compiled in or out at buildtime.
