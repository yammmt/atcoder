// -*- coding:utf-8-unix -*-

use std::io;

fn main() {
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();

    let ans = str.chars().filter(|c| *c == '1').count();
    println!("{}", ans);
}
