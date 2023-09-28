// petgraph クレートの UnionFind では要素数は直接は管理されておらず

use petgraph::unionfind::UnionFind;
use proconio::input;

fn perm(n: u64, d: u64) -> u64 {
    let mut ret = 1;
    for i in 1..n + 1 {
        ret = (ret * i) % d;
    }
    ret
}

fn main() {
    input! {
        n: usize,
        k: u64,
        ann: [[u64; n]; n],
    }
    let d = 998_244_353;
    let mut ans = 1;

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if (0..n).all(|kk| ann[i][kk] + ann[j][kk] <= k) {
                uf.union(i, j);
            }
        }
    }
    let mut sizes = vec![0; n];
    (0..n).for_each(|i| sizes[uf.find(i)] += 1);
    sizes.iter().for_each(|s| ans = (ans * perm(*s, d)) % d);

    let mut uf = UnionFind::new(n);
    for i in 0..n {
        for j in i + 1..n {
            if (0..n).all(|kk| ann[kk][i] + ann[kk][j] <= k) {
                uf.union(i, j);
            }
        }
    }
    let mut sizes = vec![0; n];
    (0..n).for_each(|i| sizes[uf.find(i)] += 1);
    sizes.iter().for_each(|s| ans = (ans * perm(*s, d)) % d);

    println!("{}", ans);
}
