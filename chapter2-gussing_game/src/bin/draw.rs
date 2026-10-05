use chapter2_gussing_game::input::input;

fn main() {
    let num: i64 = input().parse().unwrap();
    for i in 1..=num {
        for _ in 1..=(num - i) {
            print!(" ");
        }
        for _ in 1..=(2 * i - 1) {
            print!("*");
        }
        println!();
    }
    for i in (0..num).rev() {
        for _ in 1..=(num - i) {
            print!(" ");
        }
        for _ in 1..=(2 * i - 1) {
            print!("*");
        }
    }
}
