use proconio::input;
use std::collections::HashMap;

fn ncr(n: u64, r: u64) -> u64 {
    if r == 0 || n == r {
        1
    } else if n < r {
        0
    } else if r == 1 || n - r == 1 {
        n
    } else {
        ncr(n - 1, r - 1) + ncr(n - 1, r)
    }
}

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }

    let mut hm = HashMap::new();
    hm.insert(0, 1);
    let mut csum = 0;
    for a in &an {
        csum += *a;
        let cnt = hm.entry(csum).or_insert(0);
        *cnt += 1;
    }

    let mut ans = 0;
    for (_k, v) in &hm {
        ans += ncr(*v as u64, 2);
    }
    println!("{}", ans);
}
