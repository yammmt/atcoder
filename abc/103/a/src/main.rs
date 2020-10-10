// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: [i16; 3],
    }

    let mut ans = std::i16::MAX;
    ans = ans.min((a[0] - a[1]).abs() + (a[1] - a[2]).abs());
    ans = ans.min((a[0] - a[2]).abs() + (a[2] - a[1]).abs());
    ans = ans.min((a[1] - a[0]).abs() + (a[0] - a[2]).abs());
    ans = ans.min((a[1] - a[2]).abs() + (a[2] - a[0]).abs());
    ans = ans.min((a[2] - a[0]).abs() + (a[0] - a[1]).abs());
    ans = ans.min((a[2] - a[1]).abs() + (a[1] - a[0]).abs());
    println!("{}", ans);
}
