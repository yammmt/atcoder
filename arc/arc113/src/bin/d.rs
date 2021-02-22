// :fu: 21-02

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
        m: u64,
        k: u64,
    }
    let d = 998_244_353;

    if n == 1 {
        println!("{}", repeat_square(k, m, d));
        return;
    } else if m == 1 {
        println!("{}", repeat_square(k, n, d));
        return;
    }

    // 前計算なしだと
    let mut ans = 0u64;
    for i in 1..k + 1 {
        ans = (ans + (repeat_square(i, n, d) + d - repeat_square(i - 1, n, d)) * repeat_square(k - i + 1, m, d)) % d;
    }

    println!("{}", ans);
}
