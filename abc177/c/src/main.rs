// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut ans = 0;
    let mut part_sum = a.iter().sum::<u64>();
    let divisor = 10_u64.pow(9) + 7;
    // println!("ps: {}", part_sum);
    // println!("{:?}", a);
    for i in 1..n {
        // println!("ai-1: {}", a[i - 1]);
        part_sum = (part_sum + divisor - a[i - 1]) % divisor;
        // println!("ps: {}", part_sum);
        ans = (ans + (a[i - 1] * part_sum) % divisor) % divisor;
    }
    println!("{}", ans);
}
