fn main() {
    println!("{}", fibonacci(10));
}

fn fibonacci(n: u32) -> u32 {
    if n > 1 {
        fibonacci(n-1) + fibonacci(n-2)
    }
    else if n == 1 {
        1 
    }
    else {
        0
    }
}