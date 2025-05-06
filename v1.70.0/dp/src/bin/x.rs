use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    // 後の j+w が最大 20,000, 0 も使うので理論値最大で 20,001
    const S_MAX: usize = 20001;
    input! {
        n: usize,
        mut wsvn: [(usize, usize, usize); n],
    }

    // min(s_i, s_j-w_i) >= min(s_j, s_i-w_j) 順
    wsvn.sort_unstable_by(|a, b| {
        (a.1 as isize)
            .min(b.1 as isize - a.0 as isize)
            .cmp(&(b.1 as isize).min(a.1 as isize - b.0 as isize))
    });
    wsvn.reverse();
    let wsvn = wsvn;

    // dp[i][j]: i 個目ブロック選択完了時点で重量 j 以下時の最大価値
    let mut dp = vec![vec![0; S_MAX]; n + 1];
    for i in 0..n {
        let (w, s, v) = wsvn[i];
        for j in 0..S_MAX {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            // 載せられる？
            if j <= s {
                dp[i + 1][j + w] = dp[i + 1][j + w].max(dp[i][j] + v);
            }
        }
    }

    let mut ans = 0;
    for i in 0..S_MAX {
        ans = ans.max(dp[n][i]);
    }
    println!("{ans}");
}
