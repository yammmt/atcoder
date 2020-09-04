// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut ans = Vec::with_capacity(n);
    for _i in 0..n + 1 {
        ans.push(0);
    }

    for i in 1..(n as f64).sqrt() as usize {
        for j in i..(n as f64).sqrt() as usize {
            for k in j..(n as f64).sqrt() as usize {
                let value = i * i + j * j + k * k + i * j + j * k + k * i;
                // println!("{} {} {}", i, j, k);
                // println!("value: {}", value);
                if value > n {
                    continue;
                }

                if i == j && j == k {
                    ans[value as usize] += 1;
                } else if i != j && j != k && k != i {
                    ans[value as usize] += 6;
                } else {
                    ans[value as usize] += 3;
                }
            }
        }
    }

    for i in 1..n+1 {
        println!("{}", ans[i]);
    }
}
