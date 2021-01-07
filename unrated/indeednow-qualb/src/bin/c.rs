// 9.5min

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        abn: [(usize, usize); n - 1],
    }

    let mut vconnected = vec![vec![]; n + 1];
    for ab in &abn {
        vconnected[ab.0].push(ab.1);
        vconnected[ab.1].push(ab.0);
    }

    let mut ans = vec![1];
    let mut vselected = vec![false; n + 1];
    vselected[1] = true;
    let mut bh = BinaryHeap::new();
    for v in &vconnected[1] {
        bh.push(Reverse(*v));
    }
    while let Some(Reverse(cur)) = bh.pop() {
        if vselected[cur] {
            continue;
        }

        ans.push(cur);
        vselected[cur] = true;
        for v in &vconnected[cur] {
            if !vselected[*v] {
                bh.push(Reverse(*v));
            }
        }
        if ans.len() == n {
            break;
        }
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
