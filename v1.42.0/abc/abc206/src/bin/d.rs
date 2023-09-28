// -> 7min

use petgraph::unionfind::UnionFind;
use proconio::input;

const AMAX: usize = 200_000;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let mut uf = UnionFind::new(AMAX);
    for i in 0..n / 2 {
        uf.union(an[i] - 1, an[n - i - 1] - 1);
    }

    let mut grpnum = vec![0; AMAX];
    for i in 0..AMAX {
        grpnum[uf.find(i)] += 1;
    }

    let mut ans = 0;
    for &g in &grpnum {
        if g > 1 {
            ans += g - 1;
        }
    }

    println!("{}", ans);
}
