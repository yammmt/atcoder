use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;
const ATCODER: &str = "atcoder";

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let add_mod = |a: usize, b: usize| (a + b) % MOD;

    // dp[i]: "atcoder" の i 文字目までを作る取り出し方の数
    let mut dp = vec![0; 8];
    dp[0] = 1;
    for c in s {
        for (i, cc) in ATCODER.chars().enumerate() {
            if c == cc {
                dp[i + 1] = add_mod(dp[i], dp[i + 1]);
            }
        }
    }

    println!("{}", dp[7]);
}
