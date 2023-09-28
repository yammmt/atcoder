// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let mut b = vec![0; n];
    let mid = n / 2;
    let mut even_offset = 0usize;
    let mut odd_offset = 1usize;
    if n % 2 == 0 {
        for i in 0..n {
            if i % 2 == 1 {
                b[mid - odd_offset] = a[i];
                odd_offset += 1;
            } else {
                b[mid + even_offset] = a[i];
                even_offset += 1;
            }
            // println!("{:?}", b);
        }
    } else {
        for i in 0..n {
            if i % 2 == 1 {
                b[mid + odd_offset] = a[i];
                odd_offset += 1;
            } else {
                b[mid - even_offset] = a[i];
                even_offset += 1;
            }
            // println!("{:?}", b);
        }
    }

    for i in 0..b.len() {
        print!("{}", b[i]);
        if i == b.len() - 1 {
            println!("");
        } else {
            print!(" ");
        }
    }
}
