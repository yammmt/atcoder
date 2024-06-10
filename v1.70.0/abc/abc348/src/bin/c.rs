use proconio::fastout;
use proconio::input;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        acn: [(usize, usize); n],
    }

    let mut hm: HashMap<usize, usize> = HashMap::new();
    for (a, c) in acn {
        hm.entry(c)
            .and_modify(|m| *m = (*m).min(a))
            .or_insert(a);
    }

    let mut ans = 0;
    for v in hm.values() {
        ans = ans.max(*v);
    }

    println!("{ans}");
}
