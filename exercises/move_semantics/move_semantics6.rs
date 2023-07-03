// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("Last character: {}", last_char);

    let uppercased_data = string_uppercase(data);
    println!("Uppercased data: {}", uppercased_data);
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) -> String {
    data = data.to_uppercase();

    println!("{}", data);

    data
}
