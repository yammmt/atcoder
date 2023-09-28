// :fu: :fu: 21-09 計算量の見積もりが辛い

use proconio::input;
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    input! {
        q: usize,
    }

    let mut bh: BinaryHeap<isize> = BinaryHeap::new();
    let mut que = VecDeque::new();
    for _ in 0..q {
        input! {
            n: usize,
        }
        match n {
            1 => {
                input! {
                    x: isize,
                }
                que.push_back(x);
            }
            2 => {
                println!(
                    "{}",
                    if bh.is_empty() {
                        que.pop_front().unwrap()
                    } else {
                        -bh.pop().unwrap()
                    }
                );
            }
            3 => {
                while let Some(cur) = que.pop_front() {
                    bh.push(-cur);
                }
            }
            _ => unreachable!(),
        }
    }
}
