use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        txyq: [(usize, Usize1, usize); q],
    }
    let mut arr = (1..=n).collect::<Vec<usize>>();

    // TODO: swap 後の操作を関数化したいが所有権
    let mut bts = BTreeSet::new();
    // let mut a = 1;
    for (t, x, y) in txyq {
        if t == 1 {
            // swap
            if arr[x] > arr[x + 1] {
                // 転倒数が 1 減る
                bts.remove(&x);
            } else {
                // 転倒数が 1 増える
                bts.insert(x);
            }
            arr.swap(x, x + 1);

            // 前に行った側
            if x > 0 {
                if arr[x - 1] < arr[x] {
                    bts.remove(&(x - 1));
                } else {
                    bts.insert(x - 1);
                }
            }
            // 後ろに行った側
            if x + 2 < n {
                if arr[x + 1] > arr[x + 2] {
                    bts.insert(x + 1);
                } else {
                    bts.remove(&(x + 1));
                }
            }
        } else {
            // sort
            let y = y - 1;
            while let Some(&i) = bts.range(x..y).next() {
                arr.swap(i, i + 1);
                bts.remove(&i);

                // 前に行った側
                if i > 0 {
                    if arr[i - 1] < arr[i] {
                        bts.remove(&(i - 1));
                    } else {
                        bts.insert(i - 1);
                    }
                }
                // 後ろに行った側
                if i + 2 < n {
                    if arr[i + 1] > arr[i + 2] {
                        bts.insert(i + 1);
                    } else {
                        bts.remove(&(i + 1));
                    }
                }
            }
        }
        // println!("{a:02}\n  {}", arr.iter().join(" "));
        // println!("  {bts:?}");
        // a += 1;
    }

    println!("{}", arr.iter().join(" "));
}
