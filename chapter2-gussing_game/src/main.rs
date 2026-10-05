use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    println!("猜数字小游戏(范围1~100):");
    let secret_num = rand::rng().random_range(1..=100);
    //    println!("保密的数字是{secret_num}");
    loop {
        println!("请输入一个数字:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取行失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("这是数字吗你就输？？");
                continue;
            }
        };
        if (1..=100).contains(&guess) {
            println!("你猜测的数字是: {guess}");
            match guess.cmp(&secret_num) {
                Ordering::Less => println!("小了"),
                Ordering::Equal => {
                    println!("对了");
                    break;
                }
                Ordering::Greater => println!("大了"),
            }
        } else {
            println!("输入的数字不在范围内");
        }
    }
}
