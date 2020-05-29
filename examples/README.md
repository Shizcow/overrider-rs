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

### finals
Say you're working on a large project, with functions and overrides
spread out over several files. How do you know that the function
you're currently working on will actually be ran? `#[override_final]`
is your friend.
