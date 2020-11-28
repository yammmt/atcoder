use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
        y: i64,
    }

    let mut ans;
    if a < b {
        // println!("a < b");
        if a == 1 && b == 2 {
            ans = x + y;
        } else {
            // println!("b - a: {}", b-a);
            let aa = 2 * (b - a) * x + x;
            let bb = x + (b - a) * y;
            // println!("a: {}", a);
            // println!("b: {}", b);
            ans = aa.min(bb);
        }
    } else if a > b {
        // println!("a > b");
        let roka = (2 * (a - b) - 1) * x;
        let kaidan = x + y * (a - b - 1);
        ans = roka.min(kaidan);
    } else {
        ans = x;
    }
    println!("{}", ans);
}
