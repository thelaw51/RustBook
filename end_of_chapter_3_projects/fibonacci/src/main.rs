use std::io;
fn main() {
    println!("Enter the index of the Fibonacci you want to find: ");
    let mut fib_index = String::new();

    io::stdin()
        .read_line(&mut fib_index)
        .expect("can not read index entered");
    let fib_index: u32 = fib_index.parse().expect("can not parse enterted index");
}
