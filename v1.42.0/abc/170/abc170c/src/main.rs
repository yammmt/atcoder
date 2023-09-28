// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i32,
        n: i32,
        mut p: [i32; n],
    }

    p.sort();
    match p.binary_search(&x) {
        Ok(n) => {
            let mut lower = p[n];
            let mut i = 1;
            loop {
                if !p.contains(&(p[n] - i)) {
                    lower = p[n] - i;
                    break;
                }
                i += 1;
            }

            let mut greater = p[n];
            let mut i = 1;
            loop {
                if !p.contains(&(p[n] + i)) {
                    greater = p[n] + i;
                    break;
                }
                i += 1;
            }

            if (lower - x).abs() > (greater - x).abs() {
                println!("{}", greater);
            } else {
                println!("{}", lower);
            }
        },
        Err(_) => {
            println!("{}", x);
        }
    }
}
