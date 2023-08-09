// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.



fn call_me(num: i32) {
    for i in 0..num {
        println!("{}", i);
    }
}

fn main() {
    call_me(5);
}