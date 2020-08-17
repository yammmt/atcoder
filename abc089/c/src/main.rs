// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut march = [0_u64; 5];
    for x in s {
        let name: Vec<char> = x.chars().collect();
        match name[0] {
            'M' => march[0] += 1,
            'A' => march[1] += 1,
            'R' => march[2] += 1,
            'C' => march[3] += 1,
            'H' => march[4] += 1,
            _ => {},
        }
    }
    let mut ans = 0;
    for i in 0..5 {
        for j in i + 1..5 {
            for k in j + 1..5 {
                ans += march[i] * march[j] * march[k]
            }
        }
    }
    println!("{}", ans);
}
