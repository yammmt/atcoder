// :fu: だが解説記事ないので解き直すべきでもない 数問 (幾何)
// ABC-C で出そうだが複素数は高校数学だっけ？

// 公式解説 "垂線の足だけ考えれば良い" は線分外に出た場合を考えると WA では
// https://perogram.hateblo.jp/entry/arc042_b
// 解説記事が特に少なく大変

use proconio::input;

fn dist(mut a: (f64, f64), b: (f64, f64), mut c: (f64, f64)) -> f64 {
    if (b.0 - c.0).abs() < 1.0 {
        return (a.0 - b.0).abs()
    } else if (b.1 - c.1).abs() < 1.0 {
        return (a.1 - b.1).abs()
    }

    a.0 -= b.0;
    a.1 -= b.1;
    c.0 -= b.0;
    c.1 -= b.1;
    let c_abs = (c.0 * c.0 + c.1 * c.1).sqrt();
    if c_abs < 1e-14 {
        return 0.0;
    }

    // 複素平面虚部の係数だからこっちでは？でも WA
    // (a.0 * c.1 + a.1 * c.0).abs() / c_abs
    (a.0 * c.1 - a.1 * c.0).abs() / c_abs
}

fn main() {
    input! {
        x: f64,
        y: f64,
        n: usize,
        xyn: [(f64, f64); n],
    }

    let mut ans = std::f64::MAX;
    for i in 0..n {
        let j = (i + 1) % n;
        ans = ans.min(dist((x, y), xyn[i], xyn[j]));
    }
    println!("{}", ans);
}
