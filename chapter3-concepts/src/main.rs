use chapter3_concepts::input::input;
use std::io::{self, Write};
fn main() {
    print!("hello, world!!");
    println!();
    println!("test");
//    println!();
    for _ in 1..=5{
        let a = input("输入文本");
        io::stdout().write(a.as_bytes()).unwrap();
        io::stdout().write(b"\n").unwrap();
    }
}
