fn main() {
    println!("Input ISBN:\n");
    let mut isbn = String::new();
    let _ = std::io::stdin().read_line(&mut isbn);
    let isbn_array = isbn_to_u8_array(&isbn);
    let is_valid = validate_isbn(isbn_array);

    print!(
        "ISBN: {} is {}",
        isbn.trim(),
        if is_valid { "valid" } else { "invalid" }
    );
}

fn isbn_to_u8_array(isbn: &str) -> [u32; 10] {
    let mut result: [u32; 10] = [0; 10];
    let clean_isbn = isbn.replace("-", "").trim().to_string();

    if clean_isbn.len() != 10 {
        panic!("ISBN must be 10 digits long");
    }

    for (i, c) in clean_isbn.chars().take(10).enumerate() {
        result[i] = if c == 'x' || c == 'X' {
            10
        } else {
            c.to_digit(10).expect("Invalid ISBN character")
        };
    }

    result
}

fn validate_isbn(isbn: [u32; 10]) -> bool {
    let mut sum = 0;
    for (i, &digit) in isbn.iter().enumerate() {
        sum += digit * (10 - i) as u32;
    }

    sum % 11 == 0
}
