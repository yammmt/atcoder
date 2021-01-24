// :fu: :fu: :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }
    let vmax = 100_001;

    // 価値 v を達成する最小の重さ
    let mut dp = vec![vec![std::usize::MAX / 2; vmax]; n + 1];
    dp[0][0] = 0;
    for (i, wv) in wvn.iter().enumerate() {
        for j in 0..vmax {
            dp[i + 1][j] = if j >= wv.1 {
                dp[i][j].min(dp[i][j - wv.1] + wv.0)
            } else {
                dp[i][j]
            };
        }
        // println!("{:?}", dp);
    }

    let mut ans = 0;
    for j in 0..vmax {
        if dp[n][j] <= w {
            ans = ans.max(j);
        }
    }
    println!("{}", ans);
}
