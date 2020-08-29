// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
    }

    let mut ans = std::i32::MAX;
    for i in -100..101 {
        let current = a.iter().map(|&y| (i - y).pow(2)).sum::<i32>();
        ans = ans.min(current);
    }
    println!("{}", ans);
}
