// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        a: [u16; n],
    }

    let mut ans = 0;
    for i in 0..n {
        // println!("i: {}", i);
        if i % 2 != 0 {
            // println!("{}", n);
            continue;
        }

        if a[i as usize] % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
