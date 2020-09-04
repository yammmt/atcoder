// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }
    let set_time = m * 1900 + (n - m) * 100;
    let try_cnt = 2_u32.pow(m);
    println!("{}", set_time * try_cnt);
}
