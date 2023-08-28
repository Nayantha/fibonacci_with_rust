fn main() {
    println!("Hello, welcome to Fibonacci Sequence Generator!");
    for i in 0..10 {
        println!("{}th Fibonacci number is: {}", i, generate_fibonacci_sequence_up_to_n(i))
    }
    for i in 0..10 {
        println!("{}th Fibonacci number is: {}", i, generate_fibonacci_sequence_up_to_n_with_recursion(i))
    }
}
fn generate_fibonacci_sequence_up_to_n(mut n: u32) -> u32{
    let mut a = 0;
    let mut b = 1;
    while n > 0 {
        let temp = a;
        a = b;
        b += temp;
         n -= 1;
    }
    b
}

fn generate_fibonacci_sequence_up_to_n_with_recursion(n: u32) -> u32{
    if n <= 1 {
        return 1
    }
    generate_fibonacci_sequence_up_to_n_with_recursion(n) + generate_fibonacci_sequence_up_to_n_with_recursion(n-1)
}