// -> 6min

use proconio::input;

const DUMMY: usize = std::usize::MAX / 3;

fn main() {
    input! {
        h: usize,
        n: usize,
        abn: [(usize, usize); n],
    }

    // dp[i]: i ダメージを与えるのに必要な最小魔力
    let mut dp = vec![DUMMY; h + 1];
    dp[0] = 0;
    for ab in &abn {
        // 同じ魔法は何度でも使える
        for i in 0..h + 1 {
            let next_dmg = (i + ab.0).min(h);
            let next_mp = dp[i] + ab.1;
            dp[next_dmg] = dp[next_dmg].min(next_mp);
        }
    }

    println!("{}", dp.last().unwrap());
}
