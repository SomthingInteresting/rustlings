// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut x = 3; // in rust you can't change the value of a variable unless you declare it as mutable
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
