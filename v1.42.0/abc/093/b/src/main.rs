// -*- coding:utf-8-unix -*-

// 4.5min

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        k: i64,
    }

    let mut ans = vec![];
    for i in 0..k {
        if a + i > b {
            break;
        }

        ans.push(a + i);
    }
    for i in (0..k).rev() {
        if b - i < a + k {
            continue;
        }

        ans.push(b - i);
    }

    for i in &ans {
        println!("{}", i);
    }
}
