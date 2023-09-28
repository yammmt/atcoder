// PAST (2021-04) で見たような DP
// 18.5min

use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        rcvk: [(usize, usize, usize); k],
    }

    // アイテムのあるマスに重複はない
    let mut item_value = vec![vec![0; c]; r];
    for rcv in &rcvk {
        item_value[rcv.0 - 1][rcv.1 - 1] = rcv.2;
    }

    // dp[i][j][k]: マス (i, j) でその行内でアイテムを k 個 拾った場合の最大価値
    let mut dp = vec![vec![vec![0; 4]; c]; r];
    for i in 0..r {
        for j in 0..c {
            // そのマスでアイテムを拾う
            // 更新順に注意 さもなくば同じアイテムを拾い続ける
            let cur_1 = dp[i][j][1].max(dp[i][j][0] + item_value[i][j]);
            let cur_2 = dp[i][j][2].max(dp[i][j][1] + item_value[i][j]);
            let cur_3 = dp[i][j][3].max(dp[i][j][2] + item_value[i][j]);
            dp[i][j][1] = cur_1;
            dp[i][j][2] = cur_2;
            dp[i][j][3] = cur_3;

            if i < r - 1 {
                // 行超えるとアイテム数はリセット
                dp[i + 1][j][0] = *dp[i][j].iter().max().unwrap();
            }
            if j < c - 1 {
                dp[i][j + 1][1] = dp[i][j][1];
                dp[i][j + 1][2] = dp[i][j][2];
                dp[i][j + 1][3] = dp[i][j][3];
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[r - 1][c - 1].iter().max().unwrap());
}
