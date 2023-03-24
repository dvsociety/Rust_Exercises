fn main() {
    let message: &str = "hola mundo";
    let first_word: &str = get_fist_world(&message);
    println!("{first_word}");
}

fn get_fist_world(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index];
        }
    }
    &s
}
