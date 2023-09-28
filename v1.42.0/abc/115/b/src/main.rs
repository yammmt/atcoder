// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut pn: [u32; n],
    }
    pn.sort_unstable();
    println!(
        "{}",
        pn.iter().sum::<u32>() - pn[n - 1] / 2
    );
}
