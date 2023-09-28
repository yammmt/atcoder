// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        let mut sum = 0;
        let mut num = i;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }

        if (a <= sum) && (sum <= b) {
            ans += i;
        }
    }
    println!("{}", ans);
}
