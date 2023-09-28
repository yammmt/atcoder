// :fu: 21-06 数問風

use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    }

    let mut l = 0.0;
    let mut h = 200.0;
    while h - l > 1e-12 {
        let m = (h + l) / 2.0;
        if a * m + b * (c * m * std::f64::consts::PI).sin() < 100.0 {
            l = m;
        } else {
            h = m;
        }
    }
    println!("{}", l);
}
