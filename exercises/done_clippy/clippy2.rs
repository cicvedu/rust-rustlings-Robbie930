// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option: Option<Vec<i32>> = Some(vec![12]);
    match option {
        Some(vec) => vec.iter().for_each(|x| res += *x),
        None => (),
    }
    println!("{}", res);
}
