// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: [u64; n],
    }

    let mut v = vec![0; (n + 1) as usize];
    for i in &a {
        v[*i as usize] += 1;
    }

    let mut p = vec![0; (n + 1) as usize];
    for i in 0..v.len() {
        if v[i as usize] < 2{
            continue; // 0
        } else {
            p[i as usize] = (v[i as usize] * (v[i as usize] - 1)) / 2;
        }
    }
    // println!("a: {:?}", a);
    // println!("v: {:?}", v);
    // println!("p: {:?}", p);
    // return;
    let total_equal_counts: u64 = p.iter().sum();
    // println!("base: {}", total_equal_counts); // pass

    // for i in 1..n + 1 {
    for i in 0..n {
        let a_current = a[i as usize];
        let num_of_current_a = v[a_current as usize];
        // println!("i: {}, {}", i, v[a[i as usize] + 1 as usize]);
        if num_of_current_a < 2 {
            println!("{}", total_equal_counts);
        } else {
            println!("{}", total_equal_counts - (num_of_current_a - 1));
        }
    }
}
