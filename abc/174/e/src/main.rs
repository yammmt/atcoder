// -*- coding:utf-8-unix -*-

// 42.5min 1WA
// WA: f64 使ってしまった

use proconio::input;

fn upper_bound(v: &[u64], k: u64) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] > k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        k: u64,
        mut an: [u64; n],
    }
    an.sort_unstable();

    let mut pass = an[n - 1];
    let mut fail = 0;
    while pass - fail > 1 {
        // println!("{} {}", fail, pass);
        let mid = (pass + fail) / 2;
        let cut_from = upper_bound(&an, mid);

        let mut cut_num = 0;
        for i in cut_from..n {
            cut_num += (an[i] - 1) / mid;
        }

        if cut_num <= k {
            pass = mid;
        } else {
            fail = mid;
        }
    }

    println!("{}", pass);
}
