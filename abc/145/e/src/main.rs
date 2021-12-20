// :fu: :fu: 21-12
// 30min 1WA (26min) -> x (15min)

// https://ikatakos.com/pot/programming_algorithm/contest_history/atcoder/2019/1116_abc145
// > 時間のかかる料理を先に処理してしまうと、タイムリミットまでに他の料理をさらに食べられるのに、
// > 最適化されないまま満足度を確定させてしまう。
// 3 10
// 100 9
// 2 2
// 2 2

use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        mut abn: [(usize, usize); n],
    }
    abn.sort_unstable();

    // dp[i]: i 分経過時点での満足度最大 (t + 1 超過時は全部 t に落とす)
    let mut dp = vec![0; t + 1];
    // 食すのに T - 1 分かかる料理を二つ頼む場合を考えてもソートなしでいけるはず
    for ab in &abn {
        let mut cur = dp.clone();
        for i in 0..t {
            if i != 0 && dp[i] == 0 {
                // 省かなくとも変わらないはず
                continue;
            }

            let next_i = (i + ab.0).min(t);
            cur[next_i] = cur[next_i].max(dp[i] + ab.1);
        }
        dp = cur;
    }

    println!("{}", dp.iter().max().unwrap());
}
