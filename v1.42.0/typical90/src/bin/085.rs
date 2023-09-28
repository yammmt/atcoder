// :fu: :fu: :fu: 21-07 数問 苦手 300 点問題にもありそう

use proconio::input;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        k: u64,
    }

    let mut divisors = vec![];
    let mut i = 1;
    while i * i <= k {
        if k % i == 0 {
            divisors.push(i);
            let a = k / i;
            if a != i {
                divisors.push(a);
            }
        }
        i += 1;
    }
    divisors.sort_unstable();

    let mut ans = 0;
    for ai in 0..divisors.len() {
        let a = divisors[ai];
        for bi in ai..divisors.len() {
            let b = divisors[bi];
            // a * b < k を保証
            if b > k / a {
                continue;
            }

            // a * b * (k / a / b) == k を保証
            if k % (a * b) != 0 {
                continue;
            }

            let c = k / a / b;
            if c >= b {
                // println!("{} {} {}", a, b, c);
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
