// :fu: 21-03 ゲーム系

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    println!(
        "{}",
        if a >= n {
            "Takahashi"
        } else if a == b {
            if n % (a + 1) == 0 {
                "Aoki"
            } else {
                "Takahashi"
            }
        } else if a > b {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
