use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [isize; n],
    }

    // WA:
    // 5 3
    // 1 2 3 4 5

    // dp[n][m][2]: n 日目終了時, m 日宿題した場合の楽しさ最大値, [2] は 1 であれば宿題
    let mut dp = vec![vec![vec![-1; 2]; m + 1]; n];
    // 貰う
    for i in 0..n {
        if i == 0 {
            dp[0][0][0] = an[0];
            if m != 0 {
                dp[0][1][1] = 0;
            }
        } else {
            for j in 0..=m {
                // 宿題する (連続ではしないので [1] は見ない)
                if j + 1 <= m {
                    if dp[i - 1][j][0] != -1 {
                        dp[i][j + 1][1] = dp[i][j + 1][1].max(dp[i - 1][j][0]);
                    }
                }
                // 宿題しない
                if dp[i - 1][j][0] != -1 {
                    dp[i][j][0] = dp[i][j][0].max(dp[i - 1][j][0] + an[i]);
                }
                if dp[i - 1][j][1] != -1 {
                    dp[i][j][0] = dp[i][j][0].max(dp[i - 1][j][1] + an[i]);
                }
            }
        }
        // println!("day {}", i+1);
        // println!("  {:?}", dp[i]);
    }

    println!("{}", dp[n - 1][m][0].max(dp[n - 1][m][1]));
}
