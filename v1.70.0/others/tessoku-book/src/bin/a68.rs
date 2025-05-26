use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

const DUMMY: usize = usize::MAX / 4;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    capacity: usize,
    rev: usize,
}

// 始点 s から各頂点への最短距離を返す
fn bfs(s: usize, edges: &Vec<Vec<Edge>>) -> Vec<usize> {
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
        for e in &edges[v_cur] {
            let v_next = e.to;
            let c_next = e.capacity;
            if c_next == 0 {
                // 最初に逆向きに貼った辺や最大限流し終わった辺
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
    edges: &mut Vec<Vec<Edge>>,
) -> usize {
    if v == t {
        return f;
    }

    while removed[v] < edges[v].len() {
        // removed[v] は加算されるので, 先頭側の接続辺から順に処理するの意
        let v_next = edges[v][removed[v]].to;
        let capacity = edges[v][removed[v]].capacity;
        let rev = edges[v][removed[v]].rev;
        // 辺の容量が残っており, 最短経路上にある辺にフローを流す
        // 最短？増加路としか判定していないが, 制約上必ず最短になるはず
        if capacity > 0 && dist[v_next] != DUMMY && dist[v] < dist[v_next] {
            let flow = dfs(v_next, t, f.min(capacity), removed, dist, edges);

            if flow > 0 {
                edges[v][removed[v]].capacity -= flow;

                edges[v_next][rev].capacity += flow;
                return flow;
            }
        }

        // フローが流せなかった辺は削除
        removed[v] += 1;
    }

    0
}

// 頂点 s から頂点 t へ流せるフローの最大値を計算する
fn calc_max_flow(s: usize, t: usize, edges: &mut Vec<Vec<Edge>>) -> usize {
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

fn add_edge(v_in: usize, v_out: usize, capacity: usize, edges: &mut Vec<Vec<Edge>>) {
    // edges: (終点, 容量, 費用, 逆辺のインデックス)
    let l = edges[v_out].len();
    edges[v_in].push(Edge {
        to: v_out,
        capacity,
        rev: l,
    });
    let l = edges[v_in].len();
    edges[v_out].push(Edge {
        to: v_in,
        capacity: 0,
        rev: l - 1,
    });
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(Usize1, Usize1, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for (a, b, c) in abcm {
        add_edge(a, b, c, &mut edges);
    }

    let ans = calc_max_flow(0, n - 1, &mut edges);
    println!("{ans}");
}
