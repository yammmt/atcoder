use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut pn: [f64; n],
    }
    pn.reverse();

    let mut zerop9 = vec![1.0];
    while zerop9.len() < n {
        zerop9.push(zerop9.last().unwrap() * 0.9);
    }

    // dp[i][j]: i 個目まで見終わって j 個選んだ場合のスコア分母最大値
    let mut dp = vec![vec![f64::MIN; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..n {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if j == 0 {
                dp[i + 1][1] = dp[i + 1][1].max(pn[i]);
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + pn[i] * zerop9[j]);
            }
        }
    }

    let mut ans = f64::MIN;
    let mut left_bunbo = 0.0;
    for i in 1..=n {
        left_bunbo += zerop9[i - 1];
        let r = dp[n][i] / left_bunbo - 1200.0 / (i as f64).sqrt();
        ans = ans.max(r);
    }

    println!("{ans}");
}
