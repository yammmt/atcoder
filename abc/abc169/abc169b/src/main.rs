// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    if a.iter().any(|&v| v == 0) {
        println!("0");
        return;
    }

    let mut ans = a[0];
    for i in 1..n {
        match ans.checked_mul(a[i as usize]) {
            Some(v) => {
                if v > 10u64.pow(18) {
                    println!("-1");
                    return;
                }

                ans = v;
            }
            None => {
                println!("-1");
                return;
            }
        }
    }
    println!("{}", ans);
}
