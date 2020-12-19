use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn seven_in_10(mut n: u64) -> bool {
    while n > 0 {
        if n % 10 == 7 {
            return false;
        }
        n /= 10;
    }
    true
}

fn seven_in_8(mut n: u64) -> bool {
    while n > 0 {
        if n % 8 == 7 {
            return false;
        }
        n /= 8;
    }
    true
}

fn main() {
    input! {
        n: u64,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        if seven_in_10(i) && seven_in_8(i) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
