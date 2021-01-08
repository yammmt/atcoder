use proconio::input;

fn main() {
    input! {
        n: usize,
        hn: [i64; n],
    }

    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        if i > 0 {
            dp[i] = dp[i].min(dp[i - 1] + (hn[i] - hn[i - 1]).abs());
        }
        if i > 1 {
            dp[i] = dp[i].min(dp[i - 2] + (hn[i] - hn[i - 2]).abs());
        }
    }

    println!("{}", dp[n - 1]);
}
