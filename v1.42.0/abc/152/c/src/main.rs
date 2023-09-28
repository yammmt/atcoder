// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut ans = 0;
    let mut cmin = std::usize::MAX;
    for i in &p {
        if *i <= cmin {
            ans += 1;
            cmin = *i;
        }
    }
    println!("{}", ans);
}
