// 発想が難しい

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

const DUMMY: usize = usize::MAX / 4;

// 始点 s から各頂点への最短距離を返す
fn bfs(s: usize, edges: &Vec<Vec<(usize, usize, usize)>>) -> Vec<usize> {
    let n = edges.len();
    let mut ret = vec![DUMMY; n];
    let mut que = VecDeque::new();
    let mut visited = vec![false; n];
    // (頂点, コスト)
    que.push_back((s, 0));
    while let Some((v_cur, cost_cur)) = que.pop_front() {
        if visited[v_cur] {
            continue;
        }

        visited[v_cur] = true;
        ret[v_cur] = cost_cur;
        for &(v_next, c_next, _) in &edges[v_cur] {
            if c_next == 0 {
                // 逆向きに貼った辺など
                continue;
            }

            que.push_back((v_next, cost_cur + 1));
        }
    }
    ret
}

// 頂点 v から頂点 t へ, f を上限としてフローを流す
// 戻り値は流した量
fn dfs(
    v: usize,
    t: usize,
    f: usize,
    removed: &mut Vec<usize>,
    dist: &Vec<usize>,
    edges: &mut Vec<Vec<(usize, usize, usize)>>,
) -> usize {
    if v == t {
        return f;
    }

    while removed[v] < edges[v].len() {
        // removed[v] は加算されるので, 先頭側の接続辺から順に処理するの意
        let (v_next, capacity, rev) = edges[v][removed[v]];
        // 辺の容量が残っており, 最短経路上にある辺にフローを流す
        if capacity > 0 && dist[v_next] != DUMMY && dist[v] < dist[v_next] {
            let flow = dfs(v_next, t, f.min(capacity), removed, dist, edges);

            if flow > 0 {
                edges[v][removed[v]].1 -= flow;

                edges[v_next][rev].1 += flow;
                return flow;
            }
        }

        // フローが流せなかった辺は削除
        removed[v] += 1;
    }

    0
}

// 頂点 s から頂点 t へ流せるフローの最大値を計算する
fn calc_max_flow(s: usize, t: usize, edges: &mut Vec<Vec<(usize, usize, usize)>>) -> usize {
    let n = edges.len();
    let mut flow = 0;
    loop {
        let dist = bfs(s, edges);

        if dist[t] == DUMMY {
            return flow;
        }

        let mut removed = vec![0; n];
        loop {
            // DFS で頂点 s から t への最短経路を見つけ, フローを流せるだけ流す
            // 初期流量は十分大きい数
            let f = dfs(s, t, DUMMY, &mut removed, &dist, edges);

            if f == 0 {
                break;
            }

            flow += f;
        }
    }
}

fn add_edge(v_in: usize, v_out: usize, edges: &mut Vec<Vec<(usize, usize, usize)>>) {
    let l = edges[v_out].len();
    edges[v_in].push((v_out, 1, l));
    let l = edges[v_in].len();
    edges[v_out].push((v_in, 0, l - 1));
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcdn: [(Usize1, Usize1, Usize1, Usize1); n],
    }

    // 始点から行を表す頂点へ, 容量 1 の辺を張る
    // 列を表す頂点から終点へ, 容量 1 の辺を張る
    // 各駒について,
    //   - 行を表す頂点から駒の入口を表す頂点へ辺を張る
    //   - 出口を表す頂点から列を表す頂点へ辺を張る
    //   - 入口を表す頂点から出口を表す頂点へ, 容量 1 の辺を張る

    // グラフの頂点数は H+W+2N+2

    // グラフの頂点と番号の対応:
    // - 行 r はそのまま (r)
    // - 列 c は H+c
    // - 駒 i の入口を H+W+i
    // - 駒 i の出口を H+W+N+i
    // - 始点を H+W+2N
    // - 終点を H+W+2N+1

    let p_start = h + w + 2 * n;
    let p_goal = p_start + 1;
    let num_of_vertices = h + w + 2 * n + 2;
    // (終点, 容量, 逆辺のインデックス)
    let mut edges = vec![vec![]; num_of_vertices];
    // 始点 -> 行
    for i in 0..h {
        add_edge(p_start, i, &mut edges);
    }
    // 列 -> 終点
    for i in 0..w {
        add_edge(h + i, p_goal, &mut edges);
    }
    for (i, &(a, b, c, d)) in abcdn.iter().enumerate() {
        let token_in = h + w + i;
        let token_out = h + w + n + i;
        // 行 -> 駒入口
        for j in a..=c {
            add_edge(j, token_in, &mut edges);
        }

        // 駒出口 -> 列
        for j in b..=d {
            add_edge(token_out, h + j, &mut edges);
        }

        // 駒入口 -> 駒出口
        add_edge(token_in, token_out, &mut edges);
    }

    println!("{}", calc_max_flow(p_start, p_goal, &mut edges));
}
