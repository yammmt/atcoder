use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    // i 個目の荷物
    for (i, wv) in wvn.iter().enumerate() {
        for j in 0..w + 1 {
            dp[i + 1][j] = if j >= wv.0 {
                dp[i][j].max(dp[i][j - wv.0] + wv.1)
            } else {
                dp[i][j]
            };
        }
        // println!("{:?}", dp);
    }

    println!("{}", dp[n][w]);
}
