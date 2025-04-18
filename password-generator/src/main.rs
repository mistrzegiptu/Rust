use rand::{Rng, rng};

fn main() {
    println!("{}", generate_password(10, &["lowercase", "special"]))
}

fn generate_password(length: usize, charsets: &[&str]) -> String {
    let lowercase_set: String = String::from("abcdefghijklmnopqrstuvwxyz");
    let uppercase_set: String = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    let digits: String = String::from("0123456789");
    let specials: String = String::from("!@#$%^&*()_+-=[]{}|;:,.<>?");

    let mut available_chars: String = String::new();
    let mut all: bool = false;

    for &charset in charsets {
        match charset {
            "lowercase" => available_chars.push_str(&lowercase_set),
            "uppercase" => available_chars.push_str(&uppercase_set),
            "digits" => available_chars.push_str(&digits),
            "specials" => available_chars.push_str(&specials),
            _ => all = true,
        }
    }

    if all || available_chars.is_empty() {
        available_chars.push_str(&lowercase_set);
        available_chars.push_str(&uppercase_set);
        available_chars.push_str(&digits);
        available_chars.push_str(&specials);
    }

    let mut password: String = String::new();
    for _ in 0..length {
        let rand_index: usize = rng().random_range(0..available_chars.len());
        let random_char = available_chars.chars().nth(rand_index).unwrap();
        password.push(random_char)
    }

    password
}
