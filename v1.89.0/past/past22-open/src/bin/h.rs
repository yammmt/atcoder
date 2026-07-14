use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        lrm: [(Usize1, Usize1); m],
    }

    let mut events = Vec::with_capacity(2 * m);
    for (l, r) in lrm {
        // l<r は保証されていない
        let a = l.min(r);
        let b = l.max(r);
        events.push((a, 1));
        events.push((b + 1, -1));
    }
    events.sort_unstable();

    let mut ans = Vec::with_capacity(n);
    let mut cur = 0;
    let mut e = 0;
    for i in 0..n {
        while e < events.len() && events[e].0 == i {
            cur += events[e].1;
            e += 1;
        }

        ans.push(cur);
    }

    println!("{}", ans.iter().join(" "));
}
