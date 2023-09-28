// -*- coding:utf-8-unix -*-

// 11min (1WA)

use proconio::input;

fn main() {
    input! {
        mut n: u32,
        m: u32,
    }
    n = n % 12;
    let n_deg = 360.0 * n as f64 / 12.0 + 360.0 * m as f64 / 60.0 / 12.0;
    let m_deg = 360.0 * m as f64 / 60.0;
    let d = (n_deg - m_deg).abs();
    println!("{}", d.min(360.0 - d));
}
