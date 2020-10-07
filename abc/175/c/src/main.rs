// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut x: i64,
        mut k: i64,
        d: i64,
    }

    if x < 0 {
        x *= -1;
    }

    let simple_move_cnt = x / d;
    if simple_move_cnt >= k {
        println!("{}", x - k * d);
        return;
    }

    x -= simple_move_cnt * d;
    k -= simple_move_cnt;
    if k % 2 == 0 {
        println!("{}", x.abs());
    } else {
        println!("{}", (x - d).abs());
    }
}
