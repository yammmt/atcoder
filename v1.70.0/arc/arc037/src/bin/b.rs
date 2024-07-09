use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(Usize1, Usize1); m],
    }

    // DFS で遷移元の頂点でない既出頂点を拾ったら終わり, でも解けそう

    let mut uf = UnionFind::new(n);
    let mut bad_vertexes = vec![];

    for (u, v) in uvm {
        if uf.equiv(u, v) {
            bad_vertexes.push(u);
        }

        uf.union(u, v);
    }

    let mut is_bad_group = vec![false; n];
    for v in bad_vertexes {
        is_bad_group[uf.find(v)] = true;
    }

    let mut is_good_group = vec![false; n];
    for i in 0..n {
        let g = uf.find(i);
        if !is_bad_group[g] {
            is_good_group[g] = true;
        }
    }

    println!("{}", is_good_group.iter().filter(|&&g| g).count());
}
