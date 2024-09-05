<<<<<<< HEAD
// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(10);
}

fn call_me(num: u32) {
=======
fn call_me(num: u8) {
>>>>>>> 2d0860fe1bd0aef512313617d8a26e9f118d2cd2
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me();
}
