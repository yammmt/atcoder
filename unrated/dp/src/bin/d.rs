// :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    // 品物 i を選ぶか否か
    for i in 0..n {
        // j: 今の重さ
        for j in 0..w + 1 {
            dp[i + 1][j] = if j >= wvn[i].0 {
                // 品物 i を選べる場合
                (dp[i][j - wvn[i].0] + wvn[i].1).max(dp[i][j])
            } else {
                dp[i][j]
            };
        }
        // println!("{:?}", dp);
    }

    println!("{}", dp[n][w]);
}
