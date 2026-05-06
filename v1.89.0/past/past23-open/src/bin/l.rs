use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abn: [(isize, isize); n],
    }

    // 全列挙は 2^(10^5) となり TLE
    // 満足度の差が 201 通りしか取り得ないので, O(N) でこれを回せばよさそう, として
    // dp[i][j]: i 番目の皿を片付けた時点で高橋君の満足度が j-200 高い状態での高橋君の最大満足度
    // 200 は m と言い換えてもよい, ワーストケース以外の実行速度にはほぼ影響なさそうだがどっちが実装楽か

    let mut dp = vec![vec![None; 2 * m + 1]; n + 1];
    dp[0][m] = Some(0);
    // 配る DP
    for i in 0..n {
        for j in 0..2 * m + 1 {
            let Some(score_before) = dp[i][j] else {
                continue;
            };

            // takahashi
            let j_next_tkhs = j + abn[i].0 as usize;
            if j_next_tkhs <= 2 * m {
                let score_cur = dp[i + 1][j_next_tkhs].unwrap_or(-1);
                let score_after = score_before + abn[i].0;
                dp[i + 1][j_next_tkhs] = Some(score_cur.max(score_after));
            }

            // aoki
            let j_next_aoki = j.wrapping_add_signed(-abn[i].1);
            if j_next_aoki <= 2 * m {
                let score_cur = dp[i + 1][j_next_aoki].unwrap_or(-1);
                dp[i + 1][j_next_aoki] = Some(score_cur.max(score_before));
            }
        }
        // for j in 0..2 * m + 1 {
        //     println!("{}: {:?}", j as isize - m as isize, dp[i + 1][j]);
        // }
        // println!();
    }

    // エッジケース: 高橋君が一食もできない場合
    let mut ans = -1;
    for i in 0..2 * m + 1 {
        if let Some(c) = dp[n][i] {
            ans = ans.max(c)
        }
    }

    println!("{}", if ans == -1 { -1 } else { ans as isize });
}
