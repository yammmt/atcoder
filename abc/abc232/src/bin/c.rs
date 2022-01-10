// C としては実装が重い？

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        cdm: [(usize, usize); m],
    }

    let mut edges_tkhs = vec![vec![]; n];
    for ab in &abm {
        edges_tkhs[ab.0 - 1].push(ab.1 - 1);
        edges_tkhs[ab.1 - 1].push(ab.0 - 1);
    }
    edges_tkhs.iter_mut().for_each(|e| e.sort());

    for perm in (0..n).permutations(n) {
        // 元の 1, 2, ... が perm[0], perm[1], ...
        let mut edges_aoki = vec![vec![]; n];
        for cd in &cdm {
            edges_aoki[perm[cd.0 - 1]].push(perm[cd.1 - 1]);
            edges_aoki[perm[cd.1 - 1]].push(perm[cd.0 - 1]);
        }
        edges_aoki.iter_mut().for_each(|e| e.sort());

        if edges_aoki == edges_tkhs {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
