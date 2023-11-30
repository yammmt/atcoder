use proconio::fastout;
use proconio::input;

static DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        h: usize,
        n: usize,
        abn: [(usize, usize); n],
    }

    // dp[i]: ダメージ i を与えるに必要な魔力の最小値
    let mut dp = vec![DUMMY; h + 1];
    dp[0] = 0;
    for ab in abn {
        let a = ab.0;
        let b = ab.1;
        // 重複して使える
        for i in 0..=h {
            let cur_dmg = (i + a).min(h);
            dp[cur_dmg] = dp[cur_dmg].min(dp[i] + b);
        }
    }

    println!("{}", dp[h]);
}
