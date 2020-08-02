// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: u32,
        mut c: Chars,
    }

    // 左端に R を集める
    // 左端から R 個数個までの範囲にある W は R に置換される
    // 上の範囲に入らない R の数は左端置換範囲内に入る W の数に等しい
    let num_of_red = c.iter().filter(|&a| *a == 'R').count();
    let mut ans = 0;
    for i in 0..num_of_red {
        if c[i as usize] == 'W' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
