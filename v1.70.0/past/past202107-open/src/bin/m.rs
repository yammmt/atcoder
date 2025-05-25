// わからん

use proconio::input;

const INF: isize = isize::MAX;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    capacity: isize,
    cost: isize,
    rev: usize,
}

fn bellman_ford(s: usize, edges: &mut Vec<Vec<Edge>>) -> (Vec<isize>, Vec<usize>, Vec<usize>) {
    let n = edges.len();
    let mut dist = vec![INF; n];
    dist[s] = 0;

    // 頂点 v への距離を更新したときに使った辺の出ている頂点
    let mut pv = vec![0; n];
    // 頂点 v への距離を更新したときに使った辺が, pv[v] の何番目の辺であるか
    let mut pe = vec![0; n];
    loop {
        let mut updated = false;

        for v in 0..n {
            if dist[v] == INF {
                continue;
            }

            for i in 0..edges[v].len() {
                let v_next = edges[v][i].to;
                let capacity = edges[v][i].capacity;
                let cost = edges[v][i].cost;

                // 容量 0 の辺は通れない
                if capacity > 0 && dist[v_next] > dist[v] + cost {
                    dist[v_next] = dist[v] + cost;
                    updated = true;

                    pv[v_next] = v;
                    pe[v_next] = i;
                }
            }
        }

        if !updated {
            break;
        }
    }

    (dist, pv, pe)
}

fn calc_min_cost_flow(s: usize, t: usize, mut f: isize, edges: &mut Vec<Vec<Edge>>) -> isize {
    let mut ret = 0;
    while f > 0 {
        let (dist, pv, pe) = bellman_ford(s, edges);

        if dist[t] == INF {
            return INF;
        }

        // 頂点 t から最短経路を逆向きに頂点 s へと辿り, その経路に流せるフローを求める
        let mut flow = f;
        let mut v = t;
        while v != s {
            flow = flow.min(edges[pv[v]][pe[v]].capacity);
            v = pv[v];
        }

        // フロー flow をコスト dist[t] の経路に流した
        ret += flow * dist[t];
        f -= flow;

        // s -> t にフローを flow だけ流したので, 経路上の各辺の容量を調整する
        v = t;
        while v != s {
            edges[pv[v]][pe[v]].capacity -= flow;
            let rev = edges[pv[v]][pe[v]].rev as usize;
            edges[v][rev].capacity += flow;
            v = pv[v];
        }
    }
    ret
}

fn add_edge(v_in: usize, v_out: usize, capacity: isize, cost: isize, edges: &mut Vec<Vec<Edge>>) {
    // edges: (終点, 容量, 費用, 逆辺のインデックス)
    let l = edges[v_out].len();
    edges[v_in].push(Edge {
        to: v_out,
        capacity,
        cost,
        rev: l,
    });
    let l = edges[v_in].len();
    edges[v_out].push(Edge {
        to: v_in,
        capacity: 0,
        cost: -cost,
        rev: l - 1,
    });
}

fn main() {
    input! {
        n: usize,
        c: isize,
        an: [isize; n],
    }

    // 追加すると前要素との絶対値分スコア, 追加しないと c だけスコア
    // 始点から頂点に容量 1, コスト 0
    // 頂点 i から頂点 n+j に容量 1, コスト絶対値
    // 頂点 i から終点に容量 1, コスト c
    // 頂点 n+j から主点に容量 1, コスト 0

    let v = n * 2 + 2;
    let s = n * 2;
    let t = n * 2 + 1;
    let mut edges = vec![vec![]; v];

    for i in 0..n {
        add_edge(s, i, 1, 0, &mut edges);
        add_edge(i, t, 1, c, &mut edges);
    }

    for i in 0..n {
        for j in i + 1..n {
            add_edge(i, n + j, 1, (an[j] - an[i]).abs(), &mut edges);
        }
    }

    for j in 0..n {
        add_edge(n + j, t, 1, 0, &mut edges);
    }

    println!("{}", calc_min_cost_flow(s, t, n as isize, &mut edges));
}
