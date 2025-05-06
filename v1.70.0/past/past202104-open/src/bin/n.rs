use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        mut abn: [(usize, usize); n],
    }

    // (a/b) desc
    abn.sort_unstable_by(|x, y| {
        ((y.0 as f32) / (y.1 as f32))
            .partial_cmp(&((x.0 as f32) / (x.1 as f32)))
            .unwrap()
    });
    let abn = abn;

    // dp[i][j]: i 個の活動選択後に体力 h 以下で達成できる最大スコア
    let mut dp = vec![vec![0; h + 1]; n + 1];
    for i in 0..n {
        let (a, b) = abn[i];
        for hh in 0..h + 1 {
            dp[i + 1][hh] = dp[i + 1][hh].max(dp[i][hh]);
            let j = (hh as isize - b as isize).max(0) as usize;
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][hh] + a * hh);
        }
    }

    let mut ans = 0;
    for i in 0..h + 1 {
        ans = ans.max(dp[n][i]);
    }

    println!("{ans}");
}
