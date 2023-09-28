use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        ann: [[usize; n]; n],
        m: usize,
        xym: [(usize, usize); m],
    }

    let mut banned = HashSet::new();
    for xy in &xym {
        banned.insert((xy.0 - 1, xy.1 - 1));
        banned.insert((xy.1 - 1, xy.0 - 1));
    }

    let mut ans = DUMMY;
    for perm in (0..n).permutations(n) {
        let mut cur = 0;
        for i in 0..n {
            cur += ann[perm[i]][i];
            if i > 0 && banned.contains(&(perm[i], perm[i - 1])) {
                cur = DUMMY;
                break;
            }
        }
        ans = ans.min(cur);
    }

    println!("{}", if ans == DUMMY { -1 } else { ans as isize });
}
