// 22min
// WA: `u32` でオーバーフロー

// 数問

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    println!(
        "{}",
        (1 + (n - 1) * 3 + (k - 1) * (n - k) * 6) as f64
             / (n * n * n) as f64
    );
}
