// index がややこしい, 嫌い

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const DUMMY_BIG: usize = usize::MAX / 3;

    input! {
        n: usize,
        m: usize,
        k: usize,
        an: [usize; n],
    }

    // cost[i][j]: i を始点に j 個連続して詰めた際のコスト
    let mut cost = vec![vec![DUMMY_BIG; m + 1]; n];
    for i in 0..n {
        let mut size_min = DUMMY_BIG;
        let mut size_max = 0;
        // j 個選ぶ (1-origin)
        for j in 1..m + 1 {
            if i + j - 1 >= n {
                break;
            }

            size_min = size_min.min(an[i + j - 1]);
            size_max = size_max.max(an[i + j - 1]);
            cost[i][j] = k + j * (size_max - size_min);
        }
    }
    // println!("cost: {:?}", cost);

    // dp[i]: 先頭 i 個を詰める最小コスト
    let mut dp = vec![DUMMY_BIG; n + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=m {
            // 配る DP
            if i + j > n {
                continue;
            }

            dp[i + j] = dp[i + j].min(dp[i] + cost[i][j]);
        }
        // println!("dp: {:?}", dp);
    }

    println!("{}", dp.last().unwrap());
}
