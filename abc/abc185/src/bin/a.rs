use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        a1: u32,
        a2: u32,
        a3: u32,
        a4: u32,
    }
    println!("{}", a1.min(a2.min(a3.min(a4))));
}
