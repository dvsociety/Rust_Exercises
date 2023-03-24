fn main() {
    let phrase = String::from("  Hola Mundo ");
    let trim_spaces = trim_spaces(&phrase);
    println!("{}", trim_spaces);
    }

fn trim_spaces(s: &str) -> &str {
    
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }
    &s[start..end]
}