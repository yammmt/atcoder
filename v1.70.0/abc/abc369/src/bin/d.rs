use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [isize; n],
    }

    // dp[i][j]: i 匹目終了時の最大経験値で, [j] はこれまで倒した数が奇数であれば 1
    // 0 が偶数判定通ると厄介だから, 先頭項を初期値として入れて, 貰う DP にする
    let mut dp = vec![vec![0; 2]; n];
    dp[0][0] = 0;
    dp[0][1] = an[0];

    for (i, a) in an.iter().enumerate() {
        if i == 0 {
            continue;
        }

        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + 2 * a);
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + a);
    }

    println!("{}", dp[n - 1][0].max(dp[n - 1][1]));
}
