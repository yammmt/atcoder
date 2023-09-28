use proconio::input;

fn main() {
    input! {
        n: usize,
        abcn: [(i64, i64, i64); n],
    }

    let mut dp = vec![vec![0; 3]; n];
    dp[0][0] = abcn[0].0;
    dp[0][1] = abcn[0].1;
    dp[0][2] = abcn[0].2;
    for i in 1..n {
        dp[i][0] = dp[i - 1][1].max(dp[i - 1][2]) + abcn[i].0;
        dp[i][1] = dp[i - 1][0].max(dp[i - 1][2]) + abcn[i].1;
        dp[i][2] = dp[i - 1][0].max(dp[i - 1][1]) + abcn[i].2;
    }

    println!(
        "{}",
        dp[n - 1].iter().max().unwrap()
    );
}
