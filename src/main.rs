fn main() {
    println!("Hello, world!");
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