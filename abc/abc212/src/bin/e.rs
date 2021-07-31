// :fu: 21-07 わからん ここまでまったくわからん難易度帯か？

use proconio::input;
use std::collections::HashSet;
// use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvm: [(usize, usize); m],
    }
    let d = 998_244_353;
    // 道が "使えなくなった"
    // M <= 5,000 の時点でこれを活用しないと TLE する
    // 削除を考えた余事象？
    let mut deleted_edges = vec![HashSet::new(); n];
    for uv in &uvm {
        deleted_edges[uv.0 - 1].insert(uv.1 - 1);
        deleted_edges[uv.1 - 1].insert(uv.0 - 1);
    }

    // 都市 1 から見て i 日目に辿り着ける街の数を両端から？
    // 1 -> 2 -> 1 -> 1 とか弾ける？
    // それはそうでも定数倍削減に過ぎない気がする
    // 次に選べるマスは 1 からの距離の偶奇に依存？だが閉路があるので違う
    // 愚直にそのマスにいるのが何通りか数えてみよう -> O(N^3) TLE
    // 一つのマスに対して到達可能かを O(1) で判断できれば O(N^2) の DP にできる

    // i + 1 日目には i - 1 日目にいたマスに必ず行ける -> 何通りか数えられない
    // i 日目に都市 j にいる, では重複を生んで WA
    // 削除された文を余事象として引く？
    // 最短経路がわかれば i 日目に j にいるパターンをすべて切ることはできるがそれでは足りず
    // 辺削除がなければ (N - 1) を掛けてなんたら
    // let mut dp = vec![0; n];
    // dp[0] = 1;
    // for _ in 0..n {
    //     for i in 0..n {
    //         if dp[i] == 0 {
    //             continue;
    //         }
    //     }
    // }
    // println!("{}", dp[0]);
}
