// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("{} is a color word I know!", word);
    } else {
        println!("{} is not a color word I know.",word);
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt.chars().all(|c| c.is_alphanumeric())
}
