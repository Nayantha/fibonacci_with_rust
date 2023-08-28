fn main() {
    println!("Hello, world!");
}
fn generate_fibonacci_sequence_up_to_n(n: u32) -> u32{
    let mut a = 0;
    let mut b = 1;
    for i in 0..n {
        a = b;
        b += a;
    }
    b
}