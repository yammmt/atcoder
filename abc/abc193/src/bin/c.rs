use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: i64,
    }

    let mut ans = HashSet::new();
    let mut i = 2;
    while i * i <= n {
        let mut cur = i * i;
        while cur <= n {
            ans.insert(cur);
            cur *= i;
        }

        i += 1;
    }

    println!("{}", n - ans.len() as i64);
}
