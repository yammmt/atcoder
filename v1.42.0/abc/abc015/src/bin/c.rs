use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        k: usize,
        tnk: [[u8; k]; n],
    }

    let mut vdq = VecDeque::new();
    for i in 0..k {
        vdq.push_back((tnk[0][i], 1));
    }
    while let Some(cur) = vdq.pop_back() {
        if cur.1 == n {
            if cur.0 == 0 {
                println!("Found");
                return;
            }
        } else {
            for i in 0..k {
                vdq.push_back((cur.0 ^ tnk[cur.1][i], cur.1 + 1));
            }
        }
    }
    println!("Nothing");
}
