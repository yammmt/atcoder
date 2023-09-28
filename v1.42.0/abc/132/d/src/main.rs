// -*- coding:utf-8-unix -*-

use proconio::input;

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

fn ncr_mod(n: u64, r: u64, m: u64) -> u64 {
    let mut denominator = n;
    let mut numerator = 1;
    for i in 1..r {
        denominator = (denominator * (n - i)) % m;
        numerator = (numerator * (i + 1)) % m;
    }
    (denominator * repeat_square(numerator, m - 2, m)) % m
}

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let d = 10u64.pow(9) + 7;
    for i in 1..k + 1 {
        let place = ncr_mod((n - k + 1) as u64, i as u64, d);
        if i == 1 {
            println!("{}", place);
            continue;
        }

        let ball = ncr_mod(((k - i) + (i - 1)) as u64, (i - 1) as u64, d);
        let cur = (place * ball) % d;
        println!("{}", cur);
    }
}
