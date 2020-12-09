use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
    }
    println!("{}", (a * n).min(b));
}
