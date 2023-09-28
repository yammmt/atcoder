// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: String,
    }
    let mut vn = n.chars().collect::<Vec<char>>();
    for i in &vn {
        if *i == '1' {
            print!("9");
        } else {
            print!("1");
        }
    }
    println!("");
}
