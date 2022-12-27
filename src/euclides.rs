fn main() {
    println!("{}",euclides(543, 240));
}

fn euclides(m: u32, n: u32) -> u32 {
    if n == 0 {
        m
    }
    else {
        let r = m % n;
        euclides(n, r)
    }
}