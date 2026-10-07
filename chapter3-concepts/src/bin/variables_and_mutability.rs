fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("三个小时是{THREE_HOURS_IN_SECONDS}秒");
    let mut x = 5;
    println!("x={x}");
    x = 6;
    println!("x={x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("内部作用域中:x={x}");
    }
    println!("x={x}");
    /*let space = "   ";
    let space = space.len();*/
    /*let mut space = "   ";
    space = space.len();*/
}
