use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    abm.iter().for_each(|ab| {
        uf.union(ab.0 - 1, ab.1 - 1);
    });

    let mut membernum = vec![0; n];
    (0..n).for_each(|i| membernum[uf.find(i)] += 1);

    println!("{}", membernum.iter().max().unwrap());
}
