use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
        m: usize,
        bm: [usize; m],
        l: usize,
        cl: [usize; l],
        q: usize,
        xq: [usize; q],
    }

    let mut hs = HashSet::new();
    for a in &an {
        for b in &bm {
            for c in &cl {
                hs.insert(*a + *b + *c);
            }
        }
    }

    for x in xq {
        println!("{}", if hs.contains(&x) { "Yes" } else { "No" });
    }
}
