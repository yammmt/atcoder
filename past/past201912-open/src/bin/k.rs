// 最大 150,000 回遡る羽目になるので愚直に親辿るでは TLE (全員が一直線に並んだ場合)
// 一回再帰でなぞらせれば落ち着く？

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        pn: [isize; n],
        q: usize,
        abq: [(usize, usize); q],
    }

    let mut bosses = vec![HashSet::new(); n + 1];
    let mut bukas = vec![HashSet::new(); n + 1];
    for i in 0..n {
        if pn[i as usize] == -1 {
            continue;
        }

        bosses[i].insert(pn[i]);
        bukas[pn[i] as usize].insert(i + 1);
    }

    for ab in &abq {
    }
}
