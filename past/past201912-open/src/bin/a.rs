use proconio::input;
use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        s: String,
    }
    match s.parse::<u16>() {
        Ok(a) => println!("{}", a * 2),
        Err(_) => println!("error"),
    }
}
