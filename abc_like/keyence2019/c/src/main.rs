// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    }

    let mut ans = 0;
    let mut surplus = 0;
    let mut shortage = 0;
    let mut surplus_per_i = vec![];
    for i in 0..n {
        if a[i] > b[i] {
            surplus += a[i] - b[i];
            surplus_per_i.push(a[i] - b[i]);
        } else if a[i] < b[i] {
            shortage += b[i] - a[i];
            ans += 1;
        }
    }
    if surplus < shortage {
        println!("-1");
        return;
    }

    let mut current = 0;
    surplus_per_i.sort();
    surplus_per_i.reverse();
    let mut idx = 0;
    while current < shortage {
        ans += 1;
        current += surplus_per_i[idx];
        idx += 1;
    }
    println!("{}", ans);
}
