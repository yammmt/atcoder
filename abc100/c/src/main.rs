// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    for i in &a {
        let mut current = *i;
        while current % 2 == 0 {
            ans += 1;
            current /= 2;
        }
    }
    println!("{}", ans);
}
