// 6.5min 想定解とは若干違うが
// 当時の試験官緑だが今なら茶色下位？

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut an: [usize; n],
    }
    an.sort_unstable();
    let an = an;

    let mut ans = 0;
    let mut used = HashSet::new();
    for a in &an {
        if used.contains(a) {
            continue;
        }

        let mut cur_a = *a;
        while cur_a <= an[n - 1] {
            used.insert(cur_a);
            cur_a *= 2;
        }

        ans += 1;
    }

    println!("{}", ans);
}
