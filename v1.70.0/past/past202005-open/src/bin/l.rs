// 結構な重実装

use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            k: usize,
            tk: [usize; k],
        }
        v.push(tk);
    }
    input! {
        m: usize,
        am: [usize; m],
    }

    // 最大でも手前 2 個しか見ないので Priority Queue 二つ立てる愚直でなんとかなる

    // 次に展示される idx
    let mut displayed1 = vec![1; n];
    let mut displayed2 = vec![1; n];
    // (消費期限, (i 列, j 番目))
    let mut bt1 = BTreeSet::new();
    let mut bt2 = BTreeSet::new();
    for i in 0..n {
        bt1.insert((v[i][0], (i, 0)));
        bt2.insert((v[i][0], (i, 0)));
        if v[i].len() > 1 {
            bt2.insert((v[i][1], (i, 1)));
            displayed2[i] = 2;
        }
    }

    let mut is_sold_out = vec![];
    for i in 0..n {
        is_sold_out.push(vec![false; v[i].len()]);
    }

    // a == 1
    //     - 1 の最大要素を消す
    //     - 2 にも同じ要素があり消す必要があるが, 最大値とは限らない
    // a == 2
    //     - 2 の最大要素を消す
    //     - 1 にも同じ要素がある可能性があり, この場合は必ず最大値
    // 消す要素がわかれば, そこから先は共通化できるはず
    for a in am {
        // dummy
        let mut removed_item = (0, (0, 0));
        if a == 1 {
            removed_item = *bt1.last().unwrap();
        } else {
            let ri1 = bt1.last().unwrap();
            let ri2 = bt2.last().unwrap();
            removed_item = if ri1.0 > ri2.0 { *ri1 } else { *ri2 };
        }
        println!("{}", removed_item.0);

        let sold_i = removed_item.1 .0;
        let sold_j = removed_item.1 .1;
        is_sold_out[sold_i][sold_j] = true;
        // 1 が更新される場合は必ず最大値
        if *bt1.last().unwrap() == removed_item {
            bt1.pop_last();
            while displayed1[sold_i] < v[sold_i].len() {
                if !is_sold_out[sold_i][displayed1[sold_i]] {
                    bt1.insert((v[sold_i][displayed1[sold_i]], (sold_i, displayed1[sold_i])));
                    displayed1[sold_i] += 1;
                    break;
                }

                displayed1[sold_i] += 1;
            }
        }
        // 2 の更新は最大値とは限らない
        if bt2.contains(&removed_item) {
            bt2.remove(&removed_item);
            while displayed2[sold_i] < v[sold_i].len() {
                if !is_sold_out[sold_i][displayed2[sold_i]] {
                    bt2.insert((v[sold_i][displayed2[sold_i]], (sold_i, displayed2[sold_i])));
                    displayed2[sold_i] += 1;
                    break;
                }

                displayed2[sold_i] += 1;
            }
        } else {
            unreachable!();
        }
    }
}
