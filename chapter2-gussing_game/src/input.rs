use std::io;

pub fn input() -> String {
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("读取行失败");
    temp.trim().to_string()
}
