// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        mut n: u64,
    }

    let mut hm = HashMap::new();
    let mut divisor = 2;
    while n > 1 {
        // println!("n, divisor: {}, {}", n, divisor);
        if n % divisor == 0 {
            let counter = hm.entry(divisor).or_insert(0);
            *counter += 1;
            n /= divisor;
        } else if divisor * divisor > n {
            let counter = hm.entry(n).or_insert(0);
            *counter += 1;
            break;
        } else {
            divisor += 1;
        }
    }
    // println!("{:?}", hm);

    let mut ans = 0;
    for (_, v) in hm.iter() {
        // println!("v: {}", v);
        let mut a = *v;
        let mut b = 1;
        while a >= b {
            a -= b;
            b += 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
