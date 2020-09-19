// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut k = vec![];
    let mut s = vec![];
    for _ in 0..m {
        input! {
            a: usize,
            b: [u16; a],
        }
        k.push(a);
        s.push(b);
    }
    input! {
        p: [u16; m],
    }
    // println!("k: {:?}", k);
    // println!("s: {:?}", s);
    // println!("p: {:?}", p);

    let mut ans = 0;
    for bit_row in 0..2u32.pow(n as u32) {
        // println!("{:b}", bit_row);
        let mut selected = HashSet::new();
        for i in 0..n {
            if (bit_row & (1 << i)) > 0 {
                selected.insert(i as u16);
            }
        }
        // println!("on: {:?}", selected);

        for i in 0..m {
            let mut swsum = 0;
            for j in 0..k[i] {
                // スイッチは 1 スタート
                if selected.contains(&(s[i][j] - 1)) {
                    swsum += 1;
                }
            }
            // println!("sw no. {}: {}", i, swsum);
            if swsum % 2 != p[i] {
                break;
            }

            if i == m - 1 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
