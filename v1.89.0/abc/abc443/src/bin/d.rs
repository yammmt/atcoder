use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    // 最も上にある駒は動かさない
    // 最も上にある駒の左右の駒を一つ下まで持ってくる

    for _ in 0..t {
        input! {
            n: usize,
            mut rn: [Usize1; n],
        }

        let mut pq = BinaryHeap::new();
        rn.iter()
            .enumerate()
            .for_each(|(i, &r)| pq.push((Reverse(r), i)));
        let mut completed = vec![false; n];
        let mut ans = 0;
        while let Some((Reverse(pos), i)) = pq.pop() {
            if completed[i] {
                continue;
            }

            if i > 0 {
                let j = i - 1;
                if !completed[j] && rn[j] > pos {
                    ans += rn[j] - (pos + 1);
                    rn[j] = pos + 1;
                    pq.push((Reverse(rn[j]), j));
                }
            }
            if i < n - 1 {
                let j = i + 1;
                if !completed[j] && rn[j] > pos {
                    ans += rn[j] - (pos + 1);
                    rn[j] = pos + 1;
                    pq.push((Reverse(rn[j]), j));
                }
            }

            completed[i] = true;
        }

        println!("{ans}");
    }
}
