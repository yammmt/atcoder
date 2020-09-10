// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l: u32,
        r: u32,
    }

    let mut ans = 2020;
    'outer: for i in l..r.min(l + 2019) {
        for j in l + 1..(r + 1).min(l + 2019) {
            ans = ans.min(((i % 2019) * (j % 2019)) % 2019);
            if ans == 0 {
                break 'outer;
            }
        }
    }
    println!("{}", ans);
}
