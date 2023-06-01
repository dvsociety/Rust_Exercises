
fn main() {
    let number = 10; 
    println!("{}", fibonacci(number));
    println!("{}", fibonacci_v2(number));
}

fn fibonacci(n: u32) -> u32 {
    if n >= 2 { 
        fibonacci(n-1) + fibonacci(n-2)
    } else if n == 1 {
        1
    } else {
        0
    }
}

fn fibonacci_v2(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut fib_prev: u32 = 0;
    let mut fib_current: u32 = 1;

    for _ in 1..n {
        let fib_next: u32 = fib_prev + fib_current;
        fib_prev = fib_current;
        fib_current = fib_next;
    }
    
    return fib_current;
}