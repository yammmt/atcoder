// TLE ハマった
// O(GEE), 高々 100x100 辺のグラフに 100 回 DFS 回すのだから
// これで間に合わない理由がよくわからない, 無限ループ入ってる？ => はい...

use proconio::fastout;
use proconio::input;

fn dfs(
    v_cur: usize,
    v_goal: usize,
    visited: &mut Vec<bool>,
    visitable: &Vec<Vec<usize>>,
    edge: &mut Vec<Vec<isize>>,
) -> isize {
    visited[v_cur] = true;
    for &v_nxt in &visitable[v_cur] {
        if visited[v_nxt] || edge[v_cur][v_nxt] == 0 {
            continue;
        }

        if v_nxt == v_goal || dfs(v_nxt, v_goal, visited, visitable, edge) > 0 {
            edge[v_cur][v_nxt] -= 1;
            edge[v_nxt][v_cur] += 1;
            return 1;
        }
    }

    0
}

#[fastout]
fn main() {
    input! {
        n: usize,
        g: usize,
        e: usize,
        pg: [usize; g],
        abe: [(usize, usize); e],
    }
    // (頂点, 残流量)
    let mut edge = vec![vec![0isize; n + 1]; n + 1];
    for (a, b) in abe {
        edge[a][b] += 1;
        edge[b][a] += 1;
    }
    for p in pg {
        edge[p][n] += 1;
    }
    let mut visitable = vec![vec![]; n + 1];
    for i in 0..=n {
        for j in 0..=n {
            if edge[i][j] > 0 {
                visitable[i].push(j);
            }
        }
    }

    // 最小カットの練習問題

    // 残余グラフ上で s-t パスを見つける
    // Ford-Fulkerson 法 (フォード・ファルカーソン) で高々流量分回の DFS をする
    // 計算量は O(GEE) となり十分高速

    let mut ans = 0;
    loop {
        let mut visited = vec![false; n + 1];
        let cost_used = dfs(0, n, &mut visited, &visitable, &mut edge);
        if cost_used == 0 {
            break;
        }

        ans += cost_used;
    }

    println!("{ans}");
}
