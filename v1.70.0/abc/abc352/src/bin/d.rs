use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        pn: [usize; n],
    }

    // 日本語が辛い
    // 題意: サンプル 3 でいうと
    // 10 5
    // 10 1 6 8 7 2 5 9 3 4
    // [1, 5]  => 2,6,7,9,10 => 8
    // [2, 6]  => 3,6,7,8,10 => 7
    // [3, 7]  => 3,5,7,9,10 => 7
    // [4, 8]  => 3,4,5,6,10 => 7
    // [5, 9]  => 3,4,5,7,8  => 5
    // [6, 10] => 1,3,4,5,8  => 7

    // heap が昇順降順で二つあればいけそう
    // 先に個々の数の出現位置を調べておく
    // [1, K] の index の値を heap にもっておいて peek する
    // 1 の出現位置を heap から削除して K+1 の出現位置を heap に入れる, を繰り返す
    //   heap 削除部分は高速化する

    let mut pos = vec![0; n + 1];
    for (i, &p) in pn.iter().enumerate() {
        pos[p] = i;
    }

    let mut should_rm = vec![false; n + 1];
    let mut heap_asc = BinaryHeap::new();
    let mut heap_desc = BinaryHeap::new();
    for &pp in pos.iter().skip(1).take(k) {
        heap_asc.push(Reverse(pp));
        heap_desc.push(pp);
    }

    let mut ans = DUMMY;
    let mut i = 1;
    // i + k <= n だと [n-k, n-k+1, ..., n-1, n] のパターンが確認できない (handmade_01)
    // ついでに n==k のときにはループを一度も通過しなくなる
    while i + k <= n + 1 {
        let Reverse(mut pos_min) = *heap_asc.peek().unwrap();
        while should_rm[pn[pos_min]] {
            heap_asc.pop();
            Reverse(pos_min) = *heap_asc.peek().unwrap();
        }

        let mut pos_max = *heap_desc.peek().unwrap();
        while should_rm[pn[pos_max]] {
            heap_desc.pop();
            pos_max = *heap_desc.peek().unwrap();
        }

        ans = ans.min(pos_max - pos_min);

        if i + k <= n {
            should_rm[i] = true;
            heap_asc.push(Reverse(pos[k + i]));
            heap_desc.push(pos[k + i]);
        }
        i += 1;
    }

    println!("{ans}");
}
