use proconio::fastout;
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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [isize; n],
        bn: [isize; n],
        r3: [isize; 3],
    }

    // 1 つのラウンドだけを考えると, 頂点 s から各頂点へ容量 1, コスト -P_j の辺を貼って
    // 最小費用流問題と言い換えられる
    // ペナルティは輪の数の体数倍ではないので, n 個目の輪がかあｋたっときに発生するコストを
    // 分けて考える
    // 棒 j に i 個目の輪がかかったとき, 容量 1 でコスト Q_j,i の辺を張る

    // i ラウンド目の棒 j に対応する頂点を i*n+j
    // 棒 j のペナルティを計算する頂点を 3n+j
    // i ラウンド目の始点 S_i を 4n+i
    // グラフ全体の始点を 4n+3
    // グラフ全体の終点を 4n+4

    let v = 3 * (n + 1) + n + 2;
    let mut edges = vec![vec![]; v];

    let s = vec![n * 4, n * 4 + 1, n * 4 + 2];
    let x = n * 4 + 3;
    let y = n * 4 + 4;

    for i in 0..3 {
        for j in 0..n {
            let p_j = (an[j] * (bn[j].pow(i + 1))) % r3[i as usize];
            // ラウンド始点 -> 棒 j
            add_edge(s[i as usize], n * i as usize + j, 1, -p_j, &mut edges);
            // 棒 j -> ペナルティ
            add_edge(n * i as usize + j, n * 3 + j, 1, 0, &mut edges);
        }
    }

    for i in 0..3 {
        // グラフ全体の始点 -> 各ラウンドの始点
        add_edge(x, s[i], m as isize, 0, &mut edges);
    }

    for j in 0..n {
        // 棒 j のペナルティ -> 終点
        let q1 = an[j] * bn[j];
        let q2 = an[j] * (bn[j].pow(2)) - q1;
        let q3 = an[j] * (bn[j].pow(3)) - q1 - q2;
        add_edge(n * 3 + j, y, 1, q1, &mut edges);
        add_edge(n * 3 + j, y, 1, q2, &mut edges);
        add_edge(n * 3 + j, y, 1, q3, &mut edges);
    }

    let ans = -calc_min_cost_flow(x, y, m as isize * 3, &mut edges);
    println!("{ans}");
}
