// 制約より x < y でありトポロジカルソートするまでもなかった

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [i64; n],
        mut xym: [(usize, usize); m],
    }

    xym.sort_unstable();
    let mut dp = vec![std::i64::MAX / 2; n];
    for xy in &xym {
        dp[xy.1 - 1] = dp[xy.1 - 1].min(an[xy.0 - 1].min(dp[xy.0 - 1]));
    }
    // println!("{:?}", dp);

    let mut ans = std::i64::MIN;
    for i in 0..n {
        ans = ans.max(an[i] - dp[i]);
    }

    println!("{}", ans);
}
