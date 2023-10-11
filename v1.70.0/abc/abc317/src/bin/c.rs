use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, usize); m],
    }

    let mut edges = vec![vec![0; n]; n];
    for abc in &abcm {
        let a = abc.0 - 1;
        let b = abc.1 - 1;
        let c = abc.2;
        edges[a][b] = c;
        edges[b][a] = c;
    }

    let mut ans = 0;
    for perm in (0..n).permutations(n) {
        let mut cur = 0;
        for i in 0..n - 1 {
            let cost = edges[perm[i]][perm[i + 1]];
            if cost == 0 {
                break;
            }

            cur += cost;
        }
        ans = ans.max(cur);
    }

    println!("{ans}");
}
