// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let x_area = x / a;
    if x_area > a * b / 2.0 {
        println!("{}", (2.0 * (a * b - x_area) / (a * a)).atan() * 180.0 / std::f64::consts::PI);
    } else {
        println!("{}", (b / (2.0 * x_area / b)).atan() * 180.0 / std::f64::consts::PI);
    }
}
