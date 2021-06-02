// 2.5min
// EDPC の最初と同じ

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
    }
    let d = 1_000_000_007;

    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] = (dp[i + 1] + dp[i]) % d;
        if i + l <= n {
            dp[i + l] = (dp[i + l] + dp[i]) % d;
        }
    }

    println!("{}", dp[n]);
}
