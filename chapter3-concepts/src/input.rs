use std::io::{self, Write};

pub fn input(提示词:&str) -> String{
    print!("{提示词}");
    io::stdout().flush().expect("刷新失败");
    let mut 输入内容 = String::new();
    io::stdin().read_line(&mut 输入内容).expect("读取行错误");
    输入内容.trim().to_string()
}