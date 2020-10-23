// -*- coding:utf-8-unix -*-

// https://drken1215.hatenablog.com/entry/2020/01/22/071000

use proconio::input;
use std::collections::HashMap;

// n^p mod m (繰り返し二乗法)
fn repeat_square(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        1
    } else if p == 1 {
        n % m
    } else if p % 2 == 0 {
        repeat_square(n, p / 2, m).pow(2) % m
    } else {
        (n * repeat_square(n, p - 1, m)) % m
    }
}

fn ninv(n: u64, p: u64) -> u64 {
    repeat_square(n, p - 2, p)
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    if n == 1 {
        println!("1");
        return;
    }

    let modp = 10u64.pow(9) + 7;

    let mut vprime = vec![0; 1000000];
    for i in &a {
        let mut chm = HashMap::new();
        let mut n = *i;
        let mut p = 2;
        while n > 1 {
            if n % p == 0 {
                let counter = chm.entry(p).or_insert(0);
                *counter += 1;
                n /= p;
            } else if p * p > n {
                let counter = chm.entry(n).or_insert(0);
                *counter += 1;
                break;
            } else {
                p += 1;
            }
        }
        for (k, v) in &chm {
            vprime[*k as usize] = vprime[*k as usize].max(*v);
        }
    }
    // println!("{:?}", vprime);
    let mut lcm = 1;
    for i in 0..vprime.len() {
        if vprime[i] == 0 {
            continue;
        }

        for _ in 0..vprime[i] {
            lcm = (lcm * i as u64) % modp;
        }
    }
    // println!("lcm: {}", lcm);

    let mut ans = 0;
    for i in &a {
        // println!("i: {}, ninv: {}", i, ninv(*i, modp));
        ans = (ans + (lcm * ninv(*i, modp)) % modp) % modp;
        // println!("ans: {}", ans);
    }
    println!("{}", ans);
}
