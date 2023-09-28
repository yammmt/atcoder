// :fu: 21-05 標準的

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut hm = HashMap::new();
    hm.insert(0, 1i64);
    let mut cur = 0;
    for (i, a) in an.iter().enumerate() {
        if i % 2 == 0 {
            cur += *a;
        } else {
            cur -= *a;
        }
        let cnt = hm.entry(cur).or_insert(0);
        *cnt += 1;
    }
    // println!("{:?}", hm);

    let mut ans = 0i64;
    for v in hm.values() {
        if *v > 1 {
            ans += v * (v - 1) / 2;
        }
    }

    println!("{}", ans);
}
