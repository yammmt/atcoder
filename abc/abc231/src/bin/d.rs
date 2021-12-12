use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    // No となる条件: 一人に対して隣り合う人が三名以上 or ループが発生する
    // 両端の人が定義できない (横指定が一名以下の人が二名未満) 場合は必ずループが発生する？
    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    if edges.iter().any(|e| e.len() > 2) {
        println!("No");
        return;
    }

    let mut uf = UnionFind::new(n);
    for ab in &abm {
        let a = ab.0 - 1;
        let b = ab.1 - 1;
        let a_grp = uf.find(a);
        let b_grp = uf.find(b);
        if a_grp == b_grp {
            println!("No");
            return;
        }

        uf.union(a, b);
    }

    println!("Yes");
}
