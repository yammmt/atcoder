// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    print!("{}", s);
    let vs: Vec<char> = s.chars().collect();
    if vs[(vs.len() - 1) as usize] == 's' {
        println!("es");
    } else {
        println!("s");
    }
}
