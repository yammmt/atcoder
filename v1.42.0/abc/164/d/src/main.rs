// -*- coding:utf-8-unix -*-

// :fu:
// 10^i を i 桁目に対応させるために reverse するらしい
// 左から見ていって *10 ないし %2019 取っていくと合わないのはなぜ？
// 1817181 - 81 = 1817100 のような計算ができないから？

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut vn = s.iter().map(|a| a.to_digit(10).unwrap()).collect::<Vec<u32>>();
    vn.reverse();

    let mut ans = 0;
    let mut cusum = 0u32;
    let mut cnt = vec![0; 2019];
    let mut ten = 1;
    for i in 0..vn.len() {
        // println!("[{:2}]: {}", i, cusum);
        cnt[cusum as usize] += 1;
        cusum = (cusum + ten * vn[i]) % 2019;
        // 最後にカウントに対してループしても同じ
        ans += cnt[cusum as usize];
        ten = ten * 10 % 2019;
    }

    println!("{}", ans);
}
