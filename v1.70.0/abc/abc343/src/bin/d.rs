use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        abt: [(Usize1, usize); t],
    }

    let mut pts = vec![0; n];
    let mut hm = HashMap::new();
    hm.insert(0, n);
    for (a, b) in abt {
        let pts_prev = pts[a];
        let pts_next = pts[a] + b;
        pts[a] = pts_next;

        let cnt = hm.get(&pts_prev).unwrap();
        if *cnt > 1 {
            hm.insert(pts_prev, cnt - 1);
        } else {
            hm.remove(&pts_prev);
        }

        let cnt = hm.entry(pts_next).or_insert(0);
        *cnt += 1;

        println!("{}", hm.len());
    }
}
