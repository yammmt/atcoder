use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n],
    }

    let mut abmax = 0;
    abn.iter().for_each(|ab| abmax = abmax.max(ab.0.max(ab.1)));
    let mut uf = UnionFind::new(abmax + 1);

    abn.iter().for_each(|ab| {
        uf.union(ab.0, ab.1);
    });

    let mut membernum = vec![0; abmax + 1];
    (1..abmax + 1).for_each(|i| membernum[uf.find(i)] += 1);

    let mut cardnum = vec![0; abmax + 1];
    abn.iter().for_each(|ab| cardnum[uf.find(ab.0)] += 1);

    let mut ans = 0;
    (0..abmax + 1).for_each(|i| ans += membernum[i].min(cardnum[i]));
    println!("{}", ans);
}
