// 集合を状態に持つ DP

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const MOD: usize = 1_000_000_007;
    input! {
        n: usize,
        ann: [[usize; n]; n],
    }

    // dp[S]: 男性 0, 1, ..., i-1 に対して, 割り当てられた女性の番号の集合が整数値 S と
    //        表されるような割り当て方の個数
    let mut dp = vec![0; 1 << n];
    dp[0] = 1;

    for s in 0..((1 << n) - 1) {
        // i: マッチング対象となる男性 (固定)
        // マッチング成立数は bit の 1 の数と等しい
        // i = s.popcount()
        let mut i = 0;
        let mut ss = s;
        while ss > 0 {
            if ss % 2 == 1 {
                i += 1;
            }
            ss /= 2;
        }
        let i = i;

        // 女性側を j として全探索
        for j in 0..n {
            // (i, j) がマッチング可能かつ j が未選択
            if ann[i][j] == 1 && (s & (1 << j)) == 0 {
                let ss = s | (1 << j);
                dp[ss] += dp[s];
                dp[ss] %= MOD;
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp.last().unwrap());
}
