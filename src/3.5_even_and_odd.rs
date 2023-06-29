// If the number is even, it should be returned in binary; otherwise, in hexadecimal.

fn main() {
    let number = 31;
    println!("{}, {}", is_even(number), even_and_odd(number));
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn even_and_odd(n: i32) -> String {
    if n % 2 == 0 {
        format!("{:b}", n)
    } else {
        format!("{:x}", n)
    }
}