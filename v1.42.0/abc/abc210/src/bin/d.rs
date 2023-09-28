// :fu: 21-07 DP 苦手で発想段階でだめ

use proconio::input;

static DUMMY: i64 = std::i64::MAX / 4;

fn dp_min(h: usize, w: usize, c: i64, ahw: &[Vec<i64>]) -> i64 {
    let mut ret = DUMMY;
    // dp[i][j]: (i, j) に二つ目の駅を立てた場合の最小コスト
    let mut dp = vec![vec![DUMMY; w]; h];
    // 配る DP
    for i in 0..h {
        for j in 0..w {
            ret = ret.min(dp[i][j] + ahw[i][j]);
            let my_cost = ahw[i][j] + c;
            if i + 1 < h {
                dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + c);
                dp[i + 1][j] = dp[i + 1][j].min(my_cost);
            }
            if j + 1 < w {
                dp[i][j + 1] = dp[i][j + 1].min(dp[i][j] + c);
                dp[i][j + 1] = dp[i][j + 1].min(my_cost);
            }
        }
    }
    ret
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: i64,
        ahw: [[i64; w]; h],
    }

    let mut ahwr = vec![vec![]; h];
    for i in 0..h {
        ahwr[i] = ahw[h - i - 1].clone();
    }
    // println!("{:?}", ahwr);

    println!("{}", dp_min(h, w, c, &ahw).min(dp_min(h, w, c, &ahwr)));
}
