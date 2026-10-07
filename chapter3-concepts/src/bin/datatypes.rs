#[allow(unused_variables)]
fn main() {
    let x = 2.0;
    let y: f32 = 3.0;
    let 和 = 5 + 10;
    let 差 = 95.5 - 4.3;
    let 积 = 4 * 30;
    let 商 = 56.7 / 32.2;
    let 截断 = -5 / 3;
    let 取余 = 43 % 5;
    let 真 = true;
    let 假: bool = false;
    let c: char = 'z';
    let z: &str = "Z";
    let 可爱的小螃蟹 = "🦀";
    let tup: (i32, f64, u8) = (500, 6.4, 1);
//    drop(x);
//    drop(y);
//    drop(z);
    let (x, y, z) = tup;
    println!("{可爱的小螃蟹}");
    println!("y的值是{y}!");
}
