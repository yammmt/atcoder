// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut v3 = vec![];
    let mut v5 = vec![];
    let mut three_a = 1;
    let mut threepow = 3;
    while threepow <= n {
        v3.push((threepow, three_a));
        threepow *= 3;
        three_a += 1;
    }
    let mut five_b = 1;
    let mut fivepow = 5;
    while fivepow <= n {
        v5.push((fivepow, five_b));
        fivepow *= 5;
        five_b += 1;
    }

    for i in &v3 {
        for j in &v5 {
            if i.0 + j.0 == n {
                println!("{} {}", i.1, j.1);
                return;
            } else if i.0 + j.0 > n {
                break;
            }
        }
    }
    println!("-1");
}
