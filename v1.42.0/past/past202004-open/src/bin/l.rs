// :fu: 21-02 インデックスがややこしい
// WA: タプルの位置取り違えと不可判定

use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// selected 個選んだ後のインデックスの上限
fn idx_upper_limit(selected: usize, n: usize, k: usize, d: usize) -> usize {
    let rest = k - selected;
    n - rest * d
}

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        an: [i64; n],
    }

    // 不可判定
    if k > (n + d - 1) / d {
        println!("-1");
        return;
    }

    let mut cmin = (0, 9_999_999_999);
    for i in 0..idx_upper_limit(1, n, k, d) {
        if an[i] < cmin.1 {
            cmin = (i, an[i]);
        }
    }
    let mut ans = vec![cmin.1];
    let mut last_idx = cmin.0;
    let mut not_pushed_idx = cmin.0 + d;
    // println!("npi: {}", not_pushed_idx);

    let mut bh = BinaryHeap::new();
    while ans.len() < k {
        // println!("ans: {:?}", ans);
        let candidates_to = idx_upper_limit(ans.len() + 1, n, k, d);
        // println!("  [-, {})", candidates_to);
        while not_pushed_idx < candidates_to {
            bh.push(Reverse((an[not_pushed_idx], not_pushed_idx)));
            not_pushed_idx += 1;
        }
        // println!("  bh: {:?}", bh);

        let mut cur = bh.pop().unwrap();
        while (cur.0).1 < last_idx + d {
            cur = bh.pop().unwrap();
        }
        ans.push((cur.0).0);
        last_idx = (cur.0).1;
    }

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
