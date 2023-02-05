use std::io;

fn main() {
    println!("What is your name?");
    let mut input = String::new();
    let result = io::stdin().read_line(&mut input);
    result.expect("failed to read input");
    print!("Hello {input}")
}
