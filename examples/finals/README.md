## finals
This example introduces `#[override_final]`, a tool which helps ensure that an item
in development will certainly be compiled and ran. Unlike the `final` keyword in
most languages, this does not make it impossible to override. In fact,  
`#[override_final]` intentionally throws a compiler error with a helpful note stating
what attribute the item needs to guarentee execution at runtime.
**This example will fail compile.**
