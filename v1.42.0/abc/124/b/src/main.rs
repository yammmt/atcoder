// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u16; n],
    }
    let mut ans = 0;
    let mut hmax = 0;
    for i in &h {
        if hmax <= *i {
            ans += 1;
            hmax = *i;
        }
    }
    println!("{}", ans);
}
