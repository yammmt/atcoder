use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        tnn: [[usize; n]; n],
    }

    let mut ans = 0;
    for perm in (0..n).permutations(n) {
        // 汚いけどまあ
        if perm[0] != 0 {
            continue;
        }

        let mut cur = 0;
        for i in 1..n {
            cur += tnn[perm[i - 1]][perm[i]];
        }
        cur += tnn[*perm.last().unwrap()][0];

        if cur == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
