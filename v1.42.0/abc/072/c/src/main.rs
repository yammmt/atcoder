// -*- coding:utf-8-unix -*-

// 5.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    let amax = an.iter().max().unwrap();
    let mut cnt = vec![0; amax + 1];
    for a in &an {
        if *a > 0 {
            cnt[*a - 1] += 1;
        }
        cnt[*a] += 1;
        if *a < *amax {
            cnt[*a + 1] += 1;
        }
    }
    println!("{}", cnt.iter().max().unwrap());
}
