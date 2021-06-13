// 数学やエスパーに頼らず, 何も考えず三分探索しても通る

use proconio::input;

fn f(p: f64, c: f64, xyn: &[(f64, f64)]) -> f64 {
    let mut ret = 0.0;
    for xy in xyn {
        ret += (p - xy.0) * (p - xy.0);
        ret += (c - xy.1) * (c - xy.1);
    }
    ret
}

fn main() {
    input! {
        n: usize,
        c: f64,
        xyn: [(f64, f64); n],
    }

    let mut right = -100001.0f64;
    let mut left = 100001.0f64;
    for xy in &xyn {
        right = right.max(xy.0);
        left = left.min(xy.0);
    }
    let mut cnt = 0;
    while right - left > 10f64.powf(-11.0) && cnt < 10_000 {
        let mid_right = left / 3.0 + right / 3.0 * 2.0;
        let mid_left = left / 3.0 * 2.0 + right / 3.0;

        // 最小値: 下に凸
        if f(mid_right, c, &xyn) <= f(mid_left, c, &xyn) {
            left = mid_left;
        } else {
            right = mid_right;
        }
        cnt += 1;
    }

    println!("{}", f(right, c, &xyn));
}
