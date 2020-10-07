// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }

    let mut ans = 0;
    while !h.iter().all(|&a| a == 0) {
        let mut l = 0;
        for i in 0..n {
            if h[i] > 0 {
                l = i;
                break;
            }
        }
        let mut r = l;
        for i in l + 1..n {
            if h[i] == 0 {
                break;
            }

            r = i;
        }
        for i in l..r + 1 {
            h[i] -= 1;
        }
        ans += 1;
    }
    println!("{}", ans);
}
