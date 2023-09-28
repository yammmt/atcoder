// :fu: 21-11 数問 ARC の 400 点問題では？

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    // a/b を全探索すれば c は O(1) で求められるが n <= 10^11 では TLE
    // そもそも a すら全探索不可
    // a は n の三乗根以下, b は n の二乗根以下でなければならない
    let mut ans = 0u64;
    for a in 1..n + 1 {
        if a * a * a > n {
            break;
        }

        for b in a..n + 1 {
            if a * b * b > n {
                break;
            }

            ans += n / a / b + 1 - b;
        }
    }

    println!("{}", ans);
}
