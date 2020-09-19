// -*- coding:utf-8-unix -*-

use proconio::input;

// not work

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut vlr = vec![];
    for _ in 0..k {
        input! {
            l: usize,
            r: usize,
        }
        vlr.push((l, r));
    }

    let mut ans = 0;
    let divisor = 998244353;
    for bit_row in 0..2u32.pow(k as u32) {
        let mut selected = vec![];
        for i in 0..n {
            if bit_row & (1 << i) > 0 {
                selected.push(i);
            }
        }
        println!("{:?}", selected);
        let mut moved_min = 0;
        let mut moved_max = 0;
        for i in &selected {
            moved_min += vlr[*i as usize].0;
            moved_max += vlr[*i as usize].1;
        }
        if moved_min > (n - 1) || moved_max < (n - 1) {
            continue;
        }
    }
    println!("{}", ans);
}
