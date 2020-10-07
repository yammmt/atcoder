// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut n: u64,
    }

    let mut ans = vec![];
    while n > 0 {
        n -= 1;
        ans.push(b'a' + (n % 26) as u8);
        n /= 26;
    }
    ans.reverse();
    let r = std::str::from_utf8(&ans).unwrap();
    println!("{}", r);
}
