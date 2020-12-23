// セグ木で殴れ

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut cn: [u64; n], // > 0
        q: usize,
    }

    let mut ans = 0;
    for _ in 0..q {
        input! {
            n: usize,
        }
        match n {
            1 => {
                input! {
                    x: usize,
                    a: u64,
                }
                if a <= cn[x - 1] {
                    cn[x - 1] -= a;
                    ans += a;
                }
            },
            2 => {
            },
            3 => {
            },
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
