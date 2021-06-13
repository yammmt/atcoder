// WA: 探索範囲が広すぎた (TLE)
// 典型に見える三分探索 あるいは二分探索でも通る？

use proconio::input;

fn f(x: f64, an: &[f64]) -> f64 {
    let mut ret = 0.0;
    for a in an {
        ret += x + *a - a.min(2.0 * x);
    }
    ret / an.len() as f64
}

fn main() {
    input! {
        n: usize,
        an: [f64; n],
    }

    // x の値を決めて三分探索
    let mut cnt = 0;
    let mut right = 1_000_000_000.0;
    let mut left = 0.0;
    while right - left >= 10f64.powf(-8.0) && cnt < 1_000 {
        // println!("{} {}", left, right);
        let mid_left = right / 3.0 + left * 2.0 / 3.0;
        let mid_right = right * 2.0 / 3.0 + left / 3.0;

        if f(mid_left, &an) >= f(mid_right, &an) {
            left = mid_left;
        } else {
            right = mid_right;
        }
        cnt += 1;
    }
    // println!("L: {} R: {}", left, right);

    println!("{}", f(right, &an));
}
