// 緑 diff だけど順位表見ると緑ほとんど正解してないような
// 参加者の母集団の問題？

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [u64; n],
    }
    println!(
        "{}",
        if an.iter().any(|a| *a % 2 == 1) {
            "first"
        } else {
            "second"
        }
    );
}
