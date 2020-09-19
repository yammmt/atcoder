// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut current_str = 0;
    let mut max_str = 0;
    for i in 0..n {
        input! {
            d1: u32,
            d2: u32,
        }
        if d1 == d2 {
            current_str += 1;
            max_str = max_str.max(current_str);
        } else {
            current_str = 0;
        }
    }
    if max_str >= 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}
