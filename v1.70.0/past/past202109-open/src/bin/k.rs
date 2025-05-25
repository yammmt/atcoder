// 二部マッチングに見える
// この回の PAST が要求する知識量異様でないか

use proconio::input;
use proconio::marker::Chars;

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
        p: usize,
        q: usize,
        spq: [Chars; p],
        abp: [(isize, isize); p],
        cdq: [(isize, isize); q],
    }

    // オスの頂点を [0, p-1]
    // メスの頂点を [p, p+q-1]
    // 始点を p+q
    // 終点を p+q+1

    let v = p + q + 2;
    let x = p + q;
    let y = p + q + 1;
    let mut edges = vec![vec![]; v];

    for i in 0..p {
        // 始点からオス
        add_edge(x, i, 1, 0, &mut edges);
        // オスから終点
        add_edge(i, y, 1, -abp[i].1, &mut edges);
    }

    // オスからメス
    for i in 0..p {
        for j in 0..q {
            if spq[i][j] == '1' {
                add_edge(i, p + j, 1, -abp[i].0, &mut edges);
            }
        }
    }

    for i in 0..q {
        add_edge(p + i, y, 1, -cdq[i].0 + cdq[i].1, &mut edges);
    }

    let dsum = cdq.iter().map(|&cd| cd.1).sum::<isize>();
    let ans = dsum - calc_min_cost_flow(x, y, p as isize, &mut edges);
    println!("{ans}");
}
