// 2.5min

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        dn: [u64; n],
        m: usize,
        tm: [u64; m],
    }

    let mut hm = HashMap::new();
    for d in &dn {
        let cnt = hm.entry(d).or_insert(0);
        *cnt += 1;
    }

    for t in &tm {
        let cnt = hm.entry(t).or_insert(0);
        if *cnt == 0 {
            println!("NO");
            return;
        }
        *cnt -= 1;
    }

    println!("YES");
}
