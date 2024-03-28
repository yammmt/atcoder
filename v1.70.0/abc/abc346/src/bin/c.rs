use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    let mut ans = (1 + k) * k / 2;
    let mut appeared = HashSet::new();
    for a in an {
        if a <= k {
            appeared.insert(a);
        }
    }
    for a in appeared {
        ans -= a;
    }

    println!("{ans}");
}
