use proconio::input;
use std::cmp::Reverse;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        utvm: [(usize, usize, usize); m],
    }

    // 移動時間最短にしつつ満足度最大
    // 都市が多く bitDP 全探索は無理
    // 同じ都市を複数回通るメリットはない, 各頂点への最短経路とその満足度だけもってればよい
    // つまりはただの Dijkstra では

    let mut edges = vec![vec![]; n];
    for utv in utvm {
        let u = utv.0 - 1;
        let v = utv.1 - 1;
        let t = utv.2;
        // (到着先, 時間)
        edges[u].push((v, t));
        edges[v].push((u, t));
    }

    let mut visited = vec![false; n];
    // 都市 i 到着時の最大満足度
    let mut points = vec![0; n];
    let mut bts = BTreeSet::new();
    for e in &edges[0] {
        // 到着後の (移動時間総和, 満足度総和, 到着先)
        bts.insert((e.1, Reverse(an[0] + an[e.0]), e.0));
    }
    while let Some(cur) = bts.pop_first() {
        if visited[cur.2] {
            continue;
        }

        visited[cur.2] = true;
        // TODO: よい書き方？
        let Reverse(p) = cur.1;
        points[cur.2] = p;

        for e in &edges[cur.2] {
            if !visited[e.0] {
                bts.insert((cur.0 + e.1, Reverse(p + an[e.0]), e.0));
            }
        }
    }

    println!("{}", points[n - 1]);
}
