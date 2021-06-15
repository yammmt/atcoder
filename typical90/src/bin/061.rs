// 10min

use proconio::input;

fn main() {
    input! {
        q: usize,
        txq: [(usize, usize); q],
    }

    // Rust の Deque にインデックスなどないので
    let mut deque = vec![0; q];
    let mut head = 0;
    let mut tail = 0;
    for tx in &txq {
        // println!("{:?}", deque);
        match tx.0 {
            1 => {
                // to head
                deque[head] = tx.1;
                if head == tail {
                    tail = (tail + 1) % q;
                }
                head = (head + q - 1) % q;
            }
            2 => {
                // to tail
                deque[tail] = tx.1;
                if head == tail {
                    head = (head + q - 1) % q;
                }
                tail = (tail + 1) % q;
            }
            3 => println!("{}", deque[(head + tx.1) % q]),
            _ => unreachable!(),
        }
    }
}
