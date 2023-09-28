use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn sa(mut a: u32) -> u32 {
    let mut ret = 0;
    while a > 0 {
        ret += a % 10;
        a /= 10;
    }
    ret
}

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    println!(
        "{}",
        sa(a).max(sa(b))
    );
}
