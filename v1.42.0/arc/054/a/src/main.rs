// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l: f64,
        x: f64,
        y: f64,
        s: f64,
        d: f64,
    }
    let d_cw = if d >= s {
        d - s
    } else {
        d + l - s
    };
    let d_ccw = if s >= d {
        s - d
    } else {
        l - (d - s)
    };
    let t_cw = d_cw / (x + y);
    let t_ccw = d_ccw / (y - x);

    if x > y {
        println!("{}", t_cw);
    } else {
        println!("{}", t_cw.min(t_ccw));
    }
}
