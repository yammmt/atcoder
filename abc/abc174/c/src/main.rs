// -*- coding:utf-8-unix -*-

// See https://qiita.com/tanakh/items/a312a9bd684658ab1e7b#c---repsept

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        k: u64,
    }

    let mut bts = BTreeSet::new();
    let mut m = 7 % k;
    let mut ans = 1;
    loop {
        if m == 0 {
            println!("{}", ans);
            break;
        } else if bts.contains(&m) {
            println!("-1");
            break;
        }

        bts.insert(m);
        m = (m * 10 + 7) % k;
        ans += 1;
    }
}
