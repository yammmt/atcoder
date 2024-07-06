use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 4;

fn dijkstra(p_start: usize, edges: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let n = edges.len();
    let mut ret = vec![DUMMY; n];
    let mut bh = BinaryHeap::new();
    // (cost, vertex)
    bh.push((Reverse(0), p_start));

    while let Some((Reverse(cost), v)) = bh.pop() {
        if ret[v] != DUMMY {
            continue;
        }

        ret[v] = cost;

        for &(v_next, w) in &edges[v] {
            // なくともよい
            if ret[v_next] != DUMMY {
                continue;
            }

            bh.push((Reverse(cost + w), v_next));
        }
    }

    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uvwm: [(Usize1, Usize1, usize); m],
    }

    let mut edges = vec![vec![]; n];
    let mut redges = vec![vec![]; n];
    for (u, v, w) in uvwm {
        edges[u].push((v, w));
        redges[v].push((u, w));
    }

    // 同じ辺を二度通ってもよいので, 単純に 1->k 最短に k->n 最短を足す？
    // k が n 種類出てくるのでやるだけじゃ無理
    // 1 始点の Dijkstra をなんか工夫して一度やる, でいけないか
    //   => 最短経路に通った点を乗せると O(N^2) になりそうで違いそう
    // 辺を逆に読み替えて n 始点のものを求めれば 1-k + k-n できそう

    let from_1 = dijkstra(0, &edges);
    let from_n = dijkstra(n - 1, &redges);

    for i in 0..n {
        let ans = from_1[i] + from_n[i];
        if ans >= DUMMY {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
}
