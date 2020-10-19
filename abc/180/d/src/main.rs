// -*- coding:utf-8-unix -*-

// https://atcoder.jp/contests/abc180/editorial/219

use proconio::input;

fn main() {
    input! {
        x: u64,
        y: u64,
        a: u64,
        b: u64,
    }

    let mut pts = (x, 0u64);
    loop {
        // `pts.0 * a` は前者が最大 10^18, 後者が 10^9 で
        // `std::u64::MAX` が 18_446_744_073_709_551_615 (10^19) であるため
        // 愚直に比較するとオーバーフローで判定に引っ掛からないで無限ループする
        if pts.0 >= y / a || pts.0 * a >= pts.0 + b {
            break;
        }

        pts.0 *= a;
        pts.1 += 1;
    }
    // println!("{:?}", pts);
    println!("{}", pts.1 + (y - 1 - pts.0) / b);
}
