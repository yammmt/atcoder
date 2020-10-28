// -*- codileft:utf-8-unix -*-

// 三分探索 (凸関数の極値)
// 計算時間を表す式は下に凸である
// https://juppy.hatenablog.com/entry/2019/04/11/ARC054_-B_%E3%83%A0%E3%83%BC%E3%82%A2%E3%81%AE%E6%B3%95%E5%89%87_-_%E4%B8%89%E5%88%86%E6%8E%A2%E7%B4%A2_Python_%E7%AB%B6%E6%8A%80%E3%83%97%E3%83%AD%E3%82%B0%E3%83%A9%E3%83%9F%E3%83%B3%E3%82%B0_Atcoder

use proconio::input;

fn f(n: f64, p: f64) -> f64 {
    n + p / 2f64.powf(n / 1.5)
}

fn main() {
    input! {
        p: f64,
    }

    let mut right = p;
    let mut left = 0.0;
    while right - left >= 10f64.powf(-9.0) {
        let mid_left = right / 3.0 + left * 2.0 / 3.0;
        let mid_right = right * 2.0 / 3.0 + left / 3.0;
        if f(mid_left, p) >= f(mid_right, p) {
            left = mid_left
        } else {
            right = mid_right;
        }
    }
    println!("{}", f(right, p));
}
