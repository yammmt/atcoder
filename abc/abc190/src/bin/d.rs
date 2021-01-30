// 数問

use proconio::input;
use std::collections::HashSet;

fn solve(n: i64, nn: i64) -> bool {
    let a1 = (2 * n) / nn + 1 - nn;
    a1 % 2 == 0
}

fn main() {
    input! {
        n: i64,
    }

    if n == 0 {
        println!("1");
        return;
    }

    let mut ans = HashSet::new();
    let mut i = 1;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
            continue;
        }

        let n1 = i;
        let n2 = n / i;
        if solve(n, n1) {
            ans.insert(n1);
        }
        if solve(n, n2) {
            ans.insert(n2);
        }
        if solve(n, 2 * n1) {
            ans.insert(2 * n1);
        }
        if solve(n, 2 * n2) {
            ans.insert(2 * n2);
        }

        i += 1;
    }

    println!("{}", ans.len());
}
