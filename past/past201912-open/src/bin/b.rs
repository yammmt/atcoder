use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        an: [i64; n],
    }
    for i in 1..n {
        if an[i] == an[i - 1] {
            println!("stay");
        } else if an[i] > an[i - 1] {
            println!("up {}", an[i] - an[i - 1]);
        } else {
            println!("down {}", an[i - 1] - an[i]);
        }
    }
}
