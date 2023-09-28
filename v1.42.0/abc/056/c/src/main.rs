// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i64,
    }

    let mut i = 1;
    let mut current = 0;
    loop {
        current += i;
        if current >= x {
            println!("{}", i);
            return;
        }

        i += 1;
    }
}
