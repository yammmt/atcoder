// もたつく数問 (25min -> 14min)

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i64,
    }

    // 約数列挙
    let mut divisors = HashSet::new();
    let mut cur_i = 1;
    while cur_i * cur_i <= 2 * n {
        if 2 * n % cur_i == 0 {
            divisors.insert(cur_i);
            divisors.insert(2 * n / cur_i);
        }
        cur_i += 1;
    }
    // println!("{:?}", divisors);

    let mut ans = 0;
    // 等差数列の長さを m
    for m in &divisors {
        // println!("m: {}", m);
        if (2 * n / m - m + 1) % 2 == 0 {
            // println!(" += 1");
            ans += 1;
        }
    }

    println!("{}", ans);
}
