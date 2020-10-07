// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u64,
    }

    let mut k = 10u64;
    loop {
        let mut current = 0;
        let mut kk = k;
        let mut idx = 0;
        while kk > 0 {
            current += (kk % 10) * k.pow(idx);
            kk /= 10;
            idx += 1;
        }
        if current > a {
            break;
        } else if current == a {
            println!("{}", k);
            return;
        }

        k += 1;
    }
    println!("-1");
}
