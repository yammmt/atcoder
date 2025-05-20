use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pqk: [(Usize1, Usize1); k],
        rsl: [(Usize1, Usize1); l],
    }

    let mut uf_road = UnionFind::new(n);
    for (p, q) in pqk {
        uf_road.union(p, q);
    }

    let mut uf_rail = UnionFind::new(n);
    for (r, s) in rsl {
        uf_rail.union(r, s);
    }

    let mut hm = HashMap::new();
    for i in 0..n {
        let cnt = hm.entry((uf_road.find(i), uf_rail.find(i))).or_insert(0);
        *cnt += 1;
    }

    for i in 0..n {
        print!("{}", hm.get(&(uf_road.find(i), uf_rail.find(i))).unwrap());
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
