// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut a: i16,
        b: i16,
        mut c: i16,
        d: i16,
    }

    let mut is_a_attack = true;
    loop {
        if is_a_attack {
            c -= b;
            if c <= 0 {
                println!("Yes");
                break;
            }
        } else {
            a -= d;
            if a <= 0 {
                println!("No");
                break;
            }
        }
        is_a_attack = !is_a_attack;
    }
}
