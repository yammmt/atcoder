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

fn main() {
    input! {
        n: u64,
    }
    if n == 1 {
        println!("0");
        return;
    }

    let divisor = 10_u64.pow(9) + 7;
    let ten = repeat_square(10, n, divisor);
    let nine = repeat_square(9, n, divisor);
    let eight = repeat_square(8, n, divisor);
    // println!("{} {}", nine, eight);
    let hiku = (((2 * nine) % divisor) + divisor - eight) % divisor;
    let ans = (ten + divisor - hiku) % divisor;
    println!("{}", ans);
}
