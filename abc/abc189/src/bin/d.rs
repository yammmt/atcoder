// WA: 32bit でオーバーフロー
// 条件を逆にして dp して考え纏まらず死

use proconio::input;

fn main() {
    input! {
        n: usize,
        sn: [String; n],
    }

    let mut dp = vec![vec![0u64; 2]; n + 1];
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 0..n {
        let total = (dp[i][0] + dp[i][1]) * 2;
        if sn[i] == "OR" {
            dp[i + 1][0] = total - dp[i][1];
            dp[i + 1][1] = dp[i][1];
        } else {
            // AND
            dp[i + 1][0] = dp[i][0];
            dp[i + 1][1] = total - dp[i][0];
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[n][0]);
}
