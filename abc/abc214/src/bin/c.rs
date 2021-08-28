// :fu: 21-08 C としてはむずくない？でも灰

use proconio::input;
use std::collections::BinaryHeap;

static DUMMY: isize = std::isize::MAX / 4;

fn main() {
    input! {
        n: usize,
        sn: [isize; n],
        tn: [isize; n],
    }

    let mut ans = vec![DUMMY; n];
    let mut ansnum = 0;
    // 貰うの早い順
    let mut bh = BinaryHeap::new();
    for (i, t) in tn.iter().enumerate() {
        bh.push((-1 * t, i));
    }
    // println!("{:?}", bh);
    while let Some(cur) = bh.pop() {
        // println!("{:?}", cur);
        // println!("  {:?}", bh);
        if -cur.0 >= ans[cur.1] {
            continue;
        }

        ans[cur.1] = -cur.0;
        ansnum += 1;
        if ansnum == n {
            break;
        }

        // 次がまだもらっていない
        if ans[(cur.1 + 1) % n] == DUMMY {
            bh.push((cur.0 - sn[cur.1], (cur.1 + 1) % n));
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}
