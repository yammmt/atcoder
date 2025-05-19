use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut sorted = BinaryHeap::new();
    let mut deque = VecDeque::new();

    for _ in 0..q {
        input! {
            qq: usize,
        }
        match qq {
            1 => {
                input! {
                    x: usize,
                }
                deque.push_back(x);
            }
            2 => {
                if let Some(Reverse(ans)) = sorted.pop() {
                    println!("{ans}");
                } else {
                    let ans = deque.pop_front().unwrap();
                    println!("{ans}");
                }
            }
            3 => {
                while let Some(d) = deque.pop_front() {
                    sorted.push(Reverse(d));
                }
            }
            _ => unreachable!(),
        }
    }
}
