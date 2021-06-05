// :fu: 21-06 単純にする
// コードテストで通るサンプル 2 がジャッジに通らない なぜ？

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut an: [usize; n],
    }
    an.iter_mut().for_each(|a| *a %= m);
    // println!("{:?}", an);

    let mut ans = 0usize;
    let mut hm = HashMap::new();
    hm.insert(0, 1);
    let mut cur = 0;
    for a in &an {
        cur = (cur + *a) % m;
        let cnt = hm.entry(cur).or_insert(0);
        ans += *cnt;
        *cnt += 1;
    }
    // println!("{:?}", hm);

    println!("{}", ans);
}
