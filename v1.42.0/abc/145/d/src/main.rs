// -*- coding:utf-8-unix -*-

// 28min

use proconio::input;

fn ncr_mod(n: u64, r: u64, m: u64) -> u64 {
    if r == 0 || n == r {
        return 1;
    }

    let mut denominator = n;
    let mut numerator = 1;
    for i in 1..r {
        denominator = (denominator * (n - i)) % m;
        numerator = (numerator * (i + 1)) % m;
    }
    (denominator * repeat_square(numerator, m - 2, m)) % m
}

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn main() {
    input! {
        x: u64,
        y: u64,
    }
    let d = 10u64.pow(9) + 7;

    if 2 * y < x || (2 * y - x) % 3 != 0 || 2 * x < y || (2 * x - y) % 3 != 0 {
        println!("0");
        return;
    }

    let a = (2 * y - x) / 3;
    let b = (2 * x - y) / 3;

    println!(
        "{}",
        ncr_mod(a + b, a.min(b), d)
    );
}
