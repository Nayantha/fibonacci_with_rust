fn main() {
    println!("Hello, welcome to Fibonacci Sequence Generator!");
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

fn use_list_to_generate_fibonacci(mut n: usize) -> Vec<i32>{
    let mut fibonacci_sequence = vec![0, 1, 1];
    for i in 3..n + 1 {
        fibonacci_sequence.push(fibonacci_sequence[i-1] + fibonacci_sequence[i-2])
    }
    fibonacci_sequence
}