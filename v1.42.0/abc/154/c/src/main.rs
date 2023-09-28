// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut vincluded = vec![false; 1000000001];
    for i in &a {
        if vincluded[*i as usize] {
            println!("NO");
            return;
        }
        vincluded[*i as usize] = true;
    }
    println!("YES");
}
