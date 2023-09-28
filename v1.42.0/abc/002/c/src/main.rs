// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut xa: f64,
        mut ya: f64,
        mut xb: f64,
        mut yb: f64,
        mut xc: f64,
        mut yc: f64,
    }

    xb -= xa;
    yb -= ya;
    xc -= xa;
    yc -= ya;
    println!(
        "{}",
        (xb * yc - yb * xc).abs() / 2.0
    );
}
