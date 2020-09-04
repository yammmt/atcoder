// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        h: f64,
        m: f64,
    }

    let a_pos = h / 12.0 + m / 60.0 / 12.0;
    let b_pos = m / 60.0;

    let rad = 2.0 * std::f64::consts::PI * (a_pos - b_pos);
    println!("{}", (a * a + b * b - 2.0 * a * b * rad.cos()).sqrt());
}
