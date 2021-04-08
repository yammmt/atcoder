// :fu: :fu: 21-04 区間 DP

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    // dp[i][j]: 区間 [i, j) が抜き取られた場合の x - y
    let mut dp = vec![vec![0; n + 1]; n + 1];

    for len in 1..n + 1 {
        let mut i = 0;
        while i + len <= n {
            let j = i + len;

            if (n - len) % 2 == 0 {
                // 先手
                dp[i][j] = (dp[i + 1][j] + an[i]).max(dp[i][j - 1] + an[j - 1]);
            } else {
                // 後手
                dp[i][j] = (dp[i + 1][j] - an[i]).min(dp[i][j - 1] - an[j - 1]);
            }

            i += 1;
        }
    }

    println!("{}", dp[0][n]);
}
