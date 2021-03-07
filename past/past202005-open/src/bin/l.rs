// :fu: :fu: 21-03 PAST っぽい問題であり, データの扱いがとてもややこしい

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
    }
    let mut kn = vec![];
    let mut tnkn = vec![];
    for _ in 0..n {
        input! {
            kk: usize,
            // 手前の数字が若い
            tk: [usize; kk],
        }
        kn.push(kk);
        tnkn.push(tk);
    }
    input! {
        m: usize,
        am: [usize; m],
    }

    let mut ones = BinaryHeap::new();
    let mut twos = BinaryHeap::new();
    // shown[i][Some(1), None]: 棚 i の手前に品 1, 奥はなし
    let mut shown = vec![vec![]; n];

    // init
    for i in 0..n {
        let inserted = (tnkn[i][0], (i, 0));
        ones.push(inserted);
        twos.push(inserted);
        shown[i].push(Some(0));
        if kn[i] > 1 {
            let inserted = (tnkn[i][1], (i, 1));
            twos.push(inserted);
            shown[i].push(Some(1));
        } else {
            shown[i].push(None);
        }
    }

    // 関数に切り出せないからバグを生む
    for a in &am {
        // println!("shown: {:?}", shown);
        // println!("1: {:?}", ones);
        // println!("2: {:?}", twos);
        if *a == 1 {
            // もっていかれたものの次の商品を 1/2 両方に挿入する
            let mut bought = ones.pop().unwrap();
            while Some((bought.1).1) != shown[(bought.1).0][0] {
                bought = ones.pop().unwrap();
            }
            println!("{}", bought.0);

            let shelf = (bought.1).0;

            shown[shelf][0] = shown[shelf][1];
            // if let かからない場合は shown[shelf][0..2] は None であり処理不要
            if let Some(inserted) = shown[shelf][0] {
                // 1 に挿入する
                ones.push((tnkn[shelf][inserted], (shelf, inserted)));

                // 2 に挿入する
                if let Some(second) = shown[shelf][1] {
                    if second < kn[shelf] - 1 {
                        shown[shelf][1] = Some(second + 1);
                        twos.push((tnkn[shelf][second + 1], (shelf, second + 1)));
                    } else {
                        shown[shelf][1] = None;
                    }
                } else {
                    shown[shelf][1] = None;
                }
            }
        } else {
            let mut bought = twos.pop().unwrap();
            while Some((bought.1).1) != shown[(bought.1).0][0]
                && Some((bought.1).1) != shown[(bought.1).0][1]
            {
                bought = twos.pop().unwrap();
            }
            println!("{}", bought.0);

            let shelf = (bought.1).0;
            let inserted_idx = (bought.1).1;
            if Some(inserted_idx) == shown[shelf][0] {
                // println!("front");
                // もっていかれたものが手前側にあったならば, 1 には次の商品を, 2 には次の次の商品を挿入
                // 1 に挿入する
                if let Some(second) = shown[shelf][1] {
                    ones.push((tnkn[shelf][second], (shelf, second)));
                }
                shown[shelf][0] = shown[shelf][1];
                // 2 に挿入する
                if let Some(second) = shown[shelf][1] {
                    if second < kn[shelf] - 1 {
                        shown[shelf][1] = Some(second + 1);
                        twos.push((tnkn[shelf][second + 1], (shelf, second + 1)));
                    } else {
                        shown[shelf][1] = None;
                    }
                } else {
                    shown[shelf][1] = None;
                }
            } else if Some(inserted_idx) == shown[shelf][1] {
                // println!("rear");
                // もっていかれたものが奥側にあったならば, 1 には何もせず 2 にはその後ろの商品を挿入
                if inserted_idx < kn[shelf] - 1 {
                    shown[shelf][1] = Some(inserted_idx + 1);
                    twos.push((tnkn[shelf][inserted_idx + 1], (shelf, inserted_idx + 1)));
                } else {
                    shown[shelf][1] = None;
                }
            } else {
                unreachable!();
            }
        }
    }
}
