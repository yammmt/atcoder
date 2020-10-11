// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, usize); n],
        x: String,
    }

    let mut ans = 0;
    let mut asleep = false;
    for i in &st {
        if asleep {
            ans += i.1;
        }

        if i.0 == x {
            asleep = true;
        }
    }
    println!("{}", ans);
}
