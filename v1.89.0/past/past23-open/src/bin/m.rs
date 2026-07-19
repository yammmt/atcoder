use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 2;

fn shortest_paths(v_begin: usize, edges: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut ret = vec![DUMMY; edges.len()];
    let mut bh = BinaryHeap::new();
    bh.push(Reverse((0, v_begin)));
    while let Some(Reverse((cost_cur, v_cur))) = bh.pop() {
        if ret[v_cur] != DUMMY {
            continue;
        }

        ret[v_cur] = cost_cur;
        for &(v_next, c) in &edges[v_cur] {
            if ret[v_next] == DUMMY {
                bh.push(Reverse((cost_cur + c, v_next)));
            }
        }
    }

    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: f64,
        xycm: [(Usize1, Usize1, usize); m],
        ak: [Usize1; k],
    }

    let mut edges = vec![vec![]; n];
    for (x, y, c) in xycm {
        edges[x].push((y, c));
        edges[y].push((x, c));
    }

    let shortest_paths_from_1 = shortest_paths(0, &edges);
    let shortest_paths_from_n = shortest_paths(n - 1, &edges);
    // println!("{shortest_paths_from_1:?}");

    let mut ans = shortest_paths_from_1[n - 1] as f64;
    for a in ak {
        // トレーニング回数を X-1 回としたときの合計所要時間が
        // ジム街までの時間 + T(X-1) + (残り距離/X)
        // この式が最小値を取る X が sqrt(残り距離/t)
        let sq = (shortest_paths_from_n[a] as f64 / t).sqrt();
        if sq.floor() > 0.0 {
            ans = ans.min(
                shortest_paths_from_1[a] as f64
                    + (sq.floor() - 1.0) * t
                    + shortest_paths_from_n[a] as f64 / sq.floor(),
                );
        }
        if sq.ceil() > 1.0 {
            ans = ans.min(
                shortest_paths_from_1[a] as f64
                    + (sq.ceil() - 1.0) * t
                    + shortest_paths_from_n[a] as f64 / sq.ceil(),
            );
        }
    }

    println!("{ans}");
}
