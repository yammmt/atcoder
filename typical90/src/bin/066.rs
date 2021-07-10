// 確率 DP どうしてか発想を詰めるのにやたらと頭が辛い

use proconio::input;

const RANGE_MAX: usize = 101;

fn main() {
    input! {
        n: usize,
        lrn: [(usize, usize); n],
    }

    // 誤差怖いけど回避方法が思い浮かばないので祈る
    // dp[i][j]: i 番目の要素にて j 以下の数が出る確率
    let mut dp = vec![vec![0.0; RANGE_MAX]; n];
    for (i, lr) in lrn.iter().enumerate() {
        for j in 1..RANGE_MAX {
            dp[i][j] = dp[i][j - 1];
            if lr.0 <= j && j <= lr.1 {
                dp[i][j] += 1.0 / (lr.1 - lr.0 + 1) as f64;
            }
        }
    }
    // println!("{:?}", dp);

    let mut ans = 0.0;
    for i in 0..n {
        for j in lrn[i].0..lrn[i].1 + 1 {
            let appears_pct = 1.0 / (lrn[i].1 - lrn[i].0 + 1) as f64;
            for k in i + 1..n {
                ans += appears_pct * dp[k][j - 1];
            }
        }
    }

    println!("{}", ans);
}
