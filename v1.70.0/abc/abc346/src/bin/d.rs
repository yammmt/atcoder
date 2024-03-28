// 遷移ごっちゃになる, なんとかして
// WA:
// 2
// 01
// 100 200

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const DUMMY: usize = usize::MAX / 3;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        cn: [usize; n],
    }

    // dp[i][j][k]: i 文字目まで見終わった際のコスト
    // i は 0-origin
    // j = 1 なら i-1 文字目は変わっている
    // k = 1 なら i 文字目と i+1 文字目が一致する組は既出
    let mut dp = vec![vec![vec![DUMMY; 2]; 2]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        // i 文字目を変えない
        if i == 0 {
            dp[1][0][0] = 0;
        } else if s[i] == s[i - 1] {
            // 直前を変えていなければ, 今回は一致
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][0]);
            // 直前を変えていれば, 今回は不一致
            dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][1][0]);
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][1][1]);
        } else {
            // 直前を変えていなければ, 今回は不一致
            dp[i + 1][0][0] = dp[i + 1][0][0].min(dp[i][0][0]);
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][0][1]);
            // 直前を変えていれば, 今回は一致
            dp[i + 1][0][1] = dp[i + 1][0][1].min(dp[i][1][0]);
        }

        // i 文字目を変える
        if i == 0 {
            dp[1][1][0] = cn[0];
        } else if s[i] == s[i - 1] {
            // 直前を変えていなければ, 今回は不一致
            dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][0][0] + cn[i]);
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][1] + cn[i]);
            // 直前を変えていれば, 今回は一致
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][0] + cn[i]);
        } else {
            // 直前を変えていなければ, 今回は一致
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][0][0] + cn[i]);
            // 直前を変えていれば, 今回は不一致
            dp[i + 1][1][0] = dp[i + 1][1][0].min(dp[i][1][0] + cn[i]);
            dp[i + 1][1][1] = dp[i + 1][1][1].min(dp[i][1][1] + cn[i]);
        }
    }

    println!("{}", dp[n][0][1].min(dp[n][1][1]));
}
