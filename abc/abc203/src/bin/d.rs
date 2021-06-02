// 二次元累積和

// http://sigma425.hatenablog.com/entry/2015/02/12/145854
// これでも解けそうだが Rust には最大値を簡単に拾う方法がない
// `iter` で最小値は拾えるので small 側には負にした値を入れておく？
// 結果として相当の重実装となってしぬ

use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        ann: [[i64; n]; n],
    }

    // 愚直に全通り試すと O(NKK) で計算回数が最大 5.0x10^8 回ほどとなり微妙そう
    // ann 最小値から順に中央値になるか見る？でも最悪パターンで計算量減るわけでもない
    // 累積和のような見方？中央値が A 以下である場合には A 以下の数が k^2/2 個？
    // 答えに連続性がないので二分探索ではない 多分
    // multiset で長方形を動かせば間に合う？でも Rust で中央値を O(logN) で得る方法あったか
    // あるいは都度ソートしようにもソートに O(K^2logK^2) かかるので苦しい

    let mut ans = std::i64::MAX / 2;
    let small_set_num = if k % 2 == 0 {
        k * k / 2
    } else {
        k * k / 2 + 1
    };
    let mut small = BTreeSet::new();
    let mut large = BTreeSet::new();
    for i in 0..k {
        for j in 0..k {
            if small.len() < small_set_num as usize {
                small.insert((-ann[i][j], (i, j)));
            } else {
                let mut small_iter = small.iter();
                let small_largest = *(small_iter.next().unwrap());
                if -small_largest.0 < ann[i][j] {
                    large.insert((ann[i][j], (i, j)));
                } else {
                    small.remove(&small_largest);
                    small.insert((-ann[i][j], (i, j)));
                    large.insert((-small_largest.0, small_largest.1));
                }
            }
        }
    }
    // println!("{:?}", large);
    // println!("{:?}", small);
    let mut small_iter = small.iter();
    let small_largest = *(small_iter.next().unwrap());
    ans = ans.min(-small_largest.0);

    // let mut left_up = 0;
    // let mut next_dir = Some();
    // while let Some(moved_to) = next_dir {
    // }


    println!("{}", ans);
}
