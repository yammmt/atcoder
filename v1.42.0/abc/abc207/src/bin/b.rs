// 難読

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }
    let bunshi = c * d - b;
    println!(
        "{}",
        if bunshi <= 0 {
            -1
        } else {
            (a + bunshi - 1) / bunshi
        }
    );
}
