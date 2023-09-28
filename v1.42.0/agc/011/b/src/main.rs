// -*- coding:utf-8-unix -*-

// 10.5min

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [u64; n],
    }

    an.sort();
    // 最大のものは明らかに他を吸収できる
    let mut ans = 1;
    let mut csum = an.iter().sum::<u64>();
    for i in (0..n).rev() {
        csum -= an[i];
        // println!("csum: {}", csum);
        if 2 * csum >= an[i] {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
