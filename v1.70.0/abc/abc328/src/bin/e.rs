// 順列全列挙より bitDP の方が同じ思想で高速そう
// https://atcoder.jp/contests/abc328/editorial/7671

use itertools::Itertools;
use proconio::fastout;
use proconio::input;
// use std::collections::HashSet;

const DUMMY: usize = usize::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvwm: [(usize, usize, usize); m],
    }
    let mut edges = vec![vec![DUMMY; n]; n];
    for uvw in uvwm {
        let u = uvw.0 - 1;
        let v = uvw.1 - 1;
        let w = uvw.2;
        edges[u][v] = w;
        edges[v][u] = w;
    }

    // 頂点数が少ない
    // 使う辺の数は n - 1 点で固定, つまり最大で 7 辺だがすべての辺から 7 つ選ぶと TLE
    // 例 1 のように全域木は一筆書きできるとも限らない
    // 訪問順を決め打ちして, 取りうるコストをすべて DP みたいにもつ
    // 重複省くと定数倍の改善にはなりそうだけど
    //     のはずが HashSet 経由だと圧倒的に TLE する (例 3 で 10s 超)

    // でも最小全域木とはいわれていない気がするのだが
    let mut ans = DUMMY;
    'outer_loop: for perm in (0..n).permutations(n) {
        // let mut costs = HashSet::from([0]);
        let mut costs = vec![0];
        let mut visited = vec![false; n];
        visited[perm[0]] = true;
        for i_to in 1..n {
            // let mut costs_nxt = HashSet::new();
            let mut costs_nxt = vec![];
            for j in 0..n {
                if visited[j] && edges[perm[i_to]][j] != DUMMY {
                    for c in &costs {
                        // costs_nxt.insert((c + edges[perm[i_to]][j]) % k);
                        costs_nxt.push((c + edges[perm[i_to]][j]) % k);
                    }
                }
            }

            if costs_nxt.is_empty() {
                continue 'outer_loop;
            }

            visited[perm[i_to]] = true;
            costs = costs_nxt;
        }

        costs.iter().for_each(|&c| ans = ans.min(c));
    }

    println!("{ans}");
}
