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
        s: u64,
    }

    let divisor = 10_u64.pow(9) + 7;
    let mut ans = 0;
    // 項の数が i (すべての項は 3 以上)
    for i in 1..s {
        if 3 * i > s {
            break;
        } else if 3 * i == s {
            ans += 1;
            break;
        }

        let needed = s - (3 * i);
        ans += ncr_mod(i + needed - 1, needed, divisor);
        ans = ans % divisor;
    }
    println!("{}", ans);
}
