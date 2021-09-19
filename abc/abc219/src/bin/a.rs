use proconio::input;

fn main() {
    input! {
        x: i32,
    }
    if x >= 90 {
        println!("expert");
    } else {
        println!(
            "{}",
            if x < 40 {
                40 - x
            } else if x < 70 {
                70 - x
            } else {
                90 - x
            }
        );
    }
}
