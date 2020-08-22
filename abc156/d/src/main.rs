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

// m が素数であること, [0, r] がすべて m の倍数でないこと (r < m)
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
        n: u64,
        a: u64,
        b: u64,
    }

    let divisor = 10_u64.pow(9) + 7;
    let mut ans: u64 = repeat_square(2, n, divisor) - 1;

    ans = (ans + divisor - ncr_mod(n, a, divisor)) % divisor;
    ans = (ans + divisor - ncr_mod(n, b, divisor)) % divisor;

    println!("{}", ans);
}
