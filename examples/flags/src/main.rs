use overrider::*;

#[default]
fn main() {
    println!("Default");
}

#[override_flag(flag = change)]
fn main() {
    println!("Changed by flag --change");
}
/*
#[override_flag(a)]
fn main() {
    println!("Changed by flag -a");
}
 */

fn main() {
    __override_flagext_change_main();
}
