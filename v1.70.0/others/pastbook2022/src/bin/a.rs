use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        cnn: [[usize; n + 1]; n + 1],
    }

    // dp[i]: 最初の i 個の要素を考えたときの最小コスト
    let mut dp = vec![DUMMY; n + 1];
    dp[0] = 0;

    for i in 1..=n {
        for j in 0..i {
            dp[i] = dp[i].min(dp[j] + cnn[j][i]);
        }
    }

    println!("{}", dp[n]);
}
