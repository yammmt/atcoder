// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u16,
        a: [[u16; n]; 2],
    }

    let mut sum_r1: u16 = a[0][0];
    let mut sum_r2: u16 = a[1].iter().sum();
    // println!("{}, {}", sum1, sum2);
    let mut ans = 0;
    for i in 0..n {
        if i != 0 {
            sum_r1 += a[0][i as usize];
            sum_r2 -= a[1][(i - 1) as usize];
        }
        // println!("{}: {} {}", i, sum_r1, sum_r2);
        if sum_r1 + sum_r2 > ans {
            ans = sum_r1 + sum_r2;
        }
    }

    println!("{}", ans);
}
