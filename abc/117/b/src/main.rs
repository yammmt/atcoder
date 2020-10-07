// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();
    if l.iter().take(n - 1).sum::<usize>() > l[n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
