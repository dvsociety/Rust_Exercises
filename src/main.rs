/*Write a function that takes in a string of one or more words, 
and returns the same string, but with all five or more letter 
words reversed (Just like the name of this Kata). 
Strings passed in will consist of only letters and spaces. 
Spaces will be included only when more than one word is present.*/

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
