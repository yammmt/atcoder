// :fu: :fu: 21-04 何故か DP 滅茶苦茶苦手

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        mut abn: [(usize, usize); n],
    }

    abn.sort_unstable_by(|x, y| {
        (x.0 * y.1).cmp(&(x.1 * y.0))
    });
    abn.reverse();

    // dp[i]: 体力が i のときの特典の最大値
    let mut dp = vec![0; h + 1];
    for ab in &abn {
        // 体力 i の状態から体力 i - ab.1 の状態に配る DP
        for i in 1..h + 1 {
            let idx = (0.max(i as i64 - ab.1 as i64)) as usize;
            dp[idx] = dp[idx].max(dp[i] + ab.0 * i);
        }
    }

    println!("{}", dp.iter().max().unwrap());
}
