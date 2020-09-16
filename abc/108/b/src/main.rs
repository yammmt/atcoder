// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x1: i16,
        y1: i16,
        x2: i16,
        y2: i16,
    }

    let vx = x2 - x1;
    let vy = y2 - y1;
    println!("{} {} {} {}", x2-vy, y2+vx, x1-vy, y1+vx);
    // AD -> DC の順に回転行列使って座標を求めていくと
    // println!("{} {} {} {}", y1+x2-y2, -x1+x2+y2, x1+y1-y2, y1-x1+x2);
}
