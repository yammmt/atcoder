// 15min

use proconio::input;

#[allow(clippy::many_single_char_names)]
fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        sn: [i64; n],
    }

    let smax = sn.iter().max().unwrap();
    let smin = sn.iter().min().unwrap();
    if smax == smin {
        println!("-1");
        return;
    }

    let p = b as f64 / (smax - smin) as f64;
    let q = a as f64 - (p / n as f64) * (sn.iter().sum::<i64>() as f64);
    println!("{} {}", p, q);
}
