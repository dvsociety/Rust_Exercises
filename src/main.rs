fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 y r2 no deben usarse después de este punto

    let r3 = &mut s; // no problem
    println!("{}", r3);

}