use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: u8,
    }
    println!(
        "{}",
        if n % 3 == 0 {
            "YES"
        } else {
            "NO"
        }
    );
}
