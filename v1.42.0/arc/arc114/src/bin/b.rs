use petgraph::unionfind::UnionFind;
use proconio::input;
use std::collections::HashSet;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        f_n: [usize; n],
    }
    let d = 998_244_353;

    let mut uf = UnionFind::new(n + 1);
    for i in 0..n {
        uf.union(i + 1, f_n[i]);
    }

    let mut grpset = HashSet::new();
    for i in 1..n + 1 {
        grpset.insert(uf.find(i));
    }

    let mut ans = 1;
    for _ in 0..grpset.len() {
        ans = (2 * ans) % d;
    }
    ans = (ans + d - 1) % d;
    println!("{}", ans);
}
