use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut hm = HashMap::new();
    for a in &an {
        let cnt = hm.entry(a).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0usize;
    for v in hm.values() {
        ans += v * (n - v);
    }
    ans /= 2;

    println!("{}", ans);
}
