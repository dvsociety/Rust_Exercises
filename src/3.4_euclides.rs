fn main() {
    println!("{}",euclides(543, 240));
    println!("{}",euclides_v2(543, 240));
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

fn euclides_v2(mut a:u32, mut b:u32) -> u32 {
    while b != 0 {
        let r:u32 = a % b; 
        a = b; 
        b = r;
    }
    a
}