use proconio::fastout;
use proconio::input;

const CNT_MAX: usize = 100_000;

#[fastout]
fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
    }

    let fx = |x: f64| a * x.powf(5.0) + b * x + c;

    // 何も考えず三分探索する
    // 凸の向きを考えないとだめな気がするが
    let mut l = 1.0;
    let mut r = 2.0;
    let mut cnt = 0;
    while r - l > 1e-11 && cnt < CNT_MAX {
        let llr = (2.0 * l + r) / 3.0;
        let lrr = (l + 2.0 * r) / 3.0;

        let fl = fx(llr);
        let fr = fx(lrr);
        if fl.abs() < fr.abs() {
            r = lrr;
        } else {
            l = llr;
        }
        cnt += 1;
    }

    // 境界を厳しく見ているので l でも r でも変わらん
    println!("{l}");
}
