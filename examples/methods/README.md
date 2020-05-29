## methods
This example shows some of the other things that `overrider` can handle.  
The items that can be overriden are as follows:
- functions
- methods
- impl constants

It's important to know that when overriding things related to a struct,
the `#[default]` and  
`#[override_default]` must be attached to the `impl`.
This is because `proc_macro` can  
only see the thing it's attached to.
