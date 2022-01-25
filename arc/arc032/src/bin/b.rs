// < 10min x2
// UnionFind やるだけ

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }
    if m == 0 {
        println!("{}", n - 1);
        return;
    }

    let mut uf = UnionFind::new(n);
    abm.iter().for_each(|ab| {
        uf.union(ab.0 - 1, ab.1 - 1);
    });
    let mut grpnum = vec![];
    (0..n).for_each(|a| grpnum.push(uf.find(a)));
    grpnum.sort_unstable();
    grpnum.dedup();

    println!("{}", grpnum.len() - 1);
}
