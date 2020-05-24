fn main() {
    let number = 6;
    let fib_value = fib(number);
    println!("Fibonacci's number is {} for {}", fib_value, number);
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n -2) + fib(n-1)
    }
}
