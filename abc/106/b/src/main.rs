// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        if i % 2 == 0 {
            continue;
        }

        let mut divisor_num = 0;
        for j in 1..i + 1 {
            if i % j == 0 {
                divisor_num += 1;
            }
            if divisor_num > 8 {
                break;
            }
        }
        if divisor_num == 8 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
