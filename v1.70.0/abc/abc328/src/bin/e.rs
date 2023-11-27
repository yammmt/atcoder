use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        uvwm: [(usize, usize, usize); m],
    }

    let mut ans = k + 1;
    // 28C7
    // combinations でなく permutations だと TLE する
    for perm in (0..m).combinations(n - 1) {
        let mut cur = 0;
        let mut uf = UnionFind::new(n);
        for &i in &perm {
            cur = (cur + uvwm[i].2) % k;
            uf.union(uvwm[i].0 - 1, uvwm[i].1 - 1);
        }

        let mut cur_pass = true;
        for i in 1..n {
            if !uf.equiv(i, i - 1) {
                cur_pass = false;
                break;
            }
        }

        if cur_pass {
            ans = ans.min(cur);
        }
    }

    println!("{ans}");
}
