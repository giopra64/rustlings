// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(1000);
    println!("The square of 1000 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num
}
