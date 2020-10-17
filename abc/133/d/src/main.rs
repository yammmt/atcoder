// -*- coding:utf-8-unix -*-

// https://takeg.hatenadiary.jp/entry/2019/09/15/145957
// 別海: https://drken1215.hatenablog.com/entry/2019/07/07/235600

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let asum = a.iter().sum::<u64>();
    let mut m0 = asum;
    for i in 1..n {
        if i % 2 == 1 {
            m0 -= 2 * a[i];
        }
    }
    print!("{} ", m0);
    let mut prev = m0;
    for i in 1..n {
        let mi = 2 * a[i - 1] - prev;
        print!("{}", mi);
        prev = mi;
        if i == n - 1 {
            println!("");
        } else {
            print!(" ");
        }
    }
}
