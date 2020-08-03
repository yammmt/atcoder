// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        mut a: [u64; n],
        q: u64,
    }

    let mut sum: u64 = 0;
    let mut a_counter: [u64; 100001] = [0; 100001];
    for i in 0..a.len() {
        sum += a[i as usize];
        a_counter[a[i as usize] as usize] += 1;
    }

    for _i in 0..q {
        input! {
            b: u64,
            c: u64,
        }
        // b != c
        if c > b {
            let d = c - b;
            sum += d * a_counter[b as usize];
        } else {
            let d = b - c;
            sum -= d * a_counter[b as usize];
        }

        a_counter[c as usize] += a_counter[b as usize];
        a_counter[b as usize] = 0;
        println!("{}", sum);
    }
}
