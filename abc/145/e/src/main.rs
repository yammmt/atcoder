// 30min 1WA (26min)
// WA: ソート漏れ

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut t: usize,
        mut abn: [(usize, usize); n],
    }
    t *= 2;
    let t = t;
    abn.iter_mut().for_each(|ab| ab.0 *= 2);
    abn.sort_unstable();
    let abn = abn;

    // 最後の注文がなければ DP 部分和問題の典型
    // dp[i]: i 分での美味しさ最大
    let mut dp = vec![0; t + 6005];
    for ab in &abn {
        // println!("{:?}", ab);
        let mut next = dp.clone();
        for j in 0..t - 1 {
            let next_j = j + ab.0;

            next[next_j] = next[next_j].max(dp[j] + ab.1);
        }

        dp = next;
        // println!("{:?}", dp);
    }

    println!("{}", dp.iter().max().unwrap());
}
