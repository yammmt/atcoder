use proconio::fastout;
use proconio::input;
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        mut lrn: [(usize, usize); n],
    }

    lrn.sort_unstable_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        _ => a.1.cmp(&b.1),
    });

    let mut ans = 1;
    let mut cur_start = lrn[0].1;
    for lr in lrn.iter().skip(1) {
        let l = lr.0;
        let r = lr.1;
        if l > cur_start + d - 1 {
            ans += 1;
            cur_start = r;
        }
    }

    println!("{ans}");
}
