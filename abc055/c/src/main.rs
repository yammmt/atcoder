// -*- coding:utf-8-unix -*-

use proconio::input;

fn p (n: i64, d: i64) -> i64 {
    let mut ans = 1;
    for i in 2..n + 1 {
        ans = (ans * i) % d;
    }
    ans
}

fn main() {
    input! {
        n: i64,
        m: i64,
    }

    if (n - m).abs() > 1 {
        println!("0");
        return;
    }

    let divisor = 10_i64.pow(9) + 7;
    if n == m {
        let pn = p(n, divisor);
        println!("{}", (((pn * pn) % divisor) * 2) % divisor);
    } else {
        let pn = p(n.min(m), divisor);
        println!("{}", (pn * ((pn * n.max(m)) % divisor)) % divisor);
    }
}
