fn main() {
    let a: u32= 10;
    let b: u32= 7;
    println!("{}", euclides(a, b));
}

fn euclides(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        let r = a % b; 
        euclides(b, r)
    }
}