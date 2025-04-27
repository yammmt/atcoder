use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const DUMMY_BIG: usize = usize::MAX / 2;

    input! {
        n: usize,
        ann: [[usize; n]; n],
    }

    // dp[i][j]: 訪問済み i, 現在位置 j に至るまでの最小コスト
    let mut dp = vec![vec![DUMMY_BIG; n]; 1 << n];
    dp[1][0] = 0;

    // s=0 だとどこも訪問済みでないのでなにもできぬ
    for s in 1..(1 << n) {
        for i in 0..n {
            if s & (1 << i) == 0 {
                // 都市 i は未訪問
                continue;
            }

            // 配る DP
            // 初期地点である j=0 は見なくともよい
            for j in 1..n {
                // j が訪問済みの場合に抜ける
                // これを抜くと "ちょうど 1 回ずつ" に反する
                if s & (1 << j) > 0 {
                    continue;
                }

                let s_new = s | (1 << j);
                dp[s_new][j] = dp[s_new][j].min(dp[s][i] + ann[i][j]);
            }
        }
    }

    // 都市 0 に戻る分
    let mut ans = DUMMY_BIG;
    for i in 1..n {
        ans = ans.min(dp[(1 << n) - 1][i] + ann[i][0]);
    }

    println!("{ans}");
}
