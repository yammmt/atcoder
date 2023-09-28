// :fu: 21-12 数問 + 場合分け

use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
        d: i64,
    }

    let pts = if d == 0 {
        n
    } else {
        2 * (n - d)
    } as f64;
    println!("{}", pts / (n * n) as f64 * (m - 1) as f64);
}
