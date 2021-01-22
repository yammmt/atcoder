use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut ans = 0;
    for ab in 2..2 * n + 1 {
        let cd = ab as i64 - k;
        if cd < 2 || cd > 2 * n as i64 {
            continue;
        }

        // println!("ab: {}", ab);
        // println!("cd: {}", cd);
        let ab2 = if ab > n + 1 {
            (ab - 1) - 2 * (ab - (n + 1))
        } else {
            ab - 1
        };
        let cd2 = if cd > n + 1 {
            (cd - 1) - 2 * (cd - (n + 1))
        } else {
            cd - 1
        };
        ans += ab2 * cd2;
        // println!("{}", ans);
    }

    println!("{}", ans);
}
