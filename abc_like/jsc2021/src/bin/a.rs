// A にしては難しめ

use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        z: i64,
    }
    println!(
        "{}",
        if (y * z) % x == 0 {
            y * z / x - 1
        } else {
            y * z / x
        }
    );
}
