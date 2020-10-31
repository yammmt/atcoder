// -*- coding:utf-8-unix -*-

use proconio::input;

// // n^p mod m (繰り返し二乗法)
// fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
//     if p == 0 {
//         1
//     } else if p == 1 {
//         n % m
//     } else if p % 2 == 0 {
//         repeat_square(n, p / 2, m).pow(2) % m
//     } else {
//         (n * repeat_square(n, p - 1, m)) % m
//     }
// }

// fn ncr_mod(n: u64, r: u64, m: u64) -> u64 {
//     let mut denominator = n;
//     let mut numerator = 1;
//     for i in 1..r {
//         denominator = (denominator * (n - i)) % m;
//         numerator = (numerator * (i + 1)) % m;
//     }
//     (denominator * repeat_square(numerator, m - 2, m)) % m
// }

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let d = 998244353;

    // let mut ans = 0;
    // let ab = (a * b) % d;
    // let csum = (c * ((c + 1) % d)) % d / 2;
    // ans = (ab * csum) % d;
    // println!("{}", ans);
    // ans = (ans * ncr_mod(a + b, 2, d)) % d;
    // println!("{}", ans);

    // let ans = (((((a * b) % d) * c) % d) * 3) % d;
    // println!("{}", ans);

    let mut ans = 0;
    // let ab = (a * b) % d;
    let csum = (c * ((c + 1) % d)) % d / 2;
    // let ans = (ab * csum) % d;
    // let bc = (b * c) % d;
    let asum = (a * ((a + 1) % d)) % d / 2;
    // let ans = (ans + (((bc * asum) % d) / 2)) % d;
    // let ca = (c * a) % d;
    let bsum = (b * ((b + 1) % d)) % d / 2;
    // let ans = (ans + (((ca * asum) % d) / 2)) % d;
    // println!("{}", asum);
    // println!("{}", bsum);
    // println!("{}", csum);

    // let mut ans = (((asum + bsum) % d) + ((bsum + csum) % d)) % d;
    // ans = (ans + d - 2) % d;
    let ans = (((asum * bsum) % d) * csum) % d;
    println!("{}", ans);
}
