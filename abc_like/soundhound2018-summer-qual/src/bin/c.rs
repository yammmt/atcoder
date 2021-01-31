// :fu: :fu: 数問 + 場合分け

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        d: i64,
    }

    let ptrn = if d == 0 {
        n
    } else {
        2 * (n - d)
    } as f64;
    println!(
        "{}",
        ptrn / ((n * n) as f64) * (m as f64 - 1.0)
    );
}
