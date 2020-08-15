// -*- coding:utf-8-unix -*-

use proconio::input;

// n^p mod m
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
        n: u64,
        a: u64,
        b: u64,
    }

    let divisor = 10_u64.pow(9) + 7;
    let mut ans: u64 = repeat_square(2, n, divisor) - 1;

    let mut current_ncr = 1;
    for i in 1..a + 1 {
        current_ncr = (current_ncr * (n - i + 1)) % divisor;
        current_ncr = (current_ncr / i) % divisor;
    }
    ans -= current_ncr;

    let mut current_ncr = 1;
    for i in 1..b + 1 {
        current_ncr = (current_ncr * (n - i + 1)) % divisor;
        current_ncr = (current_ncr / i) % divisor;
    }
    ans -= current_ncr;

    println!("{}", ans);
}
