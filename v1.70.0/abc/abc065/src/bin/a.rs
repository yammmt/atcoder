use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        x: i32,
        a: i32,
        b: i32,
    }

    println!(
        "{}",
        if a - b >= 0 {
            "delicious"
        } else if b - a >= x + 1 {
            "dangerous"
        } else {
            "safe"
        }
    );
}
