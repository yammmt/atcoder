use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let a_sum = an.iter().sum::<usize>();

    // dp[i][j][k]: i 個目終了時に B_i の和が j で直前の B_(i-1) が k のときの線分長
    let mut dp = vec![vec![vec![f64::MAX / 2.0; a_sum + 1]; a_sum + 1]; n];
    dp[0][0][0] = 0.0;

    for i in 1..n {
        for j in 0..a_sum + 1 {
            for k in 0..a_sum + 1 {
                for l in 0..a_sum - j + 1 {
                    let j_l = j + l;
                    dp[i][j_l][l] = dp[i][j_l][l]
                        .min(dp[i - 1][j][k] + ((l as f64 - k as f64).powf(2.0) + 1.0).sqrt());
                }
            }
        }
    }

    println!("{}", dp[n - 1][a_sum][0]);
}
