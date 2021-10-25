// :fu: :fu: 21-10 解きたいが
// TLE: BFS で到達判定を忘れていた
// WA: A_M に同じ頂点が連続する場合を見落としていて泥沼

use proconio::input;
use std::collections::VecDeque;

const OFFSET: usize = 100_000;
const DP_LEN: usize = 2 * OFFSET + 1;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        am: [usize; m],
        uvn1: [(usize, usize); n - 1],
    }
    let d = 998_244_353;

    let mut edges = vec![vec![]; n];
    for uv in &uvn1 {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }
    let edges = edges;

    // 探索起点の頂点が高々 100 個であり, これなら数列 A 中の全頂点を基準に最短経路を求めても TLE しない
    // 木が与えられるため最短経路も一通りに限定されるし, どの辺を何度通過するかも求められる

    // passed_num[i][j]: (i, j) を結ぶ辺の通過回数 (i < j)
    // 木故に存在しない辺が殆どであり座圧した方が高速だが 1,000^2 なら間に合う
    let mut passed = vec![vec![0; n]; n];
    for (i, a) in am.iter().enumerate().take(m - 1) {
        // a -> am[i + 1] (どちらも 0-indexed に揃える)
        let mut vdq: VecDeque<(usize, Vec<(usize, usize)>)> = VecDeque::new();
        let mut visited = vec![false; n];
        // (現在地, 経路)
        vdq.push_back((*a - 1, vec![]));
        visited[*a - 1] = true;
        'outer: while let Some(cur) = vdq.pop_front() {
            if cur.0 == am[i + 1] - 1 {
                // goal
                // これまでの通過点
                for p in &cur.1 {
                    passed[p.0][p.1] += 1;
                }
                break 'outer;
            }

            for e in &edges[cur.0] {
                if !visited[*e] {
                    let mut cur_path = cur.1.clone();
                    // (i, j): i < j に揃える
                    cur_path.push((cur.0.min(*e), cur.0.max(*e)));
                    vdq.push_back((*e, cur_path));
                    visited[*e] = true;
                }
            }
        }
    }

    // dp[i]: R - B = i - NM となる選び方の総数
    // 辺通過回数の合計を取って部分和 DP でも可
    // 辺数が最大で 999 個, 辺通過回数が最大で 100x999 = 99,900 回
    let mut dp = vec![0; DP_LEN];
    dp[OFFSET] = 1;
    for uv in &uvn1 {
        let i = (uv.0 - 1).min(uv.1 - 1);
        let j = (uv.0 - 1).max(uv.1 - 1);
        let mut cur = vec![0; DP_LEN];
        for kk in 0..DP_LEN {
            if dp[kk] == 0 {
                continue;
            }

            let next_score_r = kk + passed[i][j];
            if next_score_r < DP_LEN {
                cur[next_score_r] = (cur[next_score_r] + dp[kk]) % d;
            }
            if kk >= passed[i][j] {
                let next_score_b = kk - passed[i][j];
                cur[next_score_b] = (cur[next_score_b] + dp[kk]) % d;
            }
        }
        dp = cur;
    }

    println!("{}", (dp[(k + (OFFSET as isize)) as usize]));
}
