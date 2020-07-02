// -*- coding:utf-8-unix -*-

use proconio::input;
use std::cmp::Ordering;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
        x: u16,
    }

    let mut ans = 0;
    for i in 0..a+1 {
        for j in 0..b+1 {
            if i * 500 + j * 100 > x {
                break;
            }
            for k in 0..c+1 {
                match (i * 500 + j * 100 + k * 50).cmp(&x) {
                    Ordering::Less => {},
                    Ordering::Equal => {
                        ans += 1;
                        break;
                    },
                    Ordering::Greater => break,
                }
            }
        }
    }
    println!("{}", ans);
}
