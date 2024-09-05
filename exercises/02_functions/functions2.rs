<<<<<<< HEAD
// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    call_me(20);
}

fn call_me(num: i32) {
=======
// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num:) {
>>>>>>> 2d0860fe1bd0aef512313617d8a26e9f118d2cd2
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
