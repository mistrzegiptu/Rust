fn main() {
    let mut v = vec![1, 2, 3];

    for e in &mut v {
        *e *= 2;
        println!("{}", e);
    }

    for (i, e) in v.iter().enumerate() {
        println!("element -> {} at index -> {}", e, i);
    }
}