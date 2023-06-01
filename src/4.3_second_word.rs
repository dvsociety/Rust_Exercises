fn main() {
    let string = String::from(" hello    crazy 😊 world ");
    let second_w = second_word(&string);
    println!("{}", second_w);
    }

fn second_word(s: &str) -> &str {

    let s = s.trim();
    let mut start = 0;
    let mut end = 0;

    for (index, character) in s.char_indices() {
        if character == ' ' {
            start = index;
            break;
        }
    }

    let new_s: &str= &s[start..].trim();
    println!("{new_s}");
    for (index, character) in new_s.char_indices() {
        if character == ' ' {
            end = index;
            println!("{start},{end}");
            break;
        }
    }
    &new_s[..end]
}