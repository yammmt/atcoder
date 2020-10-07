// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let vn: Vec<char> = s.chars().collect();
    let mut sum = 0;
    for i in 0..vn.len() {
        sum += vn[i as usize] as i32 - 48;
    }
    if sum % 9 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
