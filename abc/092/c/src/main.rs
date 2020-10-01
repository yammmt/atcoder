// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut base = a[0].abs();
    for i in 1..n {
        base += (a[i] - a[i - 1]).abs();
    }
    base += a[n - 1].abs();
    // println!("base: {}", base);
    let base = base;

    // cancel a[0]
    println!("{}", base - (a[0].abs() + (a[0] - a[1]).abs()) + a[1].abs());
    // cancel a[1..n - 1]
    for i in 1..n - 1 {
        println!("{}", base - ((a[i - 1] - a[i]).abs() + (a[i] - a[i + 1]).abs()) + (a[i - 1] - a[i + 1]).abs());
    }
    //cancel a[n - 1]
    println!("{}", base - (a[n - 1].abs() + (a[n - 1] - a[n - 2]).abs()) + a[n - 2].abs());
}
