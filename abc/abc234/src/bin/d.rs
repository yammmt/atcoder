// 13min

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [isize; n],
    }

    let mut bh = BinaryHeap::new();
    pn.iter().take(k).for_each(|&p| bh.push(-p));
    // println!("{:?}", bh);

    println!("{}", -bh.peek().unwrap());
    for &p in pn.iter().skip(k) {
        // print!("{}: ", p);
        let prev_ans = -bh.peek().unwrap();
        if p <= prev_ans {
            println!("{}", prev_ans);
        } else {
            bh.pop();
            bh.push(-p);
            println!("{}", -bh.peek().unwrap());
        }
    }
}
