use std::io;

fn main() {
    let mut number = String::new();

    println!("Enter a number: ");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please type a number!");

    println!("Fibonacci number {} is {}", number, fib(number));
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

