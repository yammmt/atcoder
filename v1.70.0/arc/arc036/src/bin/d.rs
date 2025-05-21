use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        wxyzq: [(usize, Usize1, Usize1, usize); q],
    }

    let mut uf = UnionFind::new(2 * n);
    for (w, x, y, z) in wxyzq {
        if w == 1 {
            if z % 2 == 0 {
                uf.union(x, y);
                uf.union(x + n, y + n);
            } else {
                uf.union(x, y + n);
                uf.union(y, x + n);
            }
        } else {
            println!(
                "{}",
                if uf.equiv(x, y) {
                    "YES"
                } else {
                    "NO"
                }
            );
        }
    }
}
