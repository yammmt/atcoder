// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u8,
        mut b: u8,
        mut c: u8,
        k: u8,
    }

    let mut cnt = 0;
    while cnt < k {
        if a >= b {
            b *= 2;
            cnt += 1;
        } else if b >= c {
            c *= 2;
            cnt += 1;
        } else {
            break;
        }
    }

    if a < b && b < c{
        println!("Yes");
    } else {
        println!("No");
    }
}
